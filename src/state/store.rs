// state/store.rs — Core state definitions for Paint Studio
// โครงสร้างข้อมูลหลักสำหรับระบบจัดการภาพและ Floating Selection

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for IVec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for IVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for IVec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for IVec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct LayerId(pub usize);

/// TileBuffer เก็บข้อมูลพิกเซลเฉพาะส่วน (RGBA8)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<(u8, u8, u8, u8)>>,
}

impl TileBuffer {
    pub fn new(width: usize, height: usize, fill: (u8, u8, u8, u8)) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![fill; width]; height],
        }
    }

    #[allow(dead_code)]
    pub fn from_pixels(pixels: Vec<Vec<(u8, u8, u8, u8)>>) -> Self {
        let height = pixels.len();
        let width = if height > 0 { pixels[0].len() } else { 0 };
        Self {
            width,
            height,
            pixels,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> Option<(u8, u8, u8, u8)> {
        if y < self.height && x < self.width {
            Some(self.pixels[y][x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8)) {
        if y < self.height && x < self.width {
            self.pixels[y][x] = color;
        }
    }
}

/// SelectionMask หน้ากากสำหรับกำหนดขอบเขตพิกเซลที่เลือก
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionMask {
    pub width: usize,
    pub height: usize,
    pub mask: Vec<Vec<bool>>,
}

impl SelectionMask {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, initial: bool) -> Self {
        Self {
            width,
            height,
            mask: vec![vec![initial; width]; height],
        }
    }

    #[allow(dead_code)]
    pub fn is_selected(&self, x: usize, y: usize) -> bool {
        if y < self.height && x < self.width {
            self.mask[y][x]
        } else {
            false
        }
    }
}

/// SelectionRect ขอบเขตสี่เหลี่ยมการเลือก
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectionRect {
    pub min: IVec2,
    pub max: IVec2,
}

impl SelectionRect {
    pub fn new(min: IVec2, max: IVec2) -> Self {
        Self {
            min: IVec2::new(min.x.min(max.x), min.y.min(max.y)),
            max: IVec2::new(min.x.max(max.x), min.y.max(max.y)),
        }
    }

    pub fn width(&self) -> usize {
        (self.max.x - self.min.x).max(0) as usize
    }

    pub fn height(&self) -> usize {
        (self.max.y - self.min.y).max(0) as usize
    }
}

/// FloatingSelection เก็บข้อมูลเลเยอร์ลอยชั่วคราว
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloatingSelection {
    /// พิกเซลที่ลอยอยู่
    pub pixels: TileBuffer,
    /// ตำแหน่งพิกัดปัจจุบันบน Canvas
    pub origin: IVec2,
    /// เลเยอร์ต้นทางที่ตัดพิกเซลออกมา
    pub source_layer: LayerId,
    /// ตำแหน่งเริ่มต้นตอนที่ถูก Lift ออกมา (สำหรับ Cancel)
    pub original_origin: IVec2,
    /// Mask ที่เลือกไว้ (ถ้ามี)
    pub mask: Option<SelectionMask>,
}

impl FloatingSelection {
    pub fn new(
        pixels: TileBuffer,
        origin: IVec2,
        source_layer: LayerId,
        mask: Option<SelectionMask>,
    ) -> Self {
        Self {
            pixels,
            origin,
            source_layer,
            original_origin: origin,
            mask,
        }
    }
}

/// ข้อมูลเลเยอร์ในโปรเจกต์
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity_percent: u8,
    pub blend_mode: String,
    pub is_paper: bool,
    pub pixels: Vec<Vec<(u8, u8, u8, u8)>>,
}

impl Layer {
    pub fn new(id: LayerId, name: String, width: usize, height: usize, is_paper: bool) -> Self {
        let fill = if is_paper {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 0)
        };
        Self {
            id,
            name,
            visible: true,
            locked: is_paper,
            opacity_percent: 100,
            blend_mode: "ปกติ".to_string(),
            is_paper,
            pixels: vec![vec![fill; width]; height],
        }
    }

    pub fn opacity_f32(&self) -> f32 {
        self.opacity_percent as f32 / 100.0_f32
    }
}

/// Action สำหรับระบบ Floating Selection
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// ยกและตัดพิกเซลในขอบเขตที่เลือกขึ้นมาลอย
    LiftSelection,
    /// ขยับตำแหน่งที่ลอยอยู่
    MoveFloating(IVec2),
    /// ยืนยันการวาง รวมพิกเซลกลับลง active layer
    CommitFloating,
    /// ยกเลิก คืนพิกเซลกลับตำแหน่งเดิมบนเลเยอร์ต้นทาง
    CancelFloating,
    /// ลบเนื้อหาลอยทิ้ง ไม่รวมกลับ
    DeleteFloating,
}

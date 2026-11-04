// selection.rs — rectangular selection operations and clipboard buffer
use crate::state::store::{FloatingSelection, IVec2, SelectionRect, TileBuffer};

use super::App;

impl App {
    /// ขอบเขต Selection ที่ถูก clip ให้อยู่ในขอบเขตผ้าใบ
    pub(crate) fn selection_bounds(&self) -> Option<(usize, usize, usize, usize)> {
        let sel = self.selection.as_ref()?;
        Some((
            sel.min.x.max(0) as usize,
            sel.min.y.max(0) as usize,
            (sel.max.x.min(self.w as isize)).max(0) as usize,
            (sel.max.y.min(self.h as isize)).max(0) as usize,
        ))
    }

    pub(crate) fn clear_selection_or_layer(&mut self) {
        if self.layers[self.active].locked {
            return;
        }
        let clear_col = if self.layers[self.active].is_paper {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 0)
        };
        self.fill_active(clear_col);
        self.push_hist("ลบ");
    }

    pub(crate) fn fill_selection_or_layer(&mut self) {
        if self.layers[self.active].locked {
            return;
        }
        let col = (
            self.main_color.r(),
            self.main_color.g(),
            self.main_color.b(),
            (self.opacity * 255.0) as u8,
        );
        self.fill_active(col);
        self.push_hist("เติมสี");
    }

    fn fill_active(&mut self, col: (u8, u8, u8, u8)) {
        match self.selection_bounds() {
            Some((x0, y0, x1, y1)) => {
                for y in y0..y1 {
                    for x in x0..x1 {
                        self.layers[self.active].pixels[y][x] = col;
                    }
                }
            }
            None => {
                for y in 0..self.h {
                    for x in 0..self.w {
                        self.layers[self.active].pixels[y][x] = col;
                    }
                }
            }
        }
        self.texture_dirty = true;
    }

    pub(crate) fn copy_selection(&mut self) {
        let Some((x0, y0, x1, y1)) = self.selection_bounds() else {
            return;
        };
        let width = x1.saturating_sub(x0);
        let height = y1.saturating_sub(y0);
        if width == 0 || height == 0 {
            return;
        }
        let mut tb = TileBuffer::new(width, height, (0, 0, 0, 0));
        for dy in 0..height {
            for dx in 0..width {
                tb.set(dx, dy, self.layers[self.active].pixels[y0 + dy][x0 + dx]);
            }
        }
        self.clipboard_buffer = Some(tb);
        self.saved = "คัดลอกส่วนที่เลือกแล้ว (Ctrl+C)".into();
    }

    pub(crate) fn paste_clipboard(&mut self) {
        let Some(tb) = self.clipboard_buffer.clone() else {
            return;
        };
        let origin = self
            .selection
            .map(|sel| sel.min)
            .unwrap_or(IVec2::new(50, 50));
        let fl = FloatingSelection::new(tb, origin, self.layers[self.active].id, None);
        self.floating = Some(fl);
        self.texture_dirty = true;
        self.saved = "วางส่วนที่คัดลอกแล้ว (กด Enter เพื่อรวมลงเลเยอร์)".into();
    }

    pub(crate) fn inverse_selection(&mut self) {
        if let Some(sel) = self.selection {
            if sel.min == IVec2::ZERO && sel.max == IVec2::new(self.w as isize, self.h as isize) {
                self.selection = None;
            } else {
                self.selection = Some(SelectionRect::new(
                    IVec2::ZERO,
                    IVec2::new(self.w as isize, self.h as isize),
                ));
            }
            self.saved = "สลับพื้นที่เลือก (Inverse Selection)".into();
        }
    }

    /// เลือกทั้งผ้าใบ (Ctrl+A)
    pub(crate) fn select_all(&mut self) {
        self.selection = Some(SelectionRect::new(
            IVec2::ZERO,
            IVec2::new(self.w as isize, self.h as isize),
        ));
        self.saved = "เลือกทั้งหมด (Ctrl+A)".into();
    }

    /// ยกเลิกการเลือก (Ctrl+D)
    pub(crate) fn deselect(&mut self) {
        self.selection = None;
        self.saved = "ยกเลิกการเลือก (Ctrl+D)".into();
    }
}

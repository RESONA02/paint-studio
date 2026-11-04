// history/command.rs — Command history and FloatingCommand implementation
// คำสั่งสำหรับการย้อนกลับ (Undo/Redo) ของระบบ Floating Selection

use crate::state::store::{FloatingSelection, IVec2, Layer, LayerId, TileBuffer};

/// FloatingCommand รองรับการทำงานและการ Revert ของทุก Action ในการลอยเลเยอร์
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingCommand {
    /// ยกและตัดพิกเซลออกมา (Lift)
    Lift {
        source_layer: LayerId,
        floating: FloatingSelection,
        /// พิกเซลเดิมที่ถูกตัดออก (เพื่อใช้กู้คืนตอน Undo)
        cut_pixels: TileBuffer,
        origin: IVec2,
    },
    /// เลื่อนตำแหน่งที่ลอย (Move)
    Move { delta: IVec2 },
    /// รวมกลับลงเลเยอร์เป้าหมาย (Commit)
    Commit {
        target_layer: LayerId,
        floating: FloatingSelection,
        /// พิกเซลเดิมบนเป้าหมาย ณ จุดที่ถูกทับ (เพื่อใช้กู้คืนตอน Undo)
        overwritten_pixels: TileBuffer,
        commit_origin: IVec2,
    },
    /// ยกเลิกการลอย คืนพิกเซลกลับตำแหน่งเดิม (Cancel)
    Cancel {
        source_layer: LayerId,
        floating: FloatingSelection,
    },
    /// ลบส่วนที่ลอยอยู่ทิ้ง (Delete)
    Delete { floating: FloatingSelection },
}

impl FloatingCommand {
    /// สร้างคำสั่ง Lift: ตัดพิกเซลตามขอบเขต min..max จากเลเยอร์ต้นทางขึ้นมาลอย
    pub fn create_lift(
        source_layer_id: LayerId,
        layer: &mut Layer,
        canvas_w: usize,
        canvas_h: usize,
        min: IVec2,
        max: IVec2,
    ) -> Option<(Self, FloatingSelection)> {
        let min_x = min.x.clamp(0, canvas_w as isize) as usize;
        let min_y = min.y.clamp(0, canvas_h as isize) as usize;
        let max_x = max.x.clamp(0, canvas_w as isize) as usize;
        let max_y = max.y.clamp(0, canvas_h as isize) as usize;

        if max_x <= min_x || max_y <= min_y {
            return None;
        }

        let w = max_x - min_x;
        let h = max_y - min_y;

        let mut cut = TileBuffer::new(w, h, (0, 0, 0, 0));
        let clear_color = if layer.is_paper {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 0)
        };

        // ตัดพิกเซลจากเลเยอร์ต้นทางลง TileBuffer
        for dy in 0..h {
            for dx in 0..w {
                let px = layer.pixels[min_y + dy][min_x + dx];
                cut.set(dx, dy, px);
                // ล้างพิกเซลเดิมบนเลเยอร์
                layer.pixels[min_y + dy][min_x + dx] = clear_color;
            }
        }

        let origin = IVec2::new(min_x as isize, min_y as isize);
        let floating = FloatingSelection::new(cut.clone(), origin, source_layer_id, None);

        let cmd = FloatingCommand::Lift {
            source_layer: source_layer_id,
            floating: floating.clone(),
            cut_pixels: cut,
            origin,
        };

        Some((cmd, floating))
    }

    /// รันคำสั่งย้อนกลับ (Undo / Revert)
    pub fn revert(
        &self,
        layers: &mut [Layer],
        floating: &mut Option<FloatingSelection>,
        canvas_w: usize,
        canvas_h: usize,
    ) {
        match self {
            FloatingCommand::Lift {
                source_layer,
                cut_pixels,
                origin,
                ..
            } => {
                // คืนพิกเซลที่เคยตัดออกกลับลงเลเยอร์ต้นทาง และยกเลิกสถานะลอย
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *source_layer) {
                    for dy in 0..cut_pixels.height {
                        for dx in 0..cut_pixels.width {
                            let y = origin.y + dy as isize;
                            let x = origin.x + dx as isize;
                            if y >= 0
                                && (y as usize) < canvas_h
                                && x >= 0
                                && (x as usize) < canvas_w
                            {
                                layer.pixels[y as usize][x as usize] = cut_pixels.pixels[dy][dx];
                            }
                        }
                    }
                }
                *floating = None;
            }
            FloatingCommand::Move { delta } => {
                // เลื่อนพิกัดกลับทิศทางตรงกันข้าม
                if let Some(fl) = floating.as_mut() {
                    fl.origin -= *delta;
                }
            }
            FloatingCommand::Commit {
                target_layer,
                floating: old_floating,
                overwritten_pixels,
                commit_origin,
            } => {
                // กู้คืนพิกเซลเดิมบนเลเยอร์เป้าหมาย และนำ FloatingSelection กลับมาลอยใหม่
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *target_layer) {
                    for dy in 0..overwritten_pixels.height {
                        for dx in 0..overwritten_pixels.width {
                            let y = commit_origin.y + dy as isize;
                            let x = commit_origin.x + dx as isize;
                            if y >= 0
                                && (y as usize) < canvas_h
                                && x >= 0
                                && (x as usize) < canvas_w
                            {
                                layer.pixels[y as usize][x as usize] =
                                    overwritten_pixels.pixels[dy][dx];
                            }
                        }
                    }
                }
                *floating = Some(old_floating.clone());
            }
            FloatingCommand::Cancel {
                source_layer,
                floating: old_floating,
            } => {
                // ตัดพิกเซลออกจากเลเยอร์ต้นทางอีกครั้ง และคืนสถานะลอยกลับมา
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *source_layer) {
                    let clear_color = if layer.is_paper {
                        (255, 255, 255, 255)
                    } else {
                        (0, 0, 0, 0)
                    };
                    for dy in 0..old_floating.pixels.height {
                        for dx in 0..old_floating.pixels.width {
                            let y = old_floating.original_origin.y + dy as isize;
                            let x = old_floating.original_origin.x + dx as isize;
                            if y >= 0
                                && (y as usize) < canvas_h
                                && x >= 0
                                && (x as usize) < canvas_w
                            {
                                layer.pixels[y as usize][x as usize] = clear_color;
                            }
                        }
                    }
                }
                *floating = Some(old_floating.clone());
            }
            FloatingCommand::Delete {
                floating: old_floating,
            } => {
                // คืนสถานะของส่วนที่เคยถูกลบกลับมาลอยใหม่
                *floating = Some(old_floating.clone());
            }
        }
    }

    /// รันคำสั่งซ้ำ (Redo / Apply)
    pub fn apply(
        &self,
        layers: &mut [Layer],
        floating: &mut Option<FloatingSelection>,
        canvas_w: usize,
        canvas_h: usize,
    ) {
        match self {
            FloatingCommand::Lift {
                source_layer,
                floating: new_floating,
                origin,
                cut_pixels,
            } => {
                // ตัดพิกเซลบนเลเยอร์ต้นทางอีกครั้ง
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *source_layer) {
                    let clear_color = if layer.is_paper {
                        (255, 255, 255, 255)
                    } else {
                        (0, 0, 0, 0)
                    };
                    for dy in 0..cut_pixels.height {
                        for dx in 0..cut_pixels.width {
                            let y = origin.y + dy as isize;
                            let x = origin.x + dx as isize;
                            if y >= 0
                                && (y as usize) < canvas_h
                                && x >= 0
                                && (x as usize) < canvas_w
                            {
                                layer.pixels[y as usize][x as usize] = clear_color;
                            }
                        }
                    }
                }
                *floating = Some(new_floating.clone());
            }
            FloatingCommand::Move { delta } => {
                if let Some(fl) = floating.as_mut() {
                    fl.origin += *delta;
                }
            }
            FloatingCommand::Commit {
                target_layer,
                floating: fl,
                commit_origin,
                ..
            } => {
                // รวมพิกเซลลงเลเยอร์เป้าหมาย
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *target_layer) {
                    for dy in 0..fl.pixels.height {
                        for dx in 0..fl.pixels.width {
                            let src_px = fl.pixels.pixels[dy][dx];
                            if src_px.3 > 0 {
                                let y = commit_origin.y + dy as isize;
                                let x = commit_origin.x + dx as isize;
                                if y >= 0
                                    && (y as usize) < canvas_h
                                    && x >= 0
                                    && (x as usize) < canvas_w
                                {
                                    layer.pixels[y as usize][x as usize] = src_px;
                                }
                            }
                        }
                    }
                }
                *floating = None;
            }
            FloatingCommand::Cancel {
                source_layer,
                floating: fl,
            } => {
                // คืนพิกเซลกลับลงตำแหน่งเริ่มต้น
                if let Some(layer) = layers.iter_mut().find(|l| l.id == *source_layer) {
                    for dy in 0..fl.pixels.height {
                        for dx in 0..fl.pixels.width {
                            let y = fl.original_origin.y + dy as isize;
                            let x = fl.original_origin.x + dx as isize;
                            if y >= 0
                                && (y as usize) < canvas_h
                                && x >= 0
                                && (x as usize) < canvas_w
                            {
                                layer.pixels[y as usize][x as usize] = fl.pixels.pixels[dy][dx];
                            }
                        }
                    }
                }
                *floating = None;
            }
            FloatingCommand::Delete { .. } => {
                *floating = None;
            }
        }
    }
}

// ------------------------------------------------------------------------
// Unit Tests ตามข้อกำหนดคุณภาพ
// ------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_layer() -> Layer {
        let mut layer = Layer::new(LayerId(0), "Test".into(), 100, 100, false);
        // วาดสีดำ (20, 20, 20, 255) ในสี่เหลี่ยม (10, 10) ถึง (20, 20)
        for y in 10..=20 {
            for x in 10..=20 {
                layer.pixels[y][x] = (20, 20, 20, 255);
            }
        }
        layer
    }

    #[test]
    fn test_lift_then_undo() {
        let layer = create_test_layer();
        let mut layers = vec![layer];

        // 1. Lift ส่วนที่เลือก (10, 10) ถึง (21, 21)
        let (cmd, fl) = FloatingCommand::create_lift(
            LayerId(0),
            &mut layers[0],
            100,
            100,
            IVec2::new(10, 10),
            IVec2::new(21, 21),
        )
        .expect("Lift must succeed");
        let mut floating = Some(fl);

        // ตรวจสอบว่าพิกเซลถูกตัดออกจากเลเยอร์ต้นทางแล้ว
        assert_eq!(layers[0].pixels[15][15], (0, 0, 0, 0));
        assert!(floating.is_some());
        assert_eq!(
            floating.as_ref().unwrap().pixels.pixels[5][5],
            (20, 20, 20, 255)
        );

        // 2. Undo / Revert คำสั่ง Lift
        cmd.revert(&mut layers, &mut floating, 100, 100);

        // พิกเซลต้องกลับคืนมาที่เลเยอร์ต้นทางอย่างสมบูรณ์ และ floating ต้องเป็น None
        assert_eq!(layers[0].pixels[15][15], (20, 20, 20, 255));
        assert!(floating.is_none());
    }

    #[test]
    fn test_commit_then_undo() {
        let layer = create_test_layer();
        let mut layers = vec![layer];

        let (lift_cmd, fl) = FloatingCommand::create_lift(
            LayerId(0),
            &mut layers[0],
            100,
            100,
            IVec2::new(10, 10),
            IVec2::new(21, 21),
        )
        .unwrap();
        let mut floating = Some(fl);

        // เลื่อนตำแหน่งไปยัง (50, 50)
        let delta = IVec2::new(40, 40);
        let move_cmd = FloatingCommand::Move { delta };
        move_cmd.apply(&mut layers, &mut floating, 100, 100);
        assert_eq!(floating.as_ref().unwrap().origin, IVec2::new(50, 50));

        // ทำการ Commit
        let fl_clone = floating.as_ref().unwrap().clone();
        let commit_origin = fl_clone.origin;
        let mut overwritten =
            TileBuffer::new(fl_clone.pixels.width, fl_clone.pixels.height, (0, 0, 0, 0));
        for dy in 0..fl_clone.pixels.height {
            for dx in 0..fl_clone.pixels.width {
                overwritten.set(
                    dx,
                    dy,
                    layers[0].pixels[commit_origin.y as usize + dy][commit_origin.x as usize + dx],
                );
            }
        }

        let commit_cmd = FloatingCommand::Commit {
            target_layer: LayerId(0),
            floating: fl_clone,
            overwritten_pixels: overwritten,
            commit_origin,
        };
        commit_cmd.apply(&mut layers, &mut floating, 100, 100);

        // ตรวจสอบว่าพิกเซลอยู่ที่ (55, 55) บนเลเยอร์แล้ว และ floating เป็น None
        assert_eq!(layers[0].pixels[55][55], (20, 20, 20, 255));
        assert!(floating.is_none());

        // Undo Commit
        commit_cmd.revert(&mut layers, &mut floating, 100, 100);

        // พิกเซลที่ (55, 55) ต้องถูกกู้คืนกลับเป็นว่างเปล่า และ floating ฟื้นกลับมา
        assert_eq!(layers[0].pixels[55][55], (0, 0, 0, 0));
        assert!(floating.is_some());
        assert_eq!(floating.as_ref().unwrap().origin, IVec2::new(50, 50));

        // Revert Lift ให้สะอาด
        lift_cmd.revert(&mut layers, &mut floating, 100, 100);
    }

    #[test]
    fn test_cancel_restores_original_position() {
        let layer = create_test_layer();
        let mut layers = vec![layer];

        let (_lift_cmd, fl) = FloatingCommand::create_lift(
            LayerId(0),
            &mut layers[0],
            100,
            100,
            IVec2::new(10, 10),
            IVec2::new(21, 21),
        )
        .unwrap();
        let mut floating = Some(fl);

        // เลื่อนตำแหน่งไปยัง (70, 70)
        let move_cmd = FloatingCommand::Move {
            delta: IVec2::new(60, 60),
        };
        move_cmd.apply(&mut layers, &mut floating, 100, 100);

        // สั่ง Cancel
        let fl_now = floating.as_ref().unwrap().clone();
        let cancel_cmd = FloatingCommand::Cancel {
            source_layer: LayerId(0),
            floating: fl_now,
        };
        cancel_cmd.apply(&mut layers, &mut floating, 100, 100);

        // พิกเซลต้องกลับคืนมาที่ตำแหน่งดั้งเดิม (15, 15) ไม่ใช่ที่ (75, 75)
        assert_eq!(layers[0].pixels[15][15], (20, 20, 20, 255));
        assert_eq!(layers[0].pixels[75][75], (0, 0, 0, 0));
        assert!(floating.is_none());
    }

    #[test]
    fn test_delete_leaves_no_pixels() {
        let layer = create_test_layer();
        let mut layers = vec![layer];

        let (_lift_cmd, fl) = FloatingCommand::create_lift(
            LayerId(0),
            &mut layers[0],
            100,
            100,
            IVec2::new(10, 10),
            IVec2::new(21, 21),
        )
        .unwrap();
        let mut floating = Some(fl);

        // เลื่อนไปยัง (40, 40) แล้วลบ (Delete)
        let move_cmd = FloatingCommand::Move {
            delta: IVec2::new(30, 30),
        };
        move_cmd.apply(&mut layers, &mut floating, 100, 100);

        let fl_now = floating.as_ref().unwrap().clone();
        let delete_cmd = FloatingCommand::Delete { floating: fl_now };
        delete_cmd.apply(&mut layers, &mut floating, 100, 100);

        // ต้องไม่มีพิกเซลเหลืออยู่ที่ทั้งตำแหน่งเดิม (15, 15) และตำแหน่งลอย (45, 45)
        assert_eq!(layers[0].pixels[15][15], (0, 0, 0, 0));
        assert_eq!(layers[0].pixels[45][45], (0, 0, 0, 0));
        assert!(floating.is_none());

        // Undo Delete
        delete_cmd.revert(&mut layers, &mut floating, 100, 100);
        assert!(floating.is_some());
        assert_eq!(floating.as_ref().unwrap().origin, IVec2::new(40, 40));
    }
}

// layers.rs — layer lifecycle: create, duplicate, delete and merge
use crate::state::store::{Layer, LayerId};
use eframe::egui;

use super::App;

impl App {
    pub(crate) fn new_canvas(&mut self) {
        self.layers.clear();
        self.layers.push(Layer::new(
            LayerId(0),
            "กระดาษ".to_string(),
            self.w,
            self.h,
            true,
        ));
        self.layers.push(Layer::new(
            LayerId(1),
            "เลเยอร์ 1".to_string(),
            self.w,
            self.h,
            false,
        ));
        self.active = 1;
        self.selection = None;
        self.floating = None;
        self.commands.clear();
        self.cmd_index = 0;
        self.history_labels.clear();
        self.hi = 0;
        self.zoom = 1.0;
        self.pan_offset = egui::Vec2::ZERO;
        self.rotation = 0.0;
        self.current_frame = 0;
        self.animation_frames = vec![super::AnimationFrame {
            layers: self.layers.clone(),
        }];
        self.camera_keys.clear();
        self.playback = false;
        self.texture_dirty = true;
    }

    pub(crate) fn add_raster_layer(&mut self) {
        let next_id = next_layer_id(&self.layers);
        let k = self.layers.len();
        self.layers.push(Layer::new(
            LayerId(next_id),
            format!("เลเยอร์ {k}"),
            self.w,
            self.h,
            false,
        ));
        self.active = self.layers.len() - 1;
        self.texture_dirty = true;
        self.push_hist("เพิ่มเลเยอร์");
    }

    pub(crate) fn duplicate_active_layer(&mut self) {
        if let Some(src) = self.layers.get(self.active) {
            let next_id = next_layer_id(&self.layers);
            let mut cloned = src.clone();
            cloned.id = LayerId(next_id);
            cloned.name = format!("{} (สำเนา)", src.name);
            cloned.is_paper = false;
            self.layers.insert(self.active + 1, cloned);
            self.active += 1;
            self.texture_dirty = true;
            self.push_hist("ทำสำเนาเลเยอร์");
        }
    }

    pub(crate) fn delete_active_layer(&mut self) {
        if self.layers.len() > 1 && !self.layers[self.active].is_paper {
            self.layers.remove(self.active);
            if self.active >= self.layers.len() {
                self.active = self.layers.len() - 1;
            }
            self.texture_dirty = true;
            self.push_hist("ลบเลเยอร์");
        }
    }

    /// ล้างพิกเซลทั้งเลเยอร์ที่เลือก (ไม่สนใจ Selection)
    pub(crate) fn clear_active_layer(&mut self) {
        let li = self.active;
        if self.layers[li].locked {
            return;
        }
        let clear_col = if self.layers[li].is_paper {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 0)
        };
        for y in 0..self.h {
            for x in 0..self.w {
                self.layers[li].pixels[y][x] = clear_col;
            }
        }
        self.texture_dirty = true;
        self.push_hist("ล้างเลเยอร์");
    }

    pub(crate) fn merge_layer_down(&mut self) {
        if self.active == 0 || self.layers[self.active].is_paper {
            return;
        }
        let top = self.layers.remove(self.active);
        self.active -= 1;
        let bottom = &mut self.layers[self.active];
        for y in 0..self.h {
            for x in 0..self.w {
                bottom.pixels[y][x] = blend_down(top.pixels[y][x], bottom.pixels[y][x]);
            }
        }
        self.texture_dirty = true;
        self.push_hist("รวมเลเยอร์");
    }

    pub(crate) fn merge_visible_layers(&mut self) {
        if self.layers.len() <= 1 {
            return;
        }
        let merged = self.compose_visible(LayerId(999), "รวมเลเยอร์");
        self.layers.retain(|l| !l.visible || l.is_paper);
        self.layers.push(merged);
        self.active = self.layers.len() - 1;
        self.texture_dirty = true;
        self.push_hist("รวมเลเยอร์ที่มองเห็น");
    }

    pub(crate) fn merge_all_layers(&mut self) {
        self.merge_visible_layers();
    }

    pub(crate) fn merge_visible_to_new_layer(&mut self) {
        let id = LayerId(next_layer_id(&self.layers));
        let merged = self.compose_visible(id, "รวมเลเยอร์ใหม่");
        self.layers.push(merged);
        self.active = self.layers.len() - 1;
        self.texture_dirty = true;
        self.push_hist("รวมเลเยอร์ที่มองเห็นเป็นเลเยอร์ใหม่");
    }

    fn compose_visible(&self, id: LayerId, name: &str) -> Layer {
        let mut merged = Layer::new(id, name.to_string(), self.w, self.h, false);
        for l in &self.layers {
            if !l.visible {
                continue;
            }
            let op = l.opacity_percent as f32 / 100.0_f32;
            for y in 0..self.h {
                for x in 0..self.w {
                    let p = l.pixels[y][x];
                    if p.3 == 0 {
                        continue;
                    }
                    merged.pixels[y][x] = blend_source_over(merged.pixels[y][x], p, op);
                }
            }
        }
        merged
    }
}

fn next_layer_id(layers: &[Layer]) -> usize {
    layers.iter().map(|l| l.id.0).max().unwrap_or(0) + 1
}

fn blend_down(top: (u8, u8, u8, u8), bottom: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
    if top.3 == 0 {
        return bottom;
    }
    let a = top.3 as f32 / 255.0;
    let inv_a = 1.0 - a;
    (
        (top.0 as f32 * a + bottom.0 as f32 * inv_a) as u8,
        (top.1 as f32 * a + bottom.1 as f32 * inv_a) as u8,
        (top.2 as f32 * a + bottom.2 as f32 * inv_a) as u8,
        ((top.3 as f32 + bottom.3 as f32 * inv_a).min(255.0)) as u8,
    )
}

fn blend_source_over(dst: (u8, u8, u8, u8), src: (u8, u8, u8, u8), op: f32) -> (u8, u8, u8, u8) {
    let a = (src.3 as f32 / 255.0_f32) * op;
    let da = dst.3 as f32 / 255.0_f32;
    let out_a = a + da * (1.0_f32 - a);
    if out_a <= 0.0_f32 {
        return dst;
    }
    let r = (src.0 as f32 * a + dst.0 as f32 * da * (1.0_f32 - a)) / out_a;
    let g = (src.1 as f32 * a + dst.1 as f32 * da * (1.0_f32 - a)) / out_a;
    let b = (src.2 as f32 * a + dst.2 as f32 * da * (1.0_f32 - a)) / out_a;
    (
        r.round().clamp(0.0_f32, 255.0_f32) as u8,
        g.round().clamp(0.0_f32, 255.0_f32) as u8,
        b.round().clamp(0.0_f32, 255.0_f32) as u8,
        (out_a * 255.0_f32).round().clamp(0.0_f32, 255.0_f32) as u8,
    )
}

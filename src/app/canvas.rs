// canvas.rs — pixel painting, layer compositing and PNG export
use crate::state::store::Layer;
use crate::tools::Tool;
use eframe::egui;

use super::App;

impl App {
    pub(crate) fn stroke(&mut self, a: (isize, isize), b: (isize, isize)) {
        if self.page_lock.is_none() {
            return;
        }
        // หากมีเลเยอร์ลอยอยู่ ห้ามวาดลงบนเลเยอร์ปกติ
        if self.floating.is_some() {
            return;
        }

        let li = self.active;
        if self.layers[li].locked || !self.layers[li].visible {
            return;
        }

        let steps = ((b.0 - a.0).abs().max((b.1 - a.1).abs())).max(1);
        let r = ((self.size as isize).max(1)) / 2;
        let col = self.stroke_color(li);

        for s in 0..=steps {
            let cx = a.0 + (b.0 - a.0) * s / steps;
            let cy = a.1 + (b.1 - a.1) * s / steps;
            self.stamp_disc(li, cx, cy, r, col);
            if self.symmetry_enabled {
                let sym_cx = 2 * self.symmetry_axis_x - cx;
                self.stamp_disc(li, sym_cx, cy, r, col);
            }
        }
        self.texture_dirty = true;
    }

    fn stroke_color(&self, li: usize) -> (u8, u8, u8, u8) {
        if self.is_transparent || self.tool == Tool::Eraser {
            if self.layers[li].is_paper {
                (255, 255, 255, 255)
            } else {
                (0, 0, 0, 0)
            }
        } else {
            let c = self.main_color;
            let a = (255.0_f32 * self.opacity) as u8;
            match self.tool {
                Tool::Pencil => (c.r(), c.g(), c.b(), (a as f32 * 0.75_f32) as u8),
                _ => (c.r(), c.g(), c.b(), a),
            }
        }
    }

    fn stamp_disc(&mut self, li: usize, cx: isize, cy: isize, r: isize, col: (u8, u8, u8, u8)) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    self.put_pixel(li, cx + dx, cy + dy, col);
                }
            }
        }
    }

    fn put_pixel(&mut self, li: usize, x: isize, y: isize, col: (u8, u8, u8, u8)) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.layers[li].pixels[y as usize][x as usize] = col;
        }
    }

    /// ผสมสีและคอมโพสิตเลเยอร์ทั้งหมด (รวม Floating Layer) เป็นภาพเดียวส่งไปยัง GPU
    pub(crate) fn update_texture(&mut self, ctx: &egui::Context) {
        if !self.texture_dirty && self.texture.is_some() {
            return;
        }

        let raw = self.compose_rgba();
        let color_image = egui::ColorImage::from_rgba_unmultiplied([self.w, self.h], &raw);

        match &mut self.texture {
            Some(tex) => tex.set(color_image, egui::TextureOptions::LINEAR),
            None => {
                self.texture = Some(ctx.load_texture(
                    "canvas_texture",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
        self.texture_dirty = false;
    }

    fn compose_rgba(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(self.w * self.h * 4);
        for y in 0..self.h {
            for x in 0..self.w {
                let mut px = [255u8, 255, 255, 255];
                self.composite_at(&mut px, x, y);
                if self.onion_enabled {
                    if self.current_frame > 0 {
                        blend_onion(
                            &mut px,
                            &self.animation_frames[self.current_frame - 1].layers,
                            x,
                            y,
                            (70, 145, 255),
                            self.onion_previous_alpha,
                        );
                    }
                    if self.current_frame + 1 < self.animation_frames.len() {
                        blend_onion(
                            &mut px,
                            &self.animation_frames[self.current_frame + 1].layers,
                            x,
                            y,
                            (55, 210, 120),
                            self.onion_next_alpha,
                        );
                    }
                }
                raw.extend_from_slice(&px);
            }
        }
        raw
    }

    fn composite_at(&self, px: &mut [u8; 4], x: usize, y: usize) {
        // วนลูปผสมสีแต่ละเลเยอร์จากล่างขึ้นบน
        for (idx, l) in self.layers.iter().enumerate() {
            if !l.visible {
                continue;
            }
            let (r, g, b, a) = l.pixels[y][x];
            blend_layer(
                px,
                (r, g, b),
                (a as f32 / 255.0_f32) * l.opacity_f32(),
                &l.blend_mode,
            );

            // Floating Selection ต้องแทรกผสมระหว่าง active layer กับเลเยอร์ที่อยู่เหนือมัน
            if idx == self.active {
                self.composite_floating_at(px, x, y);
            }
        }
    }

    fn composite_floating_at(&self, px: &mut [u8; 4], x: usize, y: usize) {
        let Some(fl) = &self.floating else {
            return;
        };
        let fx = x as isize - fl.origin.x;
        let fy = y as isize - fl.origin.y;
        if fx < 0 || fy < 0 || fx as usize >= fl.pixels.width || fy as usize >= fl.pixels.height {
            return;
        }
        let (fr, fg, fb, fa) = fl.pixels.pixels[fy as usize][fx as usize];
        if fa > 0 {
            blend_over(px, (fr, fg, fb), fa as f32 / 255.0_f32);
        }
    }

    pub(crate) fn save_png(&self) -> std::result::Result<(), String> {
        if self.page_lock.is_none() {
            return Err("Cannot save: this page is locked by another session".into());
        }
        let mut img = image::RgbaImage::new(self.w as u32, self.h as u32);
        for y in 0..self.h {
            for x in 0..self.w {
                img.put_pixel(x as u32, y as u32, image::Rgba(self.flatten_pixel(x, y)));
            }
        }
        let out_path = std::path::Path::new("shots/art.png");
        ensure_parent_dir(out_path);
        img.save(out_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn flatten_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let mut px = [255u8, 255, 255, 255];
        for l in &self.layers {
            if !l.visible {
                continue;
            }
            let (r, g, b, a) = l.pixels[y][x];
            blend_layer(
                &mut px,
                (r, g, b),
                (a as f32 / 255.0_f32) * l.opacity_f32(),
                &l.blend_mode,
            );
        }
        px
    }
}

/// รวมทุกเลเยอร์ที่มองเห็นเป็น RGBA แบบแบน (ใช้สำหรับหน้าคู่, LT และ thumbnail)
pub(crate) fn composite_layers(layers: &[Layer], w: usize, h: usize) -> Vec<[u8; 4]> {
    let mut out = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            let mut px = [255u8, 255, 255, 255];
            for l in layers.iter().filter(|l| l.visible) {
                let (r, g, b, a) = l.pixels[y][x];
                let alpha = (a as f32 / 255.0_f32) * l.opacity_f32();
                blend_over(&mut px, (r, g, b), alpha);
            }
            out.push(px);
        }
    }
    out
}

fn blend_over(px: &mut [u8; 4], rgb: (u8, u8, u8), alpha: f32) {
    px[0] = (rgb.0 as f32 * alpha + px[0] as f32 * (1.0_f32 - alpha)) as u8;
    px[1] = (rgb.1 as f32 * alpha + px[1] as f32 * (1.0_f32 - alpha)) as u8;
    px[2] = (rgb.2 as f32 * alpha + px[2] as f32 * (1.0_f32 - alpha)) as u8;
}

fn blend_layer(px: &mut [u8; 4], top: (u8, u8, u8), alpha: f32, mode: &str) {
    if alpha <= 0.0 {
        return;
    }
    let target = match mode {
        "คูณ" => (
            ((px[0] as u16 * top.0 as u16) / 255) as u8,
            ((px[1] as u16 * top.1 as u16) / 255) as u8,
            ((px[2] as u16 * top.2 as u16) / 255) as u8,
        ),
        "สกรีน" => (
            (255 - ((255 - px[0] as u16) * (255 - top.0 as u16)) / 255) as u8,
            (255 - ((255 - px[1] as u16) * (255 - top.1 as u16)) / 255) as u8,
            (255 - ((255 - px[2] as u16) * (255 - top.2 as u16)) / 255) as u8,
        ),
        "ซ้อนทับ" => (
            overlay_channel(px[0], top.0),
            overlay_channel(px[1], top.1),
            overlay_channel(px[2], top.2),
        ),
        "บวก" => (
            (px[0] as u16 + top.0 as u16).min(255) as u8,
            (px[1] as u16 + top.1 as u16).min(255) as u8,
            (px[2] as u16 + top.2 as u16).min(255) as u8,
        ),
        _ => top,
    };

    px[0] = (target.0 as f32 * alpha + px[0] as f32 * (1.0_f32 - alpha)).round() as u8;
    px[1] = (target.1 as f32 * alpha + px[1] as f32 * (1.0_f32 - alpha)).round() as u8;
    px[2] = (target.2 as f32 * alpha + px[2] as f32 * (1.0_f32 - alpha)).round() as u8;
}

fn overlay_channel(bot: u8, top: u8) -> u8 {
    if bot < 128 {
        ((2 * bot as u16 * top as u16) / 255) as u8
    } else {
        (255 - (2 * (255 - bot as u16) * (255 - top as u16)) / 255) as u8
    }
}

fn blend_onion(
    px: &mut [u8; 4],
    layers: &[crate::state::store::Layer],
    x: usize,
    y: usize,
    tint: (u8, u8, u8),
    opacity: f32,
) {
    let alpha = layers
        .iter()
        .filter(|l| l.visible && !l.is_paper)
        .map(|l| (l.pixels[y][x].3 as f32 / 255.0) * l.opacity_f32())
        .fold(0.0_f32, |acc, a| 1.0 - (1.0 - acc) * (1.0 - a))
        * opacity;
    if alpha <= 0.0 {
        return;
    }
    px[0] = (tint.0 as f32 * alpha + px[0] as f32 * (1.0 - alpha)) as u8;
    px[1] = (tint.1 as f32 * alpha + px[1] as f32 * (1.0 - alpha)) as u8;
    px[2] = (tint.2 as f32 * alpha + px[2] as f32 * (1.0 - alpha)) as u8;
}

pub(crate) fn ensure_parent_dir(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
}

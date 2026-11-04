// lt.rs — 2D LT conversion: line art extraction and screentone generation
use crate::state::store::{Layer, LayerId};

use super::canvas::composite_layers;
use super::App;

/// 4x4 Bayer matrix สำหรับสร้างเม็ดสกรีนโทน
const BAYER4: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

impl App {
    /// ดึงเส้นขอบ (Sobel) จากภาพรวมของหน้า → เลเยอร์ Line Art ใหม่
    #[allow(clippy::needless_range_loop)]
    pub(crate) fn apply_line_art(&mut self) {
        let (w, h) = (self.w, self.h);
        let src = composite_layers(&self.layers, w, h);
        let gray = grayscale(&src, w, h);
        let limit = self.lt_threshold as f32 * 3.0;
        let mut pixels = vec![vec![(0u8, 0, 0, 0); w]; h];
        for y in 0..h {
            for x in 0..w {
                if sobel(&gray, w, h, x, y) > limit {
                    pixels[y][x] = (20, 20, 24, 255);
                }
            }
        }
        self.add_result_layer("Line Art (2D LT)", pixels);
        self.saved = "ดึงเส้น Line Art เป็นเลเยอร์ใหม่แล้ว".into();
    }

    /// แยกแสงเงาเป็นเม็ดสกรีนโทน 2-4 ระดับ
    pub(crate) fn apply_screentone(&mut self) {
        let (w, h) = (self.w, self.h);
        let src = composite_layers(&self.layers, w, h);
        let pixels = screentone_pixels(&src, w, h, self.lt_levels, self.lt_dot);
        self.add_result_layer("Screentone (2D LT)", pixels);
        self.saved = "สร้างสกรีนโทนเป็นเลเยอร์ใหม่แล้ว".into();
    }

    fn add_result_layer(&mut self, name: &str, pixels: Vec<Vec<(u8, u8, u8, u8)>>) {
        let next_id = self.layers.iter().map(|l| l.id.0).max().unwrap_or(0) + 1;
        let mut layer = Layer::new(LayerId(next_id), name.to_string(), self.w, self.h, false);
        layer.pixels = pixels;
        self.layers.push(layer);
        self.active = self.layers.len() - 1;
        self.texture_dirty = true;
        self.push_hist(name);
    }
}

fn screentone_pixels(
    src: &[[u8; 4]],
    w: usize,
    h: usize,
    levels: u8,
    dot: u8,
) -> Vec<Vec<(u8, u8, u8, u8)>> {
    let steps = levels.max(2) as f32;
    let cell = dot.max(1) as usize;
    let mut pixels = vec![vec![(0u8, 0, 0, 0); w]; h];
    for y in 0..h {
        for x in 0..w {
            let p = src[y * w + x];
            let darkness = 1.0 - luminance(p[0], p[1], p[2]);
            let tone = (darkness * steps).floor() / steps;
            let threshold = BAYER4[(y / cell) % 4][(x / cell) % 4] as f32 / 16.0;
            if tone > 0.0 && threshold < tone {
                pixels[y][x] = (15, 15, 18, 255);
            }
        }
    }
    pixels
}

fn grayscale(src: &[[u8; 4]], w: usize, h: usize) -> Vec<f32> {
    (0..w * h)
        .map(|i| {
            let p = src[i];
            luminance(p[0], p[1], p[2]) * 255.0
        })
        .collect()
}

fn luminance(r: u8, g: u8, b: u8) -> f32 {
    (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0
}

fn sobel(gray: &[f32], w: usize, h: usize, x: usize, y: usize) -> f32 {
    let gx = -sample(gray, w, h, x as isize - 1, y as isize - 1)
        + sample(gray, w, h, x as isize + 1, y as isize - 1)
        - 2.0 * sample(gray, w, h, x as isize - 1, y as isize)
        + 2.0 * sample(gray, w, h, x as isize + 1, y as isize)
        - sample(gray, w, h, x as isize - 1, y as isize + 1)
        + sample(gray, w, h, x as isize + 1, y as isize + 1);
    let gy = -sample(gray, w, h, x as isize - 1, y as isize - 1)
        - 2.0 * sample(gray, w, h, x as isize, y as isize - 1)
        - sample(gray, w, h, x as isize + 1, y as isize - 1)
        + sample(gray, w, h, x as isize - 1, y as isize + 1)
        + 2.0 * sample(gray, w, h, x as isize, y as isize + 1)
        + sample(gray, w, h, x as isize + 1, y as isize + 1);
    (gx * gx + gy * gy).sqrt()
}

fn sample(gray: &[f32], w: usize, h: usize, x: isize, y: isize) -> f32 {
    let x = x.clamp(0, w as isize - 1) as usize;
    let y = y.clamp(0, h as isize - 1) as usize;
    gray[y * w + x]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::pages::build_page_layers;

    fn tiny_app() -> App {
        let mut app = App::new(None);
        app.w = 32;
        app.h = 32;
        app.layers = build_page_layers(32, 32);
        for y in 8..24 {
            for x in 8..24 {
                app.layers[1].pixels[y][x] = (10, 10, 10, 255);
            }
        }
        app.active = 1;
        app
    }

    #[test]
    fn line_art_adds_ink_layer() {
        let mut app = tiny_app();
        app.apply_line_art();
        assert_eq!(app.layers.len(), 3);
        assert!(app.layers[2]
            .pixels
            .iter()
            .any(|r| r.iter().any(|p| p.3 > 0)));
    }

    #[test]
    fn screentone_adds_dot_layer() {
        let mut app = tiny_app();
        app.apply_screentone();
        assert_eq!(app.layers.len(), 3);
        assert!(app.layers[2]
            .pixels
            .iter()
            .any(|r| r.iter().any(|p| p.3 > 0)));
    }
}

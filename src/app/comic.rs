// comic.rs — vector comic frame borders, speech balloons and rasterization
use crate::state::store::{IVec2, Layer};
use eframe::egui;

use super::App;

const INK: (u8, u8, u8, u8) = (20, 20, 24, 255);
const PAPER: (u8, u8, u8, u8) = (255, 255, 255, 255);

#[derive(Clone, Debug)]
pub(crate) struct ComicFrame {
    pub(crate) min: IVec2,
    pub(crate) max: IVec2,
}

impl ComicFrame {
    pub(crate) fn new(a: IVec2, b: IVec2) -> Self {
        Self {
            min: IVec2::new(a.x.min(b.x), a.y.min(b.y)),
            max: IVec2::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }

    pub(crate) fn width(&self) -> isize {
        self.max.x - self.min.x
    }

    pub(crate) fn height(&self) -> isize {
        self.max.y - self.min.y
    }

    pub(crate) fn shifted(&self, dx: isize, dy: isize) -> Self {
        Self {
            min: IVec2::new(self.min.x + dx, self.min.y + dy),
            max: IVec2::new(self.max.x + dx, self.max.y + dy),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Balloon {
    pub(crate) center: IVec2,
    pub(crate) rx: isize,
    pub(crate) ry: isize,
    pub(crate) tail: IVec2,
    pub(crate) text: String,
}

impl Balloon {
    pub(crate) fn shifted(&self, dx: isize, dy: isize) -> Self {
        let mut out = self.clone();
        out.center = IVec2::new(self.center.x + dx, self.center.y + dy);
        out.tail = IVec2::new(self.tail.x + dx, self.tail.y + dy);
        out
    }

    pub(crate) fn contains(&self, p: IVec2) -> bool {
        let nx = (p.x - self.center.x) as f32 / self.rx.max(1) as f32;
        let ny = (p.y - self.center.y) as f32 / self.ry.max(1) as f32;
        nx * nx + ny * ny <= 1.0
    }
}

impl App {
    pub(crate) fn begin_frame_drag(&mut self, p: IVec2) {
        self.frame_start = Some(p);
        self.frame_current = Some(p);
    }

    pub(crate) fn update_frame_drag(&mut self, p: IVec2) {
        if self.frame_start.is_some() {
            self.frame_current = Some(p);
        }
    }

    pub(crate) fn commit_frame_drag(&mut self) {
        let (Some(a), Some(b)) = (self.frame_start.take(), self.frame_current.take()) else {
            return;
        };
        let frame = ComicFrame::new(a, b);
        if frame.width() > 4 && frame.height() > 4 {
            self.comic_frames.push(frame);
            self.push_hist("ตัดกรอบช่องการ์ตูน");
        }
    }

    pub(crate) fn begin_balloon_drag(&mut self, p: IVec2) {
        self.balloon_start = Some(p);
    }

    pub(crate) fn commit_balloon_drag(&mut self, end: IVec2) {
        let Some(start) = self.balloon_start.take() else {
            return;
        };
        let rx = (end.x - start.x).abs().max(40);
        let ry = (end.y - start.y).abs().max(32);
        let balloon = Balloon {
            center: start,
            rx,
            ry,
            tail: IVec2::new(start.x - rx / 2, start.y + ry + ry / 2),
            text: "…".to_string(),
        };
        self.balloons.push(balloon);
        self.selected_balloon = Some(self.balloons.len() - 1);
        self.push_hist("เพิ่มบอลลูนคำพูด");
    }

    pub(crate) fn balloon_at(&self, p: IVec2) -> Option<usize> {
        self.balloons.iter().position(|b| b.contains(p))
    }

    pub(crate) fn delete_balloon(&mut self, index: usize) {
        if index < self.balloons.len() {
            self.balloons.remove(index);
            self.selected_balloon = None;
        }
    }

    /// แปลงกรอบและบอลลูนเวกเตอร์เป็นพิกเซลบนเลเยอร์ที่กำลังแก้ไข
    pub(crate) fn rasterize_comic_items(&mut self) {
        let index = self.active;
        if self.layers[index].locked {
            self.saved = "เลเยอร์ถูกล็อค".into();
            return;
        }
        let frames = self.comic_frames.clone();
        let balloons = self.balloons.clone();
        let (w, h) = (self.w, self.h);
        for frame in &frames {
            stroke_frame(&mut self.layers[index], frame, 3, w, h);
        }
        for balloon in &balloons {
            draw_balloon(&mut self.layers[index], balloon, w, h);
        }
        if frames.is_empty() && balloons.is_empty() {
            self.saved = "ยังไม่มีกรอบหรือบอลลูนให้แปลง".into();
            return;
        }
        self.texture_dirty = true;
        self.push_hist("แปลงกรอบ/บอลลูนเป็นภาพ");
        self.saved = "แปลงกรอบ/บอลลูนเป็นภาพแล้ว".into();
    }
}

fn stroke_frame(layer: &mut Layer, frame: &ComicFrame, thickness: isize, w: usize, h: usize) {
    for t in 0..thickness {
        let (x0, x1) = (frame.min.x + t, frame.max.x - t);
        let (y0, y1) = (frame.min.y + t, frame.max.y - t);
        draw_hline(layer, x0, x1, y0, w, h);
        draw_hline(layer, x0, x1, y1, w, h);
        draw_vline(layer, y0, y1, x0, w, h);
        draw_vline(layer, y0, y1, x1, w, h);
    }
}

fn draw_hline(layer: &mut Layer, x0: isize, x1: isize, y: isize, w: usize, h: usize) {
    for x in x0..=x1 {
        put(layer, x, y, INK, w, h);
    }
}

fn draw_vline(layer: &mut Layer, y0: isize, y1: isize, x: isize, w: usize, h: usize) {
    for y in y0..=y1 {
        put(layer, x, y, INK, w, h);
    }
}

fn draw_balloon(layer: &mut Layer, b: &Balloon, w: usize, h: usize) {
    let (rx, ry) = (b.rx.max(1), b.ry.max(1));
    for dy in -ry..=ry {
        for dx in -rx..=rx {
            let outer = (dx * dx) as f32 / (rx * rx) as f32 + (dy * dy) as f32 / (ry * ry) as f32;
            if outer > 1.0 {
                continue;
            }
            let irx = (rx - 2).max(1);
            let iry = (ry - 2).max(1);
            let inner =
                (dx * dx) as f32 / (irx * irx) as f32 + (dy * dy) as f32 / (iry * iry) as f32;
            let color = if inner > 1.0 { INK } else { PAPER };
            put(layer, b.center.x + dx, b.center.y + dy, color, w, h);
        }
    }
    draw_tail(layer, b, w, h);
}

fn draw_tail(layer: &mut Layer, b: &Balloon, w: usize, h: usize) {
    let (rx, ry) = (b.rx.max(1), b.ry.max(1));
    let a = IVec2::new(b.center.x - rx / 3, b.center.y + ry * 4 / 5);
    let c = IVec2::new(b.center.x + rx / 3, b.center.y + ry * 4 / 5);
    for y in a.y.min(b.tail.y)..=a.y.max(b.tail.y) {
        for x in a.x.min(b.tail.x)..=c.x.max(b.tail.x) {
            if point_in_triangle(IVec2::new(x, y), a, c, b.tail) {
                put(layer, x, y, PAPER, w, h);
            }
        }
    }
    draw_line(layer, a, b.tail, w, h);
    draw_line(layer, c, b.tail, w, h);
}

fn point_in_triangle(p: IVec2, a: IVec2, b: IVec2, c: IVec2) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

fn sign(p: IVec2, a: IVec2, b: IVec2) -> f32 {
    ((p.x - b.x) * (a.y - b.y) - (a.x - b.x) * (p.y - b.y)) as f32
}

fn draw_line(layer: &mut Layer, a: IVec2, b: IVec2, w: usize, h: usize) {
    let steps = (b.x - a.x).abs().max((b.y - a.y).abs()).max(1);
    for s in 0..=steps {
        let x = a.x + (b.x - a.x) * s / steps;
        let y = a.y + (b.y - a.y) * s / steps;
        put(layer, x, y, INK, w, h);
    }
}

fn put(layer: &mut Layer, x: isize, y: isize, color: (u8, u8, u8, u8), w: usize, h: usize) {
    if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
        layer.pixels[y as usize][x as usize] = color;
    }
}

/// สีแดงสำหรับพรีวิวกรอบ (RGBA) — ใช้ฝั่ง UI
pub(crate) const PREVIEW_RED: egui::Color32 = egui::Color32::from_rgb(205, 45, 45);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::pages::build_page_layers;

    #[test]
    fn frame_and_balloon_rasterize_to_layer() {
        let mut app = App::new(None);
        app.w = 80;
        app.h = 80;
        app.layers = build_page_layers(80, 80);
        app.active = 1;
        app.begin_frame_drag(IVec2::new(4, 4));
        app.update_frame_drag(IVec2::new(70, 70));
        app.commit_frame_drag();
        app.begin_balloon_drag(IVec2::new(40, 40));
        app.commit_balloon_drag(IVec2::new(65, 60));
        assert_eq!(app.comic_frames.len(), 1);
        assert_eq!(app.balloons.len(), 1);
        app.rasterize_comic_items();
        assert!(app.layers[1]
            .pixels
            .iter()
            .any(|r| r.iter().any(|p| p.3 > 0)));
    }

    #[test]
    fn tiny_drag_is_ignored() {
        let mut app = App::new(None);
        app.begin_frame_drag(IVec2::new(4, 4));
        app.commit_frame_drag();
        assert!(app.comic_frames.is_empty());
    }
}

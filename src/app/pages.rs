// pages.rs — multi-page project engine (.cmc), 2-page spreads, folio and story script
use crate::state::store::{Layer, LayerId};
use eframe::egui;

use super::canvas::composite_layers;
use super::comic::{Balloon, ComicFrame};
use super::{App, LayerComp};

pub(crate) const THUMB_W: usize = 168;
pub(crate) const THUMB_H: usize = 96;
pub(crate) const SCRIPT_SEPARATOR: &str = "\n---\n";

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Binding {
    #[default]
    LeftToRight,
    RightToLeft,
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct PageDoc {
    pub(crate) name: String,
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) layers: Vec<Layer>,
    pub(crate) script: String,
    pub(crate) folio: u32,
    pub(crate) spread: bool,
    pub(crate) frames: Vec<ComicFrame>,
    pub(crate) balloons: Vec<Balloon>,
    pub(crate) thumb: Option<egui::TextureHandle>,
    pub(crate) thumb_dirty: bool,
    pub(crate) comps: Vec<LayerComp>,
}

impl PageDoc {
    pub(crate) fn new_page(name: &str, w: usize, h: usize, layers: Vec<Layer>) -> Self {
        Self {
            name: name.to_owned(),
            w,
            h,
            layers,
            script: String::new(),
            folio: 0,
            spread: false,
            frames: Vec::new(),
            balloons: Vec::new(),
            thumb: None,
            thumb_dirty: true,
            comps: Vec::new(),
        }
    }

    /// บรรทัดแรกของสคริปต์ ใช้แสดงใน Page Manager
    pub(crate) fn script_preview(&self) -> String {
        let line = self
            .script
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("");
        truncate_chars(line.trim(), 34)
    }
}

pub(crate) fn thumbnail_image(layers: &[Layer], w: usize, h: usize) -> egui::ColorImage {
    let mut pixels = Vec::with_capacity(THUMB_W * THUMB_H);
    for ty in 0..THUMB_H {
        let sy = (ty * h / THUMB_H).min(h.saturating_sub(1));
        for tx in 0..THUMB_W {
            let sx = (tx * w / THUMB_W).min(w.saturating_sub(1));
            let (r, g, b, a) = composite_pixel(layers, sx, sy);
            pixels.push(egui::Color32::from_rgba_unmultiplied(r, g, b, a));
        }
    }
    egui::ColorImage::new([THUMB_W, THUMB_H], pixels)
}

fn composite_pixel(layers: &[Layer], x: usize, y: usize) -> (u8, u8, u8, u8) {
    let mut out = (255u8, 255u8, 255u8, 255u8);
    for l in layers.iter().filter(|l| l.visible) {
        let (r, g, b, a) = l.pixels[y][x];
        let alpha = (a as f32 / 255.0_f32) * l.opacity_f32();
        out = blend_pixel(out, (r, g, b), alpha);
    }
    out
}

fn blend_pixel(dst: (u8, u8, u8, u8), rgb: (u8, u8, u8), alpha: f32) -> (u8, u8, u8, u8) {
    (
        (rgb.0 as f32 * alpha + dst.0 as f32 * (1.0_f32 - alpha)) as u8,
        (rgb.1 as f32 * alpha + dst.1 as f32 * (1.0_f32 - alpha)) as u8,
        (rgb.2 as f32 * alpha + dst.2 as f32 * (1.0_f32 - alpha)) as u8,
        255,
    )
}

pub(crate) fn build_page_layers(w: usize, h: usize) -> Vec<Layer> {
    vec![
        Layer::new(LayerId(0), "กระดาษ".to_string(), w, h, true),
        Layer::new(LayerId(1), "เลเยอร์ 1".to_string(), w, h, false),
    ]
}

fn truncate_chars(text: &str, max: usize) -> String {
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.push('…');
    }
    out
}

impl App {
    /// บันทึกสถานะผ้าใบปัจจุบันกลับลงหน้าในโปรเจกต์
    pub(crate) fn save_current_page(&mut self) {
        let (w, h) = (self.w, self.h);
        let Some(page) = self.pages.get_mut(self.page_index) else {
            return;
        };
        page.w = w;
        page.h = h;
        page.layers = self.layers.clone();
        page.frames = self.comic_frames.clone();
        page.balloons = self.balloons.clone();
        page.thumb_dirty = true;
    }

    /// สลับหน้าที่กำลังแก้ไข
    pub(crate) fn switch_page(&mut self, index: usize) {
        if index >= self.pages.len() || index == self.page_index {
            return;
        }
        let new_lock = match self.acquire_page_lock(index) {
            Ok(lock) => lock,
            Err(error) => {
                self.saved = error;
                return;
            }
        };
        self.save_current_page();
        self.release_page_lock();
        self.page_index = index;
        self.load_current_page();
        self.page_lock = Some(new_lock);
    }

    pub(crate) fn load_current_page(&mut self) {
        self.w = self.pages[self.page_index].w;
        self.h = self.pages[self.page_index].h;
        self.layers = self.pages[self.page_index].layers.clone();
        self.comic_frames = self.pages[self.page_index].frames.clone();
        self.balloons = self.pages[self.page_index].balloons.clone();
        self.active = self.active.min(self.layers.len().saturating_sub(1));
        self.selection = None;
        self.floating = None;
        self.undo_snapshots.clear();
        self.redo_snapshots.clear();
        self.selected_balloon = None;
        self.pan_offset = egui::Vec2::ZERO;
        self.texture_dirty = true;
    }

    pub(crate) fn add_page(&mut self) {
        let target_index = self.pages.len();
        let new_lock = match self.acquire_page_lock(target_index) {
            Ok(lock) => lock,
            Err(error) => {
                self.saved = error;
                return;
            }
        };
        self.save_current_page();
        self.release_page_lock();
        let (w, h) = (self.w, self.h);
        let index = self.pages.len();
        let mut page =
            PageDoc::new_page(&format!("หน้า {}", index + 1), w, h, build_page_layers(w, h));
        page.folio = index as u32 + 1;
        self.pages.push(page);
        self.page_index = index;
        self.load_current_page();
        self.page_lock = Some(new_lock);
    }

    pub(crate) fn delete_page(&mut self) {
        if self.pages.len() <= 1 {
            self.saved = "ต้องมีอย่างน้อย 1 หน้า".into();
            return;
        }
        self.release_page_lock();
        self.pages.remove(self.page_index);
        if self.page_index >= self.pages.len() {
            self.page_index = self.pages.len() - 1;
        }
        self.load_current_page();
        self.page_lock = self.acquire_page_lock(self.page_index).ok();
        self.renumber_folios();
    }

    pub(crate) fn move_page(&mut self, delta: isize) {
        let target = self.page_index as isize + delta;
        if target < 0 || target as usize >= self.pages.len() {
            return;
        }
        self.save_current_page();
        self.release_page_lock();
        let target = target as usize;
        self.pages.swap(self.page_index, target);
        self.page_index = target;
        self.renumber_folios();
        self.page_lock = self.acquire_page_lock(self.page_index).ok();
    }

    /// จัดเลขหน้า (Folio) ตามทิศทางการเข้าเล่ม
    pub(crate) fn renumber_folios(&mut self) {
        let binding = self.binding;
        let total = self.pages.len();
        for (i, page) in self.pages.iter_mut().enumerate() {
            page.folio = match binding {
                Binding::LeftToRight => i as u32 + 1,
                Binding::RightToLeft => (total - i) as u32,
            };
        }
    }

    pub(crate) fn merge_marked_pages(&mut self) -> Result<(), String> {
        if self.page_marks.len() != 2 {
            return Err("เลือกหน้าให้ครบ 2 หน้าเพื่อรวมเป็นหน้าคู่".into());
        }
        let mut idx: Vec<usize> = self.page_marks.iter().copied().collect();
        idx.sort_unstable();
        let (a, b) = (idx[0], idx[1]);
        if b >= self.pages.len() {
            return Err("หน้าไม่ถูกต้อง".into());
        }
        self.save_current_page();
        let merged = self.build_spread(a, b);
        self.pages.remove(b);
        self.pages.remove(a);
        self.pages.insert(a, merged);
        self.page_marks.clear();
        self.page_index = a;
        self.load_current_page();
        self.renumber_folios();
        Ok(())
    }

    fn build_spread(&self, a: usize, b: usize) -> PageDoc {
        let (pa, pb) = (&self.pages[a], &self.pages[b]);
        let (left, right) = match self.binding {
            Binding::LeftToRight => (pa, pb),
            Binding::RightToLeft => (pb, pa),
        };
        let (w, h) = (left.w + right.w, left.h.max(right.h));
        let mut layers = build_page_layers(w, h);
        let mut raster = Layer::new(LayerId(1), "ภาพรวมหน้าคู่".to_string(), w, h, false);
        blit_page(&mut raster, left, 0);
        blit_page(&mut raster, right, left.w);
        layers[1] = raster;

        let mut page =
            PageDoc::new_page(&format!("หน้าคู่ {} - {}", pa.folio, pb.folio), w, h, layers);
        page.folio = pa.folio.min(pb.folio);
        page.spread = true;
        page.frames = spread_frames(left, right);
        page.balloons = spread_balloons(left, right);
        page.script = join_scripts(&pa.script, &pb.script);
        page
    }

    pub(crate) fn split_spread_page(&mut self) -> Result<(), String> {
        let index = self.page_index;
        if !self.pages[index].spread {
            return Err("หน้านี้ไม่ใช่หน้าคู่".into());
        }
        self.save_current_page();
        let (left, right) = self.split_page(index);
        self.pages.remove(index);
        let order = match self.binding {
            Binding::LeftToRight => (left, right),
            Binding::RightToLeft => (right, left),
        };
        self.pages.insert(index, order.0);
        self.pages.insert(index + 1, order.1);
        self.page_index = index;
        self.load_current_page();
        self.renumber_folios();
        Ok(())
    }

    fn split_page(&self, index: usize) -> (PageDoc, PageDoc) {
        let page = &self.pages[index];
        let half = (page.w / 2).max(1);
        let src = composite_layers(&page.layers, page.w, page.h);
        let left = half_page(page, &src, 0, half, "ซ้าย");
        let right = half_page(page, &src, half, page.w - half, "ขวา");
        (left, right)
    }

    /// ค้นหาข้อความในสคริปต์ทุกหน้า (ไม่แก้ไข)
    pub(crate) fn find_in_story(&mut self) -> usize {
        let needle = self.story_find.clone();
        if needle.is_empty() {
            self.story_status = "กรุณากรอกคำค้นหา".into();
            return 0;
        }
        let pages = self
            .pages
            .iter()
            .filter(|p| p.script.contains(&needle))
            .count();
        let total: usize = self
            .pages
            .iter()
            .map(|p| p.script.matches(&needle).count())
            .sum();
        self.story_status = format!("พบ “{}” {} จุด ใน {} หน้า", needle, total, pages);
        total
    }

    /// แทนที่ข้อความในสคริปต์ทุกหน้า แล้วอัปเดตกลับลงหน้าเหล่านั้น
    pub(crate) fn replace_in_story(&mut self) -> usize {
        let needle = self.story_find.clone();
        if needle.is_empty() {
            self.story_status = "กรุณากรอกคำค้นหา".into();
            return 0;
        }
        let replacement = self.story_replace.clone();
        let mut count = 0;
        let mut touched = 0;
        for page in &mut self.pages {
            let hits = page.script.matches(&needle).count();
            if hits > 0 {
                page.script = page.script.replace(&needle, &replacement);
                count += hits;
                touched += 1;
            }
        }
        self.story_status = format!("แก้ไข {} จุด ใน {} หน้า", count, touched);
        count
    }

    /// นำเข้าบทพูดทีละหน้า โดยคั่นด้วยบรรทัด "---"
    pub(crate) fn batch_assign_story(&mut self) {
        let text = self.story_batch.trim().to_owned();
        if text.is_empty() {
            self.story_status = "ยังไม่มีข้อความสำหรับนำเข้า".into();
            return;
        }
        let chunks: Vec<&str> = text.split(SCRIPT_SEPARATOR).map(|s| s.trim()).collect();
        let mut written = 0;
        for (i, chunk) in chunks.iter().enumerate() {
            let target = self.story_target + i;
            if target >= self.pages.len() {
                break;
            }
            self.pages[target].script = (*chunk).to_string();
            written += 1;
        }
        self.story_status = format!("นำเข้าบทพูดลง {} หน้า", written);
    }
}

fn blit_page(dst: &mut Layer, src: &PageDoc, x0: usize) {
    let rgba = composite_layers(&src.layers, src.w, src.h);
    for y in 0..src.h.min(dst.pixels.len()) {
        for x in 0..src.w {
            let target = x0 + x;
            if target >= dst.pixels[y].len() {
                break;
            }
            let p = rgba[y * src.w + x];
            dst.pixels[y][target] = (p[0], p[1], p[2], p[3]);
        }
    }
}

fn half_page(page: &PageDoc, src: &[[u8; 4]], x0: usize, w: usize, side: &str) -> PageDoc {
    let mut layers = build_page_layers(w, page.h);
    let mut raster = Layer::new(LayerId(1), "ภาพแยกหน้าคู่".to_string(), w, page.h, false);
    for y in 0..page.h {
        for x in 0..w {
            let p = src[y * page.w + x0 + x];
            raster.pixels[y][x] = (p[0], p[1], p[2], p[3]);
        }
    }
    layers[1] = raster;
    let mut half = PageDoc::new_page(&format!("หน้า {} ({})", page.folio, side), w, page.h, layers);
    half.folio = page.folio;
    half.script = page.script.clone();
    let (x0i, x1i) = (x0 as isize, (x0 + w) as isize);
    half.frames = page
        .frames
        .iter()
        .filter(|f| f.max.x > x0i && f.min.x < x1i)
        .map(|f| f.shifted(-x0i, 0))
        .collect();
    half.balloons = page
        .balloons
        .iter()
        .filter(|b| b.center.x >= x0i && b.center.x < x1i)
        .map(|b| b.shifted(-x0i, 0))
        .collect();
    half
}

fn spread_frames(left: &PageDoc, right: &PageDoc) -> Vec<ComicFrame> {
    let mut frames = left.frames.clone();
    frames.extend(right.frames.iter().map(|f| f.shifted(left.w as isize, 0)));
    frames
}

fn spread_balloons(left: &PageDoc, right: &PageDoc) -> Vec<Balloon> {
    let mut balloons = left.balloons.clone();
    balloons.extend(right.balloons.iter().map(|b| b.shifted(left.w as isize, 0)));
    balloons
}

fn join_scripts(a: &str, b: &str) -> String {
    match (a.trim().is_empty(), b.trim().is_empty()) {
        (true, true) => String::new(),
        (true, false) => b.trim().to_string(),
        (false, true) => a.trim().to_string(),
        (false, false) => format!("{}\n---\n{}", a.trim(), b.trim()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new(None);
        app.w = 64;
        app.h = 48;
        app.layers = build_page_layers(64, 48);
        app.pages = vec![PageDoc::new_page("หน้า 1", 64, 48, app.layers.clone())];
        app
    }

    #[test]
    fn add_switch_move_delete_pages() {
        let mut app = test_app();
        app.add_page();
        assert_eq!(app.pages.len(), 2);
        assert_eq!(app.page_index, 1);
        app.switch_page(0);
        assert_eq!(app.page_index, 0);
        app.move_page(1);
        assert_eq!(app.page_index, 1);
        app.delete_page();
        assert_eq!(app.pages.len(), 1);
        assert_eq!(app.page_index, 0);
    }

    #[test]
    fn merge_and_split_spread_roundtrip() {
        let mut app = test_app();
        app.add_page();
        app.page_marks.insert(0);
        app.page_marks.insert(1);
        assert!(app.merge_marked_pages().is_ok());
        assert_eq!(app.pages.len(), 1);
        assert!(app.pages[0].spread);
        assert_eq!(app.pages[0].w, 128);
        assert!(app.split_spread_page().is_ok());
        assert_eq!(app.pages.len(), 2);
        assert!(!app.pages[0].spread);
        assert_eq!(app.pages[0].w, 64);
    }

    #[test]
    fn story_replace_updates_every_page() {
        let mut app = test_app();
        app.add_page();
        app.pages[0].script = "แมว นอน".into();
        app.pages[1].script = "แมว กิน".into();
        app.story_find = "แมว".into();
        app.story_replace = "หมา".into();
        assert_eq!(app.replace_in_story(), 2);
        assert_eq!(app.pages[0].script, "หมา นอน");
        assert_eq!(app.pages[1].script, "หมา กิน");
        assert!(app.story_status.contains('2'));
    }

    #[test]
    fn story_batch_assign_fills_pages() {
        let mut app = test_app();
        app.add_page();
        app.story_batch = "หน้าแรก\n---\nหน้าที่สอง".into();
        app.story_target = 0;
        app.batch_assign_story();
        assert_eq!(app.pages[0].script, "หน้าแรก");
        assert_eq!(app.pages[1].script, "หน้าที่สอง");
    }
}

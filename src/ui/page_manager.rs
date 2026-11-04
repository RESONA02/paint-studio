// page_manager.rs — Page Manager window: thumbnails, reorder, add/delete and spreads
use crate::app::pages::{thumbnail_image, Binding, THUMB_H, THUMB_W};
use crate::app::App;
use eframe::egui;

impl App {
    pub(crate) fn page_manager_window(&mut self, ctx: &egui::Context) {
        let mut open = self.page_manager_open;
        egui::Window::new("จัดการหน้า (Page Manager)")
            .open(&mut open)
            .default_size([660.0, 540.0])
            .resizable(true)
            .show(ctx, |ui| {
                self.page_manager_tools(ui);
                ui.separator();
                self.page_manager_grid(ctx, ui);
            });
        self.page_manager_open = open;
    }

    fn page_manager_tools(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("＋ เพิ่มหน้า").clicked() {
                self.add_page();
            }
            if ui.button("🗑 ลบหน้า").clicked() {
                self.delete_page();
            }
            if ui.button("▲ ย้ายขึ้น").clicked() {
                self.move_page(-1);
            }
            if ui.button("▼ ย้ายลง").clicked() {
                self.move_page(1);
            }
            if ui.button("จัดเลขหน้า (Folio)").clicked() {
                self.renumber_folios();
            }
        });
        ui.horizontal(|ui| {
            ui.label("เข้าเล่ม:");
            ui.selectable_value(&mut self.binding, Binding::LeftToRight, "ซ้าย → ขวา");
            ui.selectable_value(&mut self.binding, Binding::RightToLeft, "ขวา → ซ้าย");
            ui.separator();
            let can_merge = self.page_marks.len() == 2;
            if ui
                .add_enabled(can_merge, egui::Button::new("รวมหน้าคู่"))
                .on_hover_text("รวม 2 หน้าที่เลือกเป็นหน้าคู่พาโนรามา")
                .clicked()
            {
                self.merge_selected_report();
            }
            let can_split = self.pages[self.page_index].spread;
            if ui
                .add_enabled(can_split, egui::Button::new("แยกหน้าคู่"))
                .clicked()
            {
                self.split_selected_report();
            }
        });
        ui.label(format!(
            "เลือก {} หน้า · หน้าปัจจุบัน {}/{}",
            self.page_marks.len(),
            self.page_index + 1,
            self.pages.len()
        ));
        self.page_manager_io(ui);
    }

    fn page_manager_io(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("บันทึกโปรเจกต์ .cmc").clicked() {
                if let Err(error) = self.save_project("shots/project.cmc") {
                    self.saved = error;
                }
            }
            if ui.button("เปิดโปรเจกต์ .cmc").clicked() {
                if let Err(error) = self.load_project("shots/project.cmc") {
                    self.saved = error;
                }
            }
            ui.label("shots/project.cmc");
        });
    }

    fn merge_selected_report(&mut self) {
        match self.merge_marked_pages() {
            Ok(()) => self.saved = "รวมหน้าเป็นหน้าคู่พาโนรามาแล้ว".into(),
            Err(e) => self.saved = e,
        }
    }

    fn split_selected_report(&mut self) {
        match self.split_spread_page() {
            Ok(()) => self.saved = "แยกหน้าคู่กลับเป็นหน้าเดี่ยวแล้ว".into(),
            Err(e) => self.saved = e,
        }
    }

    fn page_manager_grid(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for i in 0..self.pages.len() {
                    self.page_card(ctx, ui, i);
                }
            });
        });
    }

    fn page_card(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, i: usize) {
        self.ensure_thumb(ctx, i);
        let mut switch = false;
        ui.group(|ui| {
            ui.set_width(178.0);
            ui.vertical(|ui| {
                if let Some(tex) = &self.pages[i].thumb {
                    let source = (tex.id(), egui::vec2(THUMB_W as f32, THUMB_H as f32));
                    if ui
                        .add(egui::Image::new(source).sense(egui::Sense::click()))
                        .clicked()
                    {
                        switch = true;
                    }
                }
                let title = format!("#{} · folio {}", i + 1, self.pages[i].folio);
                ui.label(egui::RichText::new(title).strong().size(12.0_f32));
                self.page_card_meta(ui, i);
                let mut marked = self.page_marks.contains(&i);
                if ui.checkbox(&mut marked, "เลือกเพื่อรวม").changed() {
                    self.mark_page(i, marked);
                }
            });
        });
        if switch {
            self.switch_page(i);
        }
    }

    fn page_card_meta(&mut self, ui: &mut egui::Ui, i: usize) {
        if self.pages[i].spread {
            ui.label(
                egui::RichText::new("หน้าคู่พาโนรามา")
                    .size(10.0_f32)
                    .color(egui::Color32::from_rgb(150, 30, 30)),
            );
        }
        let preview = self.pages[i].script_preview();
        if !preview.is_empty() {
            ui.label(
                egui::RichText::new(preview)
                    .size(10.0_f32)
                    .color(egui::Color32::from_rgb(70, 70, 70)),
            );
        }
    }

    fn mark_page(&mut self, i: usize, on: bool) {
        if on {
            self.page_marks.insert(i);
        } else {
            self.page_marks.remove(&i);
        }
    }

    fn ensure_thumb(&mut self, ctx: &egui::Context, i: usize) {
        if i == self.page_index {
            let image = thumbnail_image(&self.layers, self.w, self.h);
            self.set_page_thumb(ctx, i, image);
        } else if self.pages[i].thumb_dirty || self.pages[i].thumb.is_none() {
            let image = thumbnail_image(&self.pages[i].layers, self.pages[i].w, self.pages[i].h);
            self.set_page_thumb(ctx, i, image);
        }
    }

    fn set_page_thumb(&mut self, ctx: &egui::Context, i: usize, image: egui::ColorImage) {
        let options = egui::TextureOptions::LINEAR;
        let page = &mut self.pages[i];
        match &mut page.thumb {
            Some(tex) => tex.set(image, options),
            None => {
                page.thumb = Some(ctx.load_texture(format!("page_thumb_{i}"), image, options));
            }
        }
        page.thumb_dirty = false;
    }
}

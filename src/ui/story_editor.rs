// story_editor.rs — central script editor for every page: batch input, find & replace
use crate::app::App;
use eframe::egui;

impl App {
    pub(crate) fn story_editor_window(&mut self, ctx: &egui::Context) {
        self.story_target = self.story_target.min(self.pages.len().saturating_sub(1));
        let mut open = self.story_editor_open;
        egui::Window::new("สคริปต์ / บทพูด (Story Editor)")
            .open(&mut open)
            .default_size([880.0, 620.0])
            .resizable(true)
            .show(ctx, |ui| {
                egui::SidePanel::left("story_page_list")
                    .resizable(true)
                    .default_width(200.0_f32)
                    .show_inside(ui, |ui| self.story_page_list(ui));
                egui::CentralPanel::default().show_inside(ui, |ui| self.story_editor_body(ui));
            });
        self.story_editor_open = open;
    }

    fn story_page_list(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("รายการหน้า").strong());
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("story_pages")
            .show(ui, |ui| {
                for i in 0..self.pages.len() {
                    let label = format!(
                        "หน้า {} · folio {}\n{}",
                        i + 1,
                        self.pages[i].folio,
                        self.pages[i].script_preview()
                    );
                    let selected = i == self.story_target;
                    if ui
                        .add_sized(
                            [176.0_f32, 44.0_f32],
                            egui::Button::selectable(selected, label),
                        )
                        .clicked()
                    {
                        self.story_target = i;
                    }
                }
            });
    }

    fn story_editor_body(&mut self, ui: &mut egui::Ui) {
        let index = self.story_target;
        ui.horizontal(|ui| {
            ui.strong(format!(
                "หน้า {} · folio {}",
                index + 1,
                self.pages[index].folio
            ));
            ui.separator();
            if ui.button("＋ เพิ่มหน้า").clicked() {
                self.add_page();
            }
        });
        ui.label("สคริปต์ของหน้านี้ (แก้ไขแล้วบันทึกลงหน้าทันที)");
        ui.add(
            egui::TextEdit::multiline(&mut self.pages[index].script)
                .desired_rows(8)
                .desired_width(f32::INFINITY)
                .hint_text("บทพูด / คำบรรยายของหน้านี้"),
        );
        ui.separator();
        self.story_batch_input(ui);
        ui.separator();
        self.story_find_replace(ui);
        self.story_status_line(ui);
    }

    fn story_batch_input(&mut self, ui: &mut egui::Ui) {
        ui.strong("Batch Text Input (คั่นแต่ละหน้าด้วยบรรทัด ---)");
        ui.add(
            egui::TextEdit::multiline(&mut self.story_batch)
                .desired_rows(4)
                .desired_width(f32::INFINITY)
                .hint_text("เนื้อหาหน้า 1\n---\nเนื้อหาหน้า 2"),
        );
        if ui.button("นำเข้าลงหน้า (เริ่มจากหน้าปัจจุบัน)").clicked()
        {
            self.batch_assign_story();
        }
    }

    fn story_find_replace(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ค้นหา");
            ui.add(egui::TextEdit::singleline(&mut self.story_find).desired_width(160.0_f32));
            ui.label("แทนที่ด้วย");
            ui.add(egui::TextEdit::singleline(&mut self.story_replace).desired_width(160.0_f32));
        });
        ui.horizontal(|ui| {
            if ui.button("ค้นหาในเล่ม").clicked() {
                self.find_in_story();
            }
            if ui.button("แทนที่ทั้งหมด").clicked() {
                self.replace_in_story();
            }
            if ui.button("ล้างผล").clicked() {
                self.story_status.clear();
            }
        });
    }

    fn story_status_line(&self, ui: &mut egui::Ui) {
        if self.story_status.is_empty() {
            return;
        }
        ui.label(
            egui::RichText::new(&self.story_status)
                .strong()
                .color(egui::Color32::from_rgb(0, 110, 40)),
        );
    }
}

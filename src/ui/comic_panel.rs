// comic_panel.rs — floating properties for comic frame borders and speech balloons
use crate::app::App;
use crate::tools::Tool;
use eframe::egui;

impl App {
    pub(crate) fn comic_properties_window(&mut self, ctx: &egui::Context) {
        if self.hide_all_ui || !matches!(self.tool, Tool::ComicFrame | Tool::Balloon) {
            return;
        }
        egui::Window::new("กรอบช่อง & บอลลูน")
            .default_pos([360.0_f32, 140.0_f32])
            .resizable(false)
            .show(ctx, |ui| self.comic_properties_body(ui));
    }

    fn comic_properties_body(&mut self, ui: &mut egui::Ui) {
        ui.label("ลากบนผ้าใบเพื่อสร้างกรอบช่องหรือบอลลูน");
        ui.label(format!(
            "กรอบ {} · บอลลูน {}",
            self.comic_frames.len(),
            self.balloons.len()
        ));
        ui.separator();
        self.balloon_editor(ui);
        ui.separator();
        if ui.button("แปลงเป็นภาพลงเลเยอร์").clicked() {
            self.rasterize_comic_items();
        }
        if !self.comic_frames.is_empty() && ui.button("ลบกรอบล่าสุด").clicked()
        {
            self.comic_frames.pop();
        }
        if ui.button("ล้างกรอบ/บอลลูนทั้งหมด").clicked() {
            self.comic_frames.clear();
            self.balloons.clear();
            self.selected_balloon = None;
        }
    }

    fn balloon_editor(&mut self, ui: &mut egui::Ui) {
        let Some(index) = self.selected_balloon else {
            ui.label("คลิกบอลลูนบนผ้าใบเพื่อแก้ข้อความ");
            return;
        };
        let mut delete = false;
        if let Some(balloon) = self.balloons.get_mut(index) {
            ui.horizontal(|ui| {
                ui.label(format!("บอลลูน {}", index + 1));
                delete = ui.button("ลบ").clicked();
            });
            ui.add(
                egui::TextEdit::multiline(&mut balloon.text)
                    .desired_rows(3)
                    .desired_width(280.0_f32),
            );
        }
        if delete {
            self.delete_balloon(index);
        }
    }
}

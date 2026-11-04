// lt_dialog.rs — 2D LT conversion window (line art + screentone)
use crate::app::App;
use eframe::egui;

impl App {
    pub(crate) fn lt_dialog_window(&mut self, ctx: &egui::Context) {
        let mut open = self.lt_dialog_open;
        egui::Window::new("แปลง 2D LT (Line Art + Screentone)")
            .open(&mut open)
            .default_width(440.0_f32)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("ดึงเส้นขอบจากภาพถ่าย/ภาพวาด แล้วแยกแสงเงาเป็นเม็ดสกรีนโทน");
                ui.separator();
                ui.strong("Line Art");
                ui.add(egui::Slider::new(&mut self.lt_threshold, 8..=120).text("Threshold"));
                if ui.button("สร้างเลเยอร์เส้น (Line Art)").clicked()
                {
                    self.apply_line_art();
                }
                ui.separator();
                ui.strong("Screentone");
                ui.add(egui::Slider::new(&mut self.lt_levels, 2..=4).text("จำนวนโทน"));
                ui.add(egui::Slider::new(&mut self.lt_dot, 1..=4).text("ขนาดเม็ด"));
                if ui.button("สร้างเลเยอร์สกรีนโทน").clicked()
                {
                    self.apply_screentone();
                }
                ui.separator();
                ui.label("ผลลัพธ์จะถูกเพิ่มเป็นเลเยอร์ใหม่บนหน้าปัจจุบัน");
                if !self.saved.is_empty() {
                    ui.label(
                        egui::RichText::new(&self.saved)
                            .size(11.0_f32)
                            .color(egui::Color32::from_rgb(0, 100, 0)),
                    );
                }
            });
        self.lt_dialog_open = open;
    }
}

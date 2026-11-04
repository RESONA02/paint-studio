// dialog.rs — Shortcut Key Settings window
use crate::app::App;
use eframe::egui;

const CATEGORIES: [&str; 4] = [
    "Main menu (เมนูหลัก)",
    "Palette (พาเนล)",
    "Tool (เครื่องมือ)",
    "Pop-up menu (เมนูด่วน)",
];

impl App {
    pub(crate) fn shortcut_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("ตั้งค่าคีย์ลัด (Shortcut Key Settings)")
            .collapsible(false)
            .resizable(false)
            .default_size(egui::vec2(640.0_f32, 480.0_f32))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0_f32, 0.0_f32))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0_f32, 8.0_f32);
                self.category_tabs(ui);
                ui.separator();
                self.shortcut_table(ui);
                ui.separator();
                self.dialog_footer(ui);
            });
    }

    fn category_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Setting area:").strong());
            for (idx, cat) in CATEGORIES.iter().enumerate() {
                if ui
                    .selectable_label(self.shortcut_category == idx, *cat)
                    .clicked()
                {
                    self.shortcut_category = idx;
                }
            }
        });
    }

    fn shortcut_table(&mut self, ui: &mut egui::Ui) {
        let category = self.shortcut_category;
        egui::ScrollArea::vertical()
            .max_height(340.0_f32)
            .show(ui, |ui| {
                egui::Grid::new("shortcut_grid")
                    .striped(true)
                    .min_col_width(240.0_f32)
                    .show(ui, |ui| {
                        header_row(ui);
                        for (cmd, sc) in rows_for(category) {
                            ui.label(*cmd);
                            ui.label(shortcut_text(sc, category == 2));
                            ui.end_row();
                        }
                    });
            });
    }

    fn dialog_footer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("คืนค่าเริ่มต้น (Reset to Default)").clicked()
            {
                self.saved = "คืนค่าคีย์ลัดเป็นค่าเริ่มต้นแล้ว".into();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ตกลง (OK)").clicked() {
                    self.show_shortcut_dialog = false;
                }
                if ui.button("ยกเลิก (Cancel)").clicked() {
                    self.show_shortcut_dialog = false;
                }
            });
        });
    }
}

fn header_row(ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("คำสั่ง / เครื่องมือ (Command / Tool)").strong());
    ui.label(egui::RichText::new("คีย์ลัดปัจจุบัน (Current Shortcut)").strong());
    ui.end_row();
}

fn shortcut_text(text: &str, strong: bool) -> egui::RichText {
    let text = egui::RichText::new(text).monospace();
    if strong {
        text.strong()
    } else {
        text
    }
}

fn rows_for(category: usize) -> &'static [(&'static str, &'static str)] {
    match category {
        0 => MAIN_MENU_ROWS,
        1 => PALETTE_ROWS,
        2 => TOOL_ROWS,
        _ => POPUP_ROWS,
    }
}

const MAIN_MENU_ROWS: &[(&str, &str)] = &[
    ("ไฟล์ > ใหม่ (New)", "Ctrl + N"),
    ("ไฟล์ > เปิด (Open)", "Ctrl + O"),
    ("ไฟล์ > ปิด (Close)", "Ctrl + W"),
    ("ไฟล์ > บันทึก (Save)", "Ctrl + S"),
    ("ไฟล์ > ออกจากโปรแกรม (Exit)", "Ctrl + Q"),
    ("แก้ไข > ย้อนกลับ (Undo)", "Ctrl + Z"),
    ("แก้ไข > ทำซ้ำ (Redo)", "Ctrl + Y"),
    ("แก้ไข > ตัด (Cut / Lift)", "Ctrl + X"),
    ("แก้ไข > คัดลอก (Copy)", "Ctrl + C"),
    ("แก้ไข > วาง (Paste)", "Ctrl + V"),
    ("แก้ไข > ปรับขนาด/หมุน (Transform)", "Ctrl + T"),
    ("แก้ไข > เติมสี (Fill)", "Alt + Del / Alt + Backspace"),
    ("เลเยอร์ > เลเยอร์แรสเตอร์ใหม่ (New Layer)", "Ctrl + Shift + N"),
    ("เลเยอร์ > รวมกับเลเยอร์ด้านล่าง (Merge Down)", "Ctrl + E"),
    ("เลเยอร์ > รวมที่มองเห็นสร้างเลเยอร์ใหม่", "Ctrl + Shift + Alt + E"),
    ("เลือก > เลือกทั้งหมด (Select All)", "Ctrl + A"),
    ("เลือก > ยกเลิกการเลือก (Deselect)", "Ctrl + D"),
    ("เลือก > สลับพื้นที่เลือก (Inverse)", "Ctrl + Shift + I"),
    ("ดู > ซูมเข้า (Zoom In)", "Ctrl + (+) / Num+"),
    ("ดู > ซูมออก (Zoom Out)", "Ctrl + (-) / Num-"),
    ("ดู > พอดีหน้าจอ (Fit Screen)", "Ctrl + 0"),
    ("ดู > ซ่อน/แสดง UI ทั้งหมด", "Ctrl + Shift + H"),
    ("ดู > ซ่อน/แสดงพาเนลด้านข้าง", "Tab"),
];

const PALETTE_ROWS: &[(&str, &str)] = &[
    ("ซ่อน/แสดงพาเนลด้านข้างทั้งหมด (Toggle Docks)", "Tab"),
    ("ซ่อน/แสดงกรอบ UI ทั้งหมดเพื่อดูภาพเต็มตา", "Ctrl + Shift + H"),
    ("แถบเครื่องมือ (Tool Strip)", "Ctrl + 2"),
    ("คุณสมบัติเครื่องมือ (Tool Property)", "Ctrl + 3"),
    ("เลเยอร์ (Layer Palette)", "Ctrl + 4"),
    ("เนวิเกเตอร์ (Navigator)", "Ctrl + 5"),
    ("วงล้อสี (Color Wheel)", "Ctrl + 6"),
];

const TOOL_ROWS: &[(&str, &str)] = &[
    ("พู่กัน (Brush / Pen)", "B"),
    ("ปากกา (Pen)", "P"),
    ("ยางลบ (Eraser)", "E"),
    ("ฟิกเกอร์ 3D / รูปทรง (Direct Draw / Figure)", "U"),
    ("เครื่องมือเลือกพื้นที่ (Marquee / Selection)", "M"),
    ("บ่วงบาศเลือกพื้นที่อิสระ (Lasso)", "L"),
    ("ไม้กายสิทธิ์เลือกพื้นที่สี (Magic Wand)", "W"),
    ("ถังเทสี (Fill / Gradient)", "G"),
    ("เครื่องมือเกลี่ยสี/ปาดสี (Blend / Finger)", "O"),
    ("พู่กันเกลี่ยสี/ผสมสี (Color Mixing)", "J"),
    ("ดูดสี (Eyedropper)", "I / Hold Alt"),
    ("พ่นสเปรย์ (Airbrush)", "K"),
    ("ไม้บรรทัด/เครื่องมือช่วยวาด (Ruler)", "Shift + P"),
    ("เลื่อนมุมมองหน้ากระดาษ (Hand / Grab)", "H / Hold Space"),
    ("ย่อ/ขยาย (Zoom)", "Z / Ctrl+Space / Alt+Space"),
];

const POPUP_ROWS: &[(&str, &str)] = &[
    ("เมนูด่วนเลเยอร์ (Quick Layer Menu)", "Right Click on Canvas"),
    ("หยิบส่วนที่เลือกขึ้นมาลอย (Lift)", "Ctrl + X / L"),
    ("ยืนยันการวางเลเยอร์ลอย (Commit)", "Enter"),
    ("ยกเลิกการลอย (Cancel)", "Esc"),
    ("ลบเนื้อหาลอย (Delete Floating)", "Del"),
];

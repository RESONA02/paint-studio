// menu/file.rs — เมนู ไฟล์(F) ตามโครงสร้าง Clip Studio Paint ภาษาไทย
use crate::app::App;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn, menu_sub, sub_off};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("ไฟล์(F)", |ui| {
        ui.set_min_width(260.0);
        file_ops(ui, app);
        ui.separator();
        save_group(ui, app);
        ui.separator();
        export_group(ui, app);
        ui.separator();
        timelapse_group(ui);
        ui.separator();
        import_group(ui);
        ui.separator();
        print_group(ui);
        ui.separator();
        settings_group(ui, app);
        ui.separator();
        device_group(ui);
        ui.separator();
        external_app_group(ui);
        ui.separator();
        exit_item(ui);
    });
}

fn file_ops(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "ใหม่(N)...", "Ctrl+N").clicked() {
        app.new_canvas();
        ui.close();
    }
    item_off(ui, "สร้างใหม่จากเทมเพลต(B)");
    if menu_btn(ui, true, "เปิด(O)...", "Ctrl+O").clicked() {
        if let Err(error) = app.load_project("shots/project.cmc") {
            app.saved = error;
        }
        ui.close();
    }
    menu_sub(ui, true, "ล่าสุด(F)", |ui| {
        ui.set_min_width(200.0);
        let _ = menu_btn(ui, false, "(ไม่มีไฟล์ล่าสุด)", "");
    });
    ui.separator();
    if menu_btn(ui, true, "ปิด(C)", "Ctrl+W").clicked() {
        ui.close();
    }
}

fn save_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "บันทึก(S)", "Ctrl+S").clicked() {
        let _ = app.save_png();
        ui.close();
    }
    if menu_btn(ui, true, "บันทึกเป็น(A)...", "Shift+Alt+S").clicked() {
        let _ = app.save_png();
        ui.close();
    }
    menu_sub(ui, true, "บันทึกสำเนา(R)", |ui| {
        ui.set_min_width(220.0);
        if menu_btn(ui, true, ".cmc (โปรเจกต์มังงะหลายหน้า)...", "").clicked() {
            if let Err(error) = app.save_project("shots/project.cmc") {
                app.saved = error;
            }
            ui.close();
        }
        item(ui, ".clip (CLIP STUDIO PAINT)...", "");
        item(ui, ".psd (Photoshop Document)...", "");
    });
    item_off(ui, "บันทึกกลับ(G)...");
}

fn export_group(ui: &mut egui::Ui, app: &mut App) {
    menu_sub(ui, true, "ส่งออก (เลเยอร์เดียว)(R)", |ui| {
        ui.set_min_width(200.0);
        if menu_btn(ui, true, ".png (PNG)...", "").clicked() {
            let _ = app.save_png();
            ui.close();
        }
        item(ui, ".jpg (JPEG)...", "");
        item(ui, ".bmp (BMP)...", "");
        item(ui, ".psd (Photoshop Document)...", "");
        item(ui, ".tif (TIFF)...", "");
    });
    sub_off(ui, "ส่งออกภาพเคลื่อนไหว(Y)");
    item_off(ui, "ส่งออกไปยัง webtoon...");
    item_off(ui, "ส่งออกไทม์แลปส์(V)...");
}

fn timelapse_group(ui: &mut egui::Ui) {
    sub_off(ui, "ไทม์แลปส์(L)");
}

fn import_group(ui: &mut egui::Ui) {
    sub_off(ui, "นำเข้า(I)");
}

fn print_group(ui: &mut egui::Ui) {
    item_off(ui, "ตั้งค่าพิมพ์(D)...");
    item(ui, "พิมพ์(P)...", "Ctrl+P");
}

fn settings_group(ui: &mut egui::Ui, app: &mut App) {
    item(ui, "ตั้งค่า(E)...", "Ctrl+K");
    item_off(ui, "การตั้งค่าความเป็นส่วนตัว...");
    item_off(ui, "ตั้งค่าแถบคำสั่ง(B)...");
    if menu_btn(ui, true, "ตั้งค่าคีย์ลัด(H)...", "Ctrl+Shift+Alt+K").clicked() {
        app.show_shortcut_dialog = true;
        ui.close();
    }
    item(ui, "ตั้งค่าปุ่มโมดิฟายเออร์(K)...", "Ctrl+Shift+Alt+Y");
    item_off(ui, "จัดการฟังก์ชัน(F)...");
}

fn device_group(ui: &mut egui::Ui) {
    sub_off(ui, "เชื่อมต่อกับสตรีม Tab-Mate");
    sub_off(ui, "คอนโทรลเลอร์ Tab-Mate");
    sub_off(ui, "CLIP STUDIO TABMATE");
    item_off(ui, "ตั้งค่าแรงกดปากกา(J)...");
    sub_off(ui, "QUMARION(I)");
}

fn external_app_group(ui: &mut egui::Ui) {
    item_off(ui, "เปิด CLIP STUDIO...");
}

fn exit_item(ui: &mut egui::Ui) {
    if menu_btn(ui, true, "ออกจาก CLIP STUDIO PAINT(X)", "Ctrl+Q").clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

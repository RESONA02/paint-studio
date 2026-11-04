// menu/view.rs — เมนู ดู(V) ตามโครงสร้าง Clip Studio Paint ภาษาไทย
use crate::app::App;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn, menu_sub, sub_off};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("ดู(V)", |ui| {
        ui.set_min_width(260.0);
        zoom_rotate_group(ui, app);
        ui.separator();
        canvas_display_group(ui);
        ui.separator();
        ruler_and_grid_group(ui);
        ui.separator();
        fps_group(ui);
        ui.separator();
        snapping_group(ui);
        ui.separator();
        color_profile_group(ui);
    });
}

fn zoom_rotate_group(ui: &mut egui::Ui, app: &mut App) {
    menu_sub(ui, true, "หมุน/พลิก(V)", |ui| {
        ui.set_min_width(200.0);
        if menu_btn(ui, true, "หมุนตามเข็ม 90 องศา", "").clicked() {
            app.rotation += 90.0;
            ui.close();
        }
        if menu_btn(ui, true, "หมุนทวนเข็ม 90 องศา", "").clicked() {
            app.rotation -= 90.0;
            ui.close();
        }
        if menu_btn(ui, true, "รีเซ็ตการหมุน", "").clicked() {
            app.rotation = 0.0;
            ui.close();
        }
    });
    if menu_btn(ui, true, "ซูมเข้า(I)", "Ctrl+Num+").clicked() {
        app.zoom = (app.zoom * 1.25).min(32.0);
        ui.close();
    }
    if menu_btn(ui, true, "ซูมออก(O)", "Ctrl+Num-").clicked() {
        app.zoom = (app.zoom / 1.25).max(0.05);
        ui.close();
    }
    if menu_btn(ui, true, "100%", "Ctrl+Alt+0").clicked() {
        app.zoom = 1.0;
        ui.close();
    }
    if menu_btn(ui, true, "200%", "").clicked() {
        app.zoom = 2.0;
        ui.close();
    }
    if menu_btn(ui, true, "เต็มหน้าจอ(W)", "Ctrl+0").clicked() {
        app.zoom = 1.0;
        app.pan_offset = egui::Vec2::ZERO;
        ui.close();
    }
    if menu_btn(ui, true, "รีเซ็ตความคมชัด/หมุน(S)", "Ctrl+_").clicked() {
        app.zoom = 1.0;
        app.pan_offset = egui::Vec2::ZERO;
        app.rotation = 0.0;
        ui.close();
    }
    item_off(ui, "ปรับเปลี่ยนหน้าใบให้เต็มหน้าจอ(F)");
    item_off(ui, "ขนาดพิมพ์(Z)");
}

fn canvas_display_group(ui: &mut egui::Ui) {
    item_off(ui, "แถบเลื่อน(B)");
    item_off(ui, "ขอบเขตการเลือก(F)");
    item_off(ui, "เมนูการเลือก(L)");
    item_off(ui, "ตั้งค่าเมนูการเลือก(H)...");
    item_off(ui, "เมนูปรับเปลี่ยน(A)");
    item_off(ui, "เมนูกล่องข้อความ(K)");
    item_off(ui, "เมนูสแกนภาพ(3)");
}

fn ruler_and_grid_group(ui: &mut egui::Ui) {
    item_off(ui, "ตาราง(G)");
    item(ui, "แถบไม้บรรทัด(R)", "Ctrl+R");
    item_off(ui, "ตั้งค่าตาราง/แถบไม้บรรทัด(N)...");
    item_off(ui, "จุดตัดส่วนภาพ/ขอบเขตภายใน(M)");
    item_off(ui, "ตั้งค่าจุดตัดส่วนภาพ/ขอบเขตภายใน(T)...");
    item_off(ui, "ขอบปลอดภัย(4)");
    item_off(ui, "พื้นที่แสดงผล (webtoon)(5)");
    item_off(ui, "ตั้งค่าพื้นที่แสดงผล (webtoon)(6)...");
    item_off(ui, "กระดาษ(Y)");
    sub_off(ui, "แสดงเส้นไกด์(J)");
    sub_off(ui, "แสดงเส้นเวกเตอร์(4)");
}

fn fps_group(ui: &mut egui::Ui) {
    item_off(ui, "แสดง fps ของการเล่น");
}

fn snapping_group(ui: &mut egui::Ui) {
    item(ui, "สแนปกับไม้บรรทัด(U)", "Ctrl+1");
    item(ui, "สแนปกับไม้บรรทัดพิเศษ(P)", "Ctrl+/");
    item(ui, "สแนปเข้าตาราง(D)", "Ctrl+;");
    item(ui, "เปิดใช้งานสแนปเข้าไม้บรรทัดพิเศษ(C)", "Ctrl+N");
}

fn color_profile_group(ui: &mut egui::Ui) {
    sub_off(ui, "โปรไฟล์สี(Q)");
}

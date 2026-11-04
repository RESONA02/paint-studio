// menu/misc.rs — เมนู อนิเมชัน / ฟิลเตอร์ / หน้าต่าง / ช่วยเหลือ และข้อความสถานะ
use crate::app::App;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn, menu_sub, sub_off};

pub(crate) fn animation(ui: &mut egui::Ui) {
    ui.menu_button("อนิเมชัน(A)", |ui| {
        ui.set_min_width(240.0);
        animation_tracks(ui);
        ui.separator();
        playback_group(ui);
        ui.separator();
        timeline_group(ui);
        ui.separator();
        camera_and_display_group(ui);
    });
}

fn animation_tracks(ui: &mut egui::Ui) {
    sub_off(ui, "เลเยอร์ภาพเคลื่อนไหว(Y)");
    item_off(ui, "เซลภาพเคลื่อนไหวใหม่(C)");
    sub_off(ui, "แก้ไขแทร็ก(E)");
    sub_off(ui, "โค้งภาพเคลื่อนไหว(F)");
}

fn playback_group(ui: &mut egui::Ui) {
    item_off(ui, "เล่น/หยุด(P)");
    sub_off(ui, "ตั้งค่าการเล่น(S)");
    sub_off(ui, "ข้ามเฟรม(R)");
}

fn timeline_group(ui: &mut egui::Ui) {
    sub_off(ui, "ป้ายกำกับ(A)");
    sub_off(ui, "ไทม์ไลน์(M)");
}

fn camera_and_display_group(ui: &mut egui::Ui) {
    sub_off(ui, "แสดงเซลภาพเคลื่อนไหว(O)");
    menu_sub(ui, true, "ออนเนียนสกิน(B)", |ui| {
        ui.set_min_width(200.0);
        item_off(ui, "เปิด/ปิด ออนเนียนสกิน");
        item_off(ui, "ตั้งค่าออนเนียนสกิน...");
    });
    item_off(ui, "แสดงกรอบกล้อง 2D");
    item_off(ui, "แสดงเส้นกึ่งกลางกล้อง(H)");
    item_off(ui, "แสดงไฟล์สัมพันธ์ทั้งหมด(W)");
}

pub(crate) fn filter(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("ฟิลเตอร์(I)", |ui| {
        ui.set_min_width(220.0);
        sub_off(ui, "เบลอ(B)");
        item_off(ui, "ปรับความคมชัด(S)");
        sub_off(ui, "เอฟเฟกต์(E)");
        ui.separator();
        if menu_btn(ui, true, "แปลง 2D LT (Line Art / Screentone)...", "").clicked() {
            app.lt_dialog_open = true;
            ui.close();
        }
    });
}

pub(crate) fn window(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("หน้าต่าง(W)", |ui| {
        ui.set_min_width(240.0);
        item(ui, "แถบเครื่องมือ(T)", "");
        item(ui, "คุณสมบัติเครื่องมือ(P)", "");
        item(ui, "เลเยอร์(L)", "");
        item(ui, "เนวิเกเตอร์(N)", "");
        item(ui, "วงล้อสี(C)", "");
        item(ui, "ประวัติ(H)", "");
        ui.separator();
        if menu_btn(ui, true, "จัดการหน้า (Page Manager)", "").clicked() {
            app.page_manager_open = true;
            ui.close();
        }
        if menu_btn(ui, true, "สคริปต์ / บทพูด (Story Editor)", "").clicked() {
            app.story_editor_open = true;
            ui.close();
        }
        if menu_btn(ui, true, "แปลง 2D LT", "").clicked() {
            app.lt_dialog_open = true;
            ui.close();
        }
        ui.separator();
        sub_off(ui, "จัดการพื้นที่ทำงาน(W)");
    });
}

pub(crate) fn help(ui: &mut egui::Ui) {
    ui.menu_button("ช่วยเหลือ(H)", |ui| {
        ui.set_min_width(220.0);
        item(ui, "คู่มือการใช้งาน(I)...", "");
        item(ui, "ตรวจสอบการอัปเดต(U)...", "");
        ui.separator();
        item(ui, "เกี่ยวกับ CLIP STUDIO PAINT...", "");
    });
}

pub(crate) fn status_text(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new("FPS: 60 | Warm Ink Studio")
                .size(11.0)
                .color(egui::Color32::from_rgb(142, 144, 153)),
        );
    });
}

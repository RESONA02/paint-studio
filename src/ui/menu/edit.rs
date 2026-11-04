// menu/edit.rs — เมนู แก้ไข(E) ตามโครงสร้าง Clip Studio Paint ภาษาไทย
use crate::app::App;
use crate::state::store::Action;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn, menu_sub, sub_off};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("แก้ไข(E)", |ui| {
        ui.set_min_width(260.0);
        history_group(ui, app);
        ui.separator();
        clipboard_group(ui, app);
        ui.separator();
        delete_fill_group(ui, app);
        ui.separator();
        ai_and_vector_group(ui);
        ui.separator();
        transform_and_toning_group(ui);
        ui.separator();
        canvas_group(ui, app);
        ui.separator();
        memory_group(ui);
        ui.separator();
        advanced_group(ui);
    });
}

fn history_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, app.hi > 0, "ย้อนกลับ(U)", "Ctrl+Z").clicked() {
        app.undo();
        ui.close();
    }
    if menu_btn(ui, app.hi < app.history_labels.len(), "ทำซ้ำ(R)", "Ctrl+Y").clicked() {
        app.redo();
        ui.close();
    }
}

fn clipboard_group(ui: &mut egui::Ui, app: &mut App) {
    let has_sel = app.selection.is_some();
    let is_floating = app.floating.is_some();
    if menu_btn(ui, has_sel || is_floating, "ตัด(T)", "Ctrl+X").clicked() {
        app.dispatch(Action::LiftSelection);
        ui.close();
    }
    if menu_btn(ui, has_sel, "คัดลอก(C)", "Ctrl+C").clicked() {
        app.copy_selection();
        ui.close();
    }
    item_off(ui, "คัดลอกเวกเตอร์เป็น SVG(K)");
    if menu_btn(ui, app.clipboard_buffer.is_some(), "วาง(P)", "Ctrl+V").clicked() {
        app.paste_clipboard();
        ui.close();
    }
    item(ui, "วางไปยังตำแหน่งที่แสดง(W)", "Ctrl+Shift+V");
}

fn delete_fill_group(ui: &mut egui::Ui, app: &mut App) {
    let has_sel = app.selection.is_some();
    let is_floating = app.floating.is_some();
    if menu_btn(ui, true, "ลบ(E)", "Del").clicked() {
        if is_floating {
            app.dispatch(Action::DeleteFloating);
        } else {
            app.clear_selection_or_layer();
        }
        ui.close();
    }
    if menu_btn(ui, has_sel, "ลบนอกจุดที่เลือก(O)", "Shift+Del").clicked() {
        ui.close();
    }
    item_off(ui, "ปรับความเรียบภาพสแกน(S)...");
    if menu_btn(ui, true, "เติมสี(F)", "Alt+Del").clicked() {
        app.fill_selection_or_layer();
        ui.close();
    }
    item_off(ui, "เติมสีขั้นสูง(I)...");
}

fn ai_and_vector_group(ui: &mut egui::Ui) {
    sub_off(ui, "คลีนิค (ตัวอย่างเทคโนโลยีใหม่)(P)");
    item_off(ui, "ดำช่วยแทรก(F)...");
    item_off(ui, "แปลงเป็นสีในการวาด(H)");
    item_off(ui, "แปลงเส้นต่อขอบเขตที่เลือก(G)...");
    item_off(ui, "แปลงตารางสร้างเป็นความตื้น(B)");
}

fn transform_and_toning_group(ui: &mut egui::Ui) {
    sub_off(ui, "บันทึกวัสดุ(J)");
    sub_off(ui, "การแก้ไขโทนสี(D)");
    menu_sub(ui, true, "ปรับเปลี่ยนรูปทรง(A)", |ui| {
        ui.set_min_width(200.0);
        item(ui, "สเกล / หมุน(S)...", "Ctrl+T");
        item(ui, "อิสระ (Free Transform)(D)...", "Ctrl+Shift+T");
        item_off(ui, "บิดโค้งตาข่าย (Mesh Transform)...");
        item_off(ui, "มุมมองทัศนียภาพ (Perspective)...");
    });
    sub_off(ui, "จัดฝั่ง/กระจาย(Q)");
}

fn canvas_group(ui: &mut egui::Ui, app: &mut App) {
    item_off(ui, "เปลี่ยนความละเอียดภาพ(L)...");
    item_off(ui, "เปลี่ยนขนาดแผ่นผ้าใบ(S)...");
    if menu_btn(ui, app.selection.is_some(), "ตัดส่วนภาพ(Z)", "").clicked() {
        ui.close();
    }
    menu_sub(ui, true, "หมุน/พลิกแผ่นผ้าใบ(N)", |ui| {
        ui.set_min_width(200.0);
        if menu_btn(ui, true, "หมุนตามเข็ม 90°", "").clicked() {
            app.rotation += 90.0;
            ui.close();
        }
        if menu_btn(ui, true, "หมุนทวนเข็ม 90°", "").clicked() {
            app.rotation -= 90.0;
            ui.close();
        }
        if menu_btn(ui, true, "หมุน 180°", "").clicked() {
            app.rotation += 180.0;
            ui.close();
        }
        if menu_btn(ui, true, "รีเซ็ตการหมุน", "").clicked() {
            app.rotation = 0.0;
            ui.close();
        }
    });
}

fn memory_group(ui: &mut egui::Ui) {
    sub_off(ui, "ล้างความจำ(M)");
}

fn advanced_group(ui: &mut egui::Ui) {
    item_off(ui, "ตั้งค่าน้ำหนักเส้น(X)...");
    item_off(ui, "ซ่อนหน้าต่างและเลือกสีหน้าจอ(Y)...");
}

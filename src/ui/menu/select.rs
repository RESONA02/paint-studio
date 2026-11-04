// menu/select.rs — เมนู เลือก(S) ตามโครงสร้าง Clip Studio Paint ภาษาไทย พร้อมฟีเจอร์ Floating Selection
use crate::app::App;
use crate::state::store::Action;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("เลือก(S)", |ui| {
        ui.set_min_width(260.0);
        basic_group(ui, app);
        ui.separator();
        floating_selection_group(ui, app);
        ui.separator();
        expand_shrink_group(ui);
        ui.separator();
        color_range_group(ui);
        ui.separator();
        vector_layer_group(ui);
        ui.separator();
        quick_mask_group(ui);
    });
}

fn basic_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "เลือกทั้งหมด(A)", "Ctrl+A").clicked() {
        app.select_all();
        ui.close();
    }
    if menu_btn(ui, app.selection.is_some(), "ยกเลิกการเลือก(D)", "Ctrl+D").clicked() {
        app.deselect();
        ui.close();
    }
    item(ui, "เลือกซ้ำ(R)", "Ctrl+Shift+D");
    if menu_btn(
        ui,
        app.selection.is_some(),
        "สลับพื้นที่ที่เลือก(I)",
        "Ctrl+Shift+I",
    )
    .clicked()
    {
        app.inverse_selection();
        ui.close();
    }
}

fn floating_selection_group(ui: &mut egui::Ui, app: &mut App) {
    let can_lift = app.selection.is_some() && app.floating.is_none();
    let is_floating = app.floating.is_some();
    if menu_btn(ui, can_lift, "หยิบส่วนที่เลือก (Lift)", "Ctrl+X / L").clicked() {
        app.dispatch(Action::LiftSelection);
        ui.close();
    }
    if menu_btn(ui, is_floating, "ยืนยันการวาง (Commit)", "Enter").clicked() {
        app.dispatch(Action::CommitFloating);
        ui.close();
    }
    if menu_btn(ui, is_floating, "ยกเลิกการลอย (Cancel)", "Esc").clicked() {
        app.dispatch(Action::CancelFloating);
        ui.close();
    }
    if menu_btn(ui, is_floating, "ลบเนื้อหาลอย (Delete)", "Del").clicked() {
        app.dispatch(Action::DeleteFloating);
        ui.close();
    }
}

fn expand_shrink_group(ui: &mut egui::Ui) {
    item_off(ui, "ขยายพื้นที่เลือก(E)...");
    item_off(ui, "ลดพื้นที่เลือก(U)...");
    item_off(ui, "เบลอขอบ(B)...");
}

fn color_range_group(ui: &mut egui::Ui) {
    item_off(ui, "เลือกขอบเขตสี(C)...");
}

fn vector_layer_group(ui: &mut egui::Ui) {
    item_off(ui, "เลือกเวกเตอร์ที่เชื่อมต่อกัน(V)");
    item_off(ui, "เลือกเลเยอร์ในบริเวณ(W)");
}

fn quick_mask_group(ui: &mut egui::Ui) {
    item_off(ui, "แมสค์ด่วน(Q)");
    item_off(ui, "แปลงเป็นเลเยอร์ทางการเลือก(T)");
    item_off(ui, "แปลงเลเยอร์ที่เลือกเป็นการเลือกขอบเขต(F)");
}

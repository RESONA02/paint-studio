// menu/layer.rs — เมนู เลเยอร์(L) ตามโครงสร้าง Clip Studio Paint ภาษาไทย
use crate::app::App;
use eframe::egui;

use super::super::widgets::{item, item_off, menu_btn, menu_sub, sub_off};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("เลเยอร์(L)", |ui| {
        ui.set_min_width(260.0);
        create_group(ui, app);
        ui.separator();
        mask_and_ruler_group(ui);
        ui.separator();
        merge_group(ui, app);
        ui.separator();
        order_and_convert_group(ui);
    });
}

fn create_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "เลเยอร์แรสเตอร์ใหม่(N)", "Ctrl+Shift+N").clicked() {
        app.add_raster_layer();
        ui.close();
    }
    menu_sub(ui, true, "เลเยอร์ใหม่(E)", |ui| {
        ui.set_min_width(200.0);
        item(ui, "เลเยอร์เวกเตอร์(V)...", "");
        item(ui, "เลเยอร์เติมสี(F)...", "");
        item(ui, "เลเยอร์เกรเดียนต์(G)...", "");
    });
    sub_off(ui, "เลเยอร์การแก้ไขใหม่(J)");
    item_off(ui, "โฟลเดอร์เลเยอร์ใหม่(O)");
    item(ui, "สำเนาโฟลเดอร์และเพิ่มเลเยอร์(F)", "Ctrl+G");
    item(ui, "แยกกลุ่มโฟลเดอร์เลเยอร์(S)", "Ctrl+Shift+G");
    if menu_btn(ui, true, "ทำสำเนา(D)", "").clicked() {
        app.duplicate_active_layer();
        ui.close();
    }
    let can_delete = app.layers.len() > 1 && !app.layers[app.active].is_paper;
    if menu_btn(ui, can_delete, "ลบเลเยอร์(T)", "").clicked() {
        app.delete_active_layer();
        ui.close();
    }
}

fn mask_and_ruler_group(ui: &mut egui::Ui) {
    sub_off(ui, "มาสก์เลเยอร์(M)");
    sub_off(ui, "ไม้บรรทัด/เฟรม(R)");
    sub_off(ui, "ม็อบบิ้งใส่เลเยอร์(X)");
    sub_off(ui, "ตั้งค่าเลเยอร์(P)");
    sub_off(ui, "เลือกจากเลเยอร์(Y)");
    item_off(ui, "ทำให้เป็นมาสก์เลเยอร์(Z)");
    item_off(ui, "แปลงเลเยอร์(H)...");
}

fn merge_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, app.active > 0, "ย้ายไปเลเยอร์ด้านล่าง(C)", "").clicked() {
        app.active -= 1;
        ui.close();
    }
    let can_merge_down = app.active > 0 && !app.layers[app.active].is_paper;
    if menu_btn(ui, can_merge_down, "รวมกับเลเยอร์ด้านล่าง(W)", "Ctrl+E").clicked() {
        app.merge_layer_down();
        ui.close();
    }
    item(ui, "รวมเลเยอร์ที่เลือก(G)", "Shift+Alt+E");
    if menu_btn(
        ui,
        app.layers.len() > 1,
        "รวมเลเยอร์ที่มองเห็น(V)",
        "Ctrl+Shift+E",
    )
    .clicked()
    {
        app.merge_visible_layers();
        ui.close();
    }
    if menu_btn(
        ui,
        app.layers.len() > 1,
        "รวมถึงที่มองเห็นสร้างเลเยอร์ใหม่(X)",
        "Ctrl+Shift+Alt+E",
    )
    .clicked()
    {
        app.merge_visible_to_new_layer();
        ui.close();
    }
    if menu_btn(ui, app.layers.len() > 1, "รวมเลเยอร์ทั้งหมด(F)", "").clicked() {
        app.merge_all_layers();
        ui.close();
    }
}

fn order_and_convert_group(ui: &mut egui::Ui) {
    sub_off(ui, "ลำดับเลเยอร์(A)");
    sub_off(ui, "เปลี่ยนประเภทเลเยอร์(I)");
}

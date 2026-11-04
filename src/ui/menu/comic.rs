// menu/comic.rs — เมนู คอมิก(C) : Page Manager, spreads, Story Editor และ 2D LT
use crate::app::pages::Binding;
use crate::app::App;
use crate::tools::Tool;
use eframe::egui;

use super::super::widgets::menu_btn;

pub(crate) fn show(ui: &mut egui::Ui, app: &mut App) {
    ui.menu_button("คอมิก(C)", |ui| {
        ui.set_min_width(300.0);
        pages_group(ui, app);
        ui.separator();
        spread_group(ui, app);
        ui.separator();
        story_group(ui, app);
        ui.separator();
        lt_group(ui, app);
        ui.separator();
        frame_group(ui, app);
    });
}

fn pages_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "จัดการหน้า (Page Manager)...", "").clicked() {
        app.page_manager_open = true;
        ui.close();
    }
    if menu_btn(ui, true, "เพิ่มหน้าใหม่", "Shift+Alt+N").clicked() {
        app.add_page();
        ui.close();
    }
    if menu_btn(ui, true, "ลบหน้าปัจจุบัน", "").clicked() {
        app.delete_page();
        ui.close();
    }
}

fn spread_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "รวมหน้านี้กับหน้าถัดไปเป็นหน้าคู่", "").clicked()
    {
        merge_with_next(app);
        ui.close();
    }
    if menu_btn(ui, true, "แยกหน้าคู่กลับเป็นหน้าเดี่ยว", "").clicked()
    {
        split_current(app);
        ui.close();
    }
    ui.menu_button("การเข้าเล่ม", |ui| {
        ui.set_min_width(200.0);
        binding_item(ui, app, Binding::LeftToRight, "ซ้าย → ขวา");
        binding_item(ui, app, Binding::RightToLeft, "ขวา → ซ้าย");
    });
    if menu_btn(ui, true, "จัดเลขหน้าใหม่ (Folio)", "").clicked() {
        app.renumber_folios();
        ui.close();
    }
}

fn binding_item(ui: &mut egui::Ui, app: &mut App, binding: Binding, label: &str) {
    if menu_btn(ui, app.binding == binding, label, "").clicked() {
        app.binding = binding;
        app.renumber_folios();
        ui.close();
    }
}

fn story_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "สคริปต์ / บทพูด (Story Editor)...", "").clicked()
    {
        app.story_editor_open = true;
        ui.close();
    }
    if menu_btn(ui, true, "แทนที่คำผิดทั้งเล่ม", "").clicked()
    {
        app.story_editor_open = true;
        ui.close();
    }
}

fn lt_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, true, "แปลง 2D LT (Line Art + Screentone)...", "").clicked() {
        app.lt_dialog_open = true;
        ui.close();
    }
    if menu_btn(ui, true, "ดึงเส้น Line Art ทันที", "").clicked() {
        app.apply_line_art();
        ui.close();
    }
    if menu_btn(ui, true, "สร้างสกรีนโทนทันที", "").clicked() {
        app.apply_screentone();
        ui.close();
    }
}

fn frame_group(ui: &mut egui::Ui, app: &mut App) {
    if menu_btn(ui, app.tool == Tool::ComicFrame, "เครื่องมือตัดกรอบช่อง", "").clicked()
    {
        app.set_tool(Tool::ComicFrame);
        ui.close();
    }
    if menu_btn(ui, app.tool == Tool::Balloon, "เครื่องมือบอลลูนคำพูด", "").clicked()
    {
        app.set_tool(Tool::Balloon);
        ui.close();
    }
    if menu_btn(ui, true, "แปลงกรอบ/บอลลูนเป็นภาพ", "").clicked()
    {
        app.rasterize_comic_items();
        ui.close();
    }
}

fn merge_with_next(app: &mut App) {
    app.page_marks.clear();
    app.page_marks.insert(app.page_index);
    if app.page_index + 1 < app.pages.len() {
        app.page_marks.insert(app.page_index + 1);
    }
    match app.merge_marked_pages() {
        Ok(()) => app.saved = "รวมหน้าเป็นหน้าคู่พาโนรามาแล้ว".into(),
        Err(e) => app.saved = e,
    }
}

fn split_current(app: &mut App) {
    match app.split_spread_page() {
        Ok(()) => app.saved = "แยกหน้าคู่กลับเป็นหน้าเดี่ยวแล้ว".into(),
        Err(e) => app.saved = e,
    }
}

// toolbar.rs — command bar (quick actions) and document tab bar
// สไตล์ "Warm Ink": ใช้อำพันเน้นสถานะ, ไม่ใช้สีฟ้า-เทาเดิม, โทนอุ่นคมชัด

use super::theme::Theme;
use crate::app::App;
use crate::state::store::Action;
use eframe::egui;

impl App {
    pub(crate) fn show_command_bar(&mut self, ctx: &egui::Context) {
        let theme = if self.is_dark_theme {
            Theme::dark()
        } else {
            Theme::light()
        };

        egui::TopBottomPanel::top("command_bar")
            .frame(
                egui::Frame::NONE
                    .fill(theme.bg_panel_header)
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(6.0, 0.0);
                    self.file_buttons(ui, &theme);
                    self.history_buttons(ui, &theme);
                    self.floating_buttons(ui, &theme);
                    self.edit_buttons(ui, &theme);
                    self.view_buttons(ui, &theme);
                    self.scale_and_settings_buttons(ui, &theme, ctx);
                    self.status_label(ui, &theme);
                });
            });
    }

    fn file_buttons(&mut self, ui: &mut egui::Ui, _theme: &Theme) {
        if ui.button("สร้างใหม่").clicked() {
            self.push_hist("สร้างภาพวาดใหม่");
        }
        if ui.button("บันทึก").clicked() {
            match self.save_png() {
                Ok(_) => {
                    self.saved = "บันทึกเรียบร้อย".into();
                    self.push_hist("บันทึกภาพวาด");
                }
                Err(e) => {
                    self.saved = e;
                }
            }
        }
        if ui.button("Production").clicked() {
            self.production_panel_open = !self.production_panel_open;
        }
        ui.separator();
    }

    fn history_buttons(&mut self, ui: &mut egui::Ui, _theme: &Theme) {
        if ui.button("เลิกทำ").on_hover_text("Ctrl+Z").clicked() {
            self.undo();
        }
        if ui.button("ทำซ้ำ").on_hover_text("Ctrl+Y").clicked() {
            self.redo();
        }
        ui.separator();
    }

    fn floating_buttons(&mut self, ui: &mut egui::Ui, _theme: &Theme) {
        let can_lift = self.selection.is_some() && self.floating.is_none();
        if ui
            .add_enabled(can_lift, egui::Button::new("หยิบ (Lift)"))
            .on_hover_text("Ctrl+X หรือ L")
            .clicked()
        {
            self.dispatch(Action::LiftSelection);
        }
        if self.floating.is_some() {
            self.floating_action_buttons(ui);
        }
    }

    fn floating_action_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("ยืนยัน (Commit)").on_hover_text("Enter").clicked() {
            self.dispatch(Action::CommitFloating);
        }
        if ui.button("ยกเลิก (Cancel)").on_hover_text("Esc").clicked() {
            self.dispatch(Action::CancelFloating);
        }
        if ui.button("ลบ (Delete)").on_hover_text("Delete").clicked() {
            self.dispatch(Action::DeleteFloating);
        }
        ui.separator();
    }

    fn edit_buttons(&mut self, ui: &mut egui::Ui, _theme: &Theme) {
        if ui.button("ล้างเลเยอร์").clicked() {
            self.clear_active_layer();
        }
        if ui.button("ตัด").clicked() {
            self.push_hist("ตัด");
        }
        if ui.button("คัดลอก").clicked() {
            self.push_hist("คัดลอก");
        }
        if ui.button("วาง").clicked() {
            self.push_hist("วาง");
        }
        ui.separator();
    }

    fn view_buttons(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        if ui.button("พอดีหน้าจอ").clicked() {
            self.zoom = 1.0;
            self.pan_offset = egui::Vec2::ZERO;
        }
        let sym_btn = egui::Button::new("ไม้บรรทัดสมมาตร").selected(self.symmetry_enabled);
        if ui
            .add(sym_btn)
            .on_hover_text("เปิด/ปิดการวาดสะท้อนสมมาตร (Shift+P)")
            .clicked()
        {
            self.symmetry_enabled = !self.symmetry_enabled;
            self.texture_dirty = true;
        }
        if self.symmetry_enabled {
            ui.label(egui::RichText::new("●").size(10.0).color(theme.accent));
        }
        ui.separator();
    }

    fn scale_and_settings_buttons(
        &mut self,
        ui: &mut egui::Ui,
        _theme: &Theme,
        ctx: &egui::Context,
    ) {
        // ปุ่มสลับแสดงชื่อเครื่องมือใต้ไอคอน
        let label_btn = egui::Button::new("แสดงชื่อ").selected(self.show_tool_names);
        if ui
            .add(label_btn)
            .on_hover_text("แสดง/ซ่อนป้ายชื่อใต้ไอคอนเครื่องมือ")
            .clicked()
        {
            self.show_tool_names = !self.show_tool_names;
        }

        // ปุ่มสลับ UI Scale: 100%, 125%, 150%
        let scale_text = format!("{:.0}%", self.ui_scale * 100.0);
        egui::ComboBox::from_id_salt("ui_scale_selector")
            .selected_text(scale_text)
            .width(60.0)
            .show_ui(ui, |ui| {
                for s in [1.0, 1.25, 1.5] {
                    if ui
                        .selectable_value(&mut self.ui_scale, s, format!("{:.0}%", s * 100.0))
                        .clicked()
                    {
                        ctx.set_pixels_per_point(self.ui_scale);
                    }
                }
            });

        // ปุ่มสลับโหมดมืด/สว่าง
        let mode_icon = if self.is_dark_theme { "🌙" } else { "☀️" };
        if ui
            .button(mode_icon)
            .on_hover_text("สลับโหมด Dark / Light")
            .clicked()
        {
            self.is_dark_theme = !self.is_dark_theme;
        }

        ui.separator();
    }

    fn status_label(&self, ui: &mut egui::Ui, theme: &Theme) {
        if !self.saved.is_empty() {
            ui.label(
                egui::RichText::new(&self.saved)
                    .size(11.0)
                    .color(theme.accent),
            );
        }
    }

    pub(crate) fn show_document_tabs(&mut self, ctx: &egui::Context) {
        let theme = if self.is_dark_theme {
            Theme::dark()
        } else {
            Theme::light()
        };

        egui::TopBottomPanel::top("doc_tab_bar")
            .frame(
                egui::Frame::NONE
                    .fill(theme.bg_app)
                    .inner_margin(egui::Margin::symmetric(8, 2)),
            )
            .show(ctx, |ui| {
                let title = format!("ภาพวาด · หน้า {}", self.page_index + 1);
                ui.horizontal(|ui| document_tab(ui, &title, &theme));
            });
    }
}

fn document_tab(ui: &mut egui::Ui, title: &str, theme: &Theme) {
    let tab_rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(148.0, 24.0));
    let rounding = egui::CornerRadius {
        nw: 6,
        ne: 6,
        sw: 0,
        se: 0,
    };

    let p = ui.painter();
    p.rect_filled(tab_rect, rounding, theme.bg_panel);
    p.rect_stroke(
        tab_rect,
        rounding,
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );

    // เส้นขีดอำพันบางด้านบนบอกสถานะ active tab
    p.line_segment(
        [
            tab_rect.left_top() + egui::vec2(4.0, 1.0),
            tab_rect.right_top() - egui::vec2(4.0, -1.0),
        ],
        egui::Stroke::new(2.0, theme.accent),
    );

    ui.scope_builder(egui::UiBuilder::new().max_rect(tab_rect), |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(title)
                    .size(12.0)
                    .strong()
                    .color(theme.text_primary),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(6.0);
                let _ = ui.small_button("×");
            });
        });
    });
}

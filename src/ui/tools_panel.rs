// tools_panel.rs — แถบเครื่องมือแนวตั้ง "Warm Ink" (Floating 48px Tool Column)
// ออกแบบตาม Anti-AI-Slop Standard: Card ลอย 10px radius, Animated Pill 150ms, Subtool Popup

use super::theme::{draw_warm_ink_logo, Theme};
use super::warm_ink_icons::WarmToolIcon;
use super::widgets::ToolButton;
use crate::app::App;
use crate::tools::{SubToolDef, Tool};
use eframe::egui;

impl App {
    fn subtools_for_active_tool(&self) -> Vec<SubToolDef> {
        self.tool.subtools()
    }

    fn apply_subtool_preset(&mut self, def: SubToolDef) {
        self.select_subtool(def);
    }

    /// แสดงแถบเครื่องมือลอยด้านซ้าย กว้าง 48 px (Floating Left Card)
    pub(crate) fn show_tool_strip(&mut self, ctx: &egui::Context) {
        let theme = if self.is_dark_theme {
            Theme::dark()
        } else {
            Theme::light()
        };

        egui::SidePanel::left("warm_ink_tool_palette")
            .exact_width(48.0)
            .frame(
                egui::Frame::NONE
                    .fill(theme.bg_app)
                    .inner_margin(egui::Margin::symmetric(4, 6)),
            )
            .show(ctx, |ui| {
                // การ์ดลอยมุมโค้ง 10 px
                let avail_h = ui.available_height();
                let card_rect = egui::Rect::from_min_size(
                    ui.cursor().min + egui::vec2(0.0, 2.0),
                    egui::vec2(40.0, avail_h - 4.0),
                );

                let p = ui.painter().clone();
                p.rect_filled(
                    card_rect,
                    egui::CornerRadius::same(theme.radius_panel as u8),
                    theme.bg_panel,
                );
                p.rect_stroke(
                    card_rect,
                    egui::CornerRadius::same(theme.radius_panel as u8),
                    egui::Stroke::new(1.0, theme.border),
                    egui::StrokeKind::Inside,
                );

                ui.scope_builder(
                    egui::UiBuilder::new().max_rect(card_rect.shrink(3.0)),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(4.0);
                            // 1. ปุ่มโลโก้ Monoline Warm Ink ด้านบนสุด (คลิกเพื่อเปิดหน้า Start Screen Hub)
                            self.brand_logo_button(ui, &theme);

                            ui.add_space(6.0);
                            p.line_segment(
                                [
                                    card_rect.min + egui::vec2(6.0, 36.0),
                                    card_rect.min + egui::vec2(34.0, 36.0),
                                ],
                                egui::Stroke::new(1.0, theme.border),
                            );
                            ui.add_space(4.0);

                            // 2. รายการปุ่มเครื่องมือพร้อม Animated Pill
                            self.warm_tool_buttons(ui, &theme);

                            ui.add_space(8.0);

                            // 3. Dual Color Swatches
                            self.warm_color_swatches(ui, &theme);
                        });
                    },
                );
            });

        // จัดการเปิด Popup เครื่องมือย่อยเมื่อคลิกขวาหรือคลิกค้าง
        self.show_subtool_popup(ctx, &theme);
    }

    /// ปุ่มโลโก้แบรนด์ Warm Ink ที่มุมบน
    fn brand_logo_button(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(32.0, 28.0), egui::Sense::click());
        let p = ui.painter();

        let logo_col = if resp.hovered() {
            theme.accent_hover
        } else {
            theme.accent
        };
        draw_warm_ink_logo(p, rect.center(), 22.0, logo_col);

        if resp
            .on_hover_text("Warm Ink Hub — หน้าเริ่มต้น (Start Screen)")
            .clicked()
        {
            self.show_start_screen = !self.show_start_screen;
        }
    }

    /// วาดปุ่มเครื่องมือทั้ง 12 ตัว พร้อม Animated Pill
    fn warm_tool_buttons(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let btn_h = if self.show_tool_names { 46.0 } else { 38.0 };
        let stride = btn_h + 3.0;

        // คำนวณ Animated Pill Y Position ด้วย animate_value_with_time (150 ms ease-out)
        let active_index = self.active_warm_tool as f32;
        let start_y = ui.cursor().min.y;
        let animated_y = ui.ctx().animate_value_with_time(
            ui.id().with("warm_pill_y"),
            start_y + (active_index * stride),
            0.150, // 150 ms
        );

        // วาด Pill อำพันรองรับด้านหลัง
        let pill_rect = egui::Rect::from_min_size(
            egui::pos2(ui.cursor().min.x, animated_y),
            egui::vec2(34.0, btn_h),
        );
        ui.painter().rect_filled(
            pill_rect,
            egui::CornerRadius::same(theme.radius_button as u8),
            theme.accent,
        );

        for (idx, &icon) in WarmToolIcon::ALL.iter().enumerate() {
            let is_active = self.active_warm_tool == idx;
            let has_sub = matches!(
                icon,
                WarmToolIcon::Brush
                    | WarmToolIcon::InkPen
                    | WarmToolIcon::Eraser
                    | WarmToolIcon::FillBucket
                    | WarmToolIcon::RectSelect
            );

            let btn = ToolButton::new(icon, is_active)
                .with_subtools(has_sub)
                .show_label(self.show_tool_names);

            let resp = btn.ui(ui, theme);

            // คลิกซ้ายเลือกเครื่องมือ
            if resp.clicked() {
                self.active_warm_tool = idx;
                self.select_warm_tool(icon);
            }

            // คลิกขวาเพื่อเปิด Popup เครื่องมือย่อย
            if has_sub && resp.secondary_clicked() {
                self.subtool_popup_tool = Some(idx);
            }

            ui.add_space(3.0);
        }
    }

    /// สลับการเลือกเครื่องมือภายในระบบหลัก
    fn select_warm_tool(&mut self, icon: WarmToolIcon) {
        match icon {
            WarmToolIcon::Brush => self.set_tool(Tool::Brush),
            WarmToolIcon::InkPen => self.set_tool(Tool::Pen),
            WarmToolIcon::Eraser => self.set_tool(Tool::Eraser),
            WarmToolIcon::FillBucket => self.set_tool(Tool::Fill),
            WarmToolIcon::RectSelect => self.set_tool(Tool::Lasso),
            WarmToolIcon::FreeSelect => self.set_tool(Tool::Lasso),
            WarmToolIcon::Move => self.set_tool(Tool::Move),
            WarmToolIcon::Eyedropper => self.set_tool(Tool::EyeDropper),
            WarmToolIcon::Text => self.set_tool(Tool::Text),
            WarmToolIcon::Zoom => self.set_tool(Tool::Zoom),
            WarmToolIcon::Hand => self.set_tool(Tool::Hand),
            WarmToolIcon::Layers => {
                self.hide_side_panels = !self.hide_side_panels;
            }
        }
    }

    /// Popup เลือกเครื่องมือย่อย (Subtool Popup 10px rounded corners, subtle shadow)
    fn show_subtool_popup(&mut self, ctx: &egui::Context, theme: &Theme) {
        let Some(tool_idx) = self.subtool_popup_tool else {
            return;
        };

        let subtools: Vec<SubToolDef> = self.subtools_for_active_tool();
        if subtools.is_empty() {
            self.subtool_popup_tool = None;
            return;
        }

        let mut close_popup = false;
        let popup_pos = egui::pos2(56.0, 50.0 + (tool_idx as f32 * 38.0).min(450.0));

        egui::Area::new(egui::Id::new("subtool_popup_area"))
            .order(egui::Order::Foreground)
            .fixed_pos(popup_pos)
            .show(ctx, |ui| {
                let card_w = 180.0;
                let card_h = (subtools.len() as f32 * 32.0) + 16.0;
                let rect = egui::Rect::from_min_size(popup_pos, egui::vec2(card_w, card_h));
                let p = ui.painter();

                // เงาบางๆ 1 ชั้น
                p.rect_filled(
                    rect.translate(egui::vec2(2.0, 2.0)),
                    egui::CornerRadius::same(theme.radius_panel as u8),
                    egui::Color32::from_rgba_unmultiplied(0, 0, 0, 45),
                );
                p.rect_filled(
                    rect,
                    egui::CornerRadius::same(theme.radius_panel as u8),
                    theme.bg_panel,
                );
                p.rect_stroke(
                    rect,
                    egui::CornerRadius::same(theme.radius_panel as u8),
                    egui::Stroke::new(1.0, theme.border),
                    egui::StrokeKind::Inside,
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(6.0)), |ui| {
                    ui.vertical(|ui| {
                        for def in &subtools {
                            let is_curr = self.subtool == def.subtool;
                            let (row_rect, row_resp) = ui.allocate_exact_size(
                                egui::vec2(card_w - 12.0, 26.0),
                                egui::Sense::click(),
                            );
                            let rp = ui.painter();

                            if is_curr {
                                rp.rect_filled(
                                    row_rect,
                                    egui::CornerRadius::same(theme.radius_input as u8),
                                    theme.bg_layer_active,
                                );
                                rp.rect_filled(
                                    egui::Rect::from_min_size(row_rect.min, egui::vec2(3.0, 26.0)),
                                    egui::CornerRadius::ZERO,
                                    theme.accent,
                                );
                            } else if row_resp.hovered() {
                                rp.rect_filled(
                                    row_rect,
                                    egui::CornerRadius::same(theme.radius_input as u8),
                                    theme.bg_panel_header,
                                );
                            }

                            rp.text(
                                row_rect.min + egui::vec2(10.0, 13.0),
                                egui::Align2::LEFT_CENTER,
                                def.name_th,
                                egui::FontId::proportional(theme.font_size_body),
                                if is_curr {
                                    theme.accent
                                } else {
                                    theme.text_primary
                                },
                            );

                            if row_resp.clicked() {
                                self.apply_subtool_preset(*def);
                                close_popup = true;
                            }
                        }
                    });
                });
            });

        if close_popup || ctx.input(|i| i.pointer.any_pressed() && !ctx.wants_pointer_input()) {
            self.subtool_popup_tool = None;
        }
    }

    /// กล่องเลือกสีคู่แบบ Warm Ink (Dual Color Swatches)
    fn warm_color_swatches(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(34.0, 42.0), egui::Sense::click());
        let p = ui.painter();

        // สี่เหลี่ยมสีรอง (Sub Color)
        let sub_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(10.0, 14.0), egui::vec2(22.0, 22.0));
        p.rect_filled(sub_rect, egui::CornerRadius::same(4), self.sub_color);
        p.rect_stroke(
            sub_rect,
            egui::CornerRadius::same(4),
            egui::Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        // สี่เหลี่ยมสีหลัก (Main Color)
        let main_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(2.0, 2.0), egui::vec2(22.0, 22.0));
        p.rect_filled(main_rect, egui::CornerRadius::same(4), self.main_color);
        p.rect_stroke(
            main_rect,
            egui::CornerRadius::same(4),
            egui::Stroke::new(
                1.2,
                if self.is_transparent {
                    theme.border
                } else {
                    theme.accent
                },
            ),
            egui::StrokeKind::Inside,
        );

        if resp.on_hover_text("สลับสีหลักและสีรอง (คลิก)").clicked()
        {
            std::mem::swap(&mut self.main_color, &mut self.sub_color);
        }

        // ปุ่มสลับโปร่งใส (Transparent chip)
        ui.add_space(2.0);
        let (tr_rect, tr_resp) =
            ui.allocate_exact_size(egui::vec2(34.0, 16.0), egui::Sense::click());
        let tp = ui.painter();
        let tr_bg = if self.is_transparent {
            theme.accent
        } else {
            theme.bg_panel_header
        };
        tp.rect_filled(tr_rect, egui::CornerRadius::same(3), tr_bg);
        tp.rect_stroke(
            tr_rect,
            egui::CornerRadius::same(3),
            egui::Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
        tp.text(
            tr_rect.center(),
            egui::Align2::CENTER_CENTER,
            "CLR",
            egui::FontId::monospace(8.5),
            if self.is_transparent {
                theme.bg_panel
            } else {
                theme.text_secondary
            },
        );
        if tr_resp.on_hover_text("โหมดสีใส / ยางลบ (C)").clicked() {
            self.is_transparent = !self.is_transparent;
        }
    }
}

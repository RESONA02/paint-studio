// widgets.rs — คอมโพเนนต์ UI สไตล์ "Warm Ink" (Anti-AI-Slop Standard)
// ประกอบด้วย ToolButton, PanelHeader, LayerRow, LabeledSlider, Chip, Tooltip

use super::theme::Theme;
use super::warm_ink_icons::{draw_icon_widget, IconState, WarmToolIcon};
use eframe::egui;

// --- เมนูเฮลเปอร์เดิม ---

pub(crate) fn menu_btn(
    ui: &mut egui::Ui,
    enabled: bool,
    label: &str,
    shortcut: &str,
) -> egui::Response {
    let mut btn = egui::Button::new(label);
    if !shortcut.is_empty() {
        btn = btn.shortcut_text(shortcut);
    }
    ui.add_enabled(enabled, btn)
}

pub(crate) fn menu_sub(
    ui: &mut egui::Ui,
    enabled: bool,
    label: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    if enabled {
        ui.menu_button(label, add_contents);
    } else {
        let btn = egui::Button::new(label).shortcut_text("▶");
        let _ = ui.add_enabled(false, btn);
    }
}

pub(crate) fn item(ui: &mut egui::Ui, label: &str, shortcut: &str) {
    if menu_btn(ui, true, label, shortcut).clicked() {
        ui.close();
    }
}

pub(crate) fn item_off(ui: &mut egui::Ui, label: &str) {
    if menu_btn(ui, false, label, "").clicked() {
        ui.close();
    }
}

pub(crate) fn sub_off(ui: &mut egui::Ui, label: &str) {
    menu_sub(ui, false, label, |_| {});
}

// =========================================================================
// Warm Ink Custom Production-Grade Components
// =========================================================================

/// 1. ToolButton: ปุ่มเครื่องมือที่มีไอคอนเวกเตอร์, Pill ไฮไลต์, มุมสามเหลี่ยมย่อย และ Tooltip หน่วงเวลา
pub struct ToolButton<'a> {
    icon: WarmToolIcon,
    is_active: bool,
    is_disabled: bool,
    has_subtools: bool,
    show_label: bool,
    disabled_reason: Option<&'a str>,
}

impl<'a> ToolButton<'a> {
    pub fn new(icon: WarmToolIcon, is_active: bool) -> Self {
        Self {
            icon,
            is_active,
            is_disabled: false,
            has_subtools: false,
            show_label: false,
            disabled_reason: None,
        }
    }

    #[allow(dead_code)]
    pub fn disabled(mut self, disabled: bool, reason: Option<&'a str>) -> Self {
        self.is_disabled = disabled;
        self.disabled_reason = reason;
        self
    }

    pub fn with_subtools(mut self, has: bool) -> Self {
        self.has_subtools = has;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn ui(self, ui: &mut egui::Ui, theme: &Theme) -> egui::Response {
        let btn_w = 40.0;
        let btn_h = if self.show_label { 46.0 } else { 38.0 };
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(btn_w, btn_h), egui::Sense::click());

        let state = if self.is_disabled {
            IconState::Disabled
        } else if self.is_active {
            IconState::Active
        } else if resp.hovered() {
            IconState::Hovered
        } else {
            IconState::Normal
        };

        let p = ui.painter();

        // พื้นหลังปุ่มปกติ
        if state == IconState::Normal {
            p.rect_filled(
                rect,
                egui::CornerRadius::same(theme.radius_button as u8),
                theme.bg_panel,
            );
        }

        // วาดไอคอน
        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(rect.center().x, rect.min.y + 19.0),
            egui::vec2(28.0, 28.0),
        );
        draw_icon_widget(p, icon_rect, self.icon, state, theme);

        // มุมสามเหลี่ยมเล็ก 4 px ด้านขวาล่างเมื่อมีเครื่องมือย่อย (Subtool marker)
        if self.has_subtools {
            let tr_col = if self.is_active {
                theme.bg_panel
            } else {
                theme.accent
            };
            let br = rect.max - egui::vec2(3.0, 3.0);
            p.add(egui::Shape::convex_polygon(
                vec![br, br - egui::vec2(5.0, 0.0), br - egui::vec2(0.0, 5.0)],
                tr_col,
                egui::Stroke::NONE,
            ));
        }

        // ป้ายชื่อเล็กใต้ไอคอน (เมื่อเปิดโหมดแสดงชื่อ)
        if self.show_label {
            let label_col = if self.is_active {
                theme.accent
            } else {
                theme.text_secondary
            };
            p.text(
                egui::pos2(rect.center().x, rect.max.y - 7.0),
                egui::Align2::CENTER_CENTER,
                self.icon.name_th(),
                egui::FontId::proportional(9.5),
                label_col,
            );
        }

        // Tooltip อธิบายเครื่องมือ (แสดงชื่อ, คีย์ลัด, และคำอธิบาย 1 บรรทัด)
        if resp.hovered() {
            show_custom_tooltip(
                ui,
                &resp,
                theme,
                self.icon.name_th(),
                self.icon.shortcut(),
                self.icon.description(),
                self.disabled_reason,
            );
        }

        resp
    }
}

/// 2. PanelHeader: ส่วนหัวการ์ดลอยสูง 32 px พร้อมชื่อด้านซ้าย ปุ่มพับและเมนูด้านขวา
pub fn draw_panel_header(
    ui: &mut egui::Ui,
    theme: &Theme,
    title: &str,
    is_folded: &mut bool,
) -> egui::Response {
    let avail_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(avail_w, 32.0), egui::Sense::hover());
    let p = ui.painter();

    // พื้นหลังหัวการ์ด (Top rounded corners)
    let corner = egui::CornerRadius {
        nw: theme.radius_panel as u8,
        ne: theme.radius_panel as u8,
        sw: if *is_folded {
            theme.radius_panel as u8
        } else {
            0
        },
        se: if *is_folded {
            theme.radius_panel as u8
        } else {
            0
        },
    };
    p.rect_filled(rect, corner, theme.bg_panel_header);
    p.line_segment(
        [
            egui::pos2(rect.min.x, rect.max.y),
            egui::pos2(rect.max.x, rect.max.y),
        ],
        egui::Stroke::new(1.0, theme.border),
    );

    // ชื่อพาเนลด้านซ้าย
    p.text(
        egui::pos2(rect.min.x + theme.space_12, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(theme.font_size_header),
        theme.text_primary,
    );

    // ปุ่มพับ/กาง (Fold/Unfold icon button) ด้านขวา
    let fold_rect = egui::Rect::from_center_size(
        egui::pos2(rect.max.x - 18.0, rect.center().y),
        egui::vec2(20.0, 20.0),
    );
    let fold_resp = ui.interact(fold_rect, ui.id().with("fold_btn"), egui::Sense::click());
    if fold_resp.clicked() {
        *is_folded = !*is_folded;
    }

    let arrow_col = if fold_resp.hovered() {
        theme.accent_hover
    } else {
        theme.text_secondary
    };
    let fc = fold_rect.center();
    if *is_folded {
        // ลูกศรชี้ลง ▽
        p.add(egui::Shape::convex_polygon(
            vec![
                fc + egui::vec2(-4.0, -2.5),
                fc + egui::vec2(4.0, -2.5),
                fc + egui::vec2(0.0, 3.0),
            ],
            arrow_col,
            egui::Stroke::NONE,
        ));
    } else {
        // ลูกศรชี้ขึ้น △
        p.add(egui::Shape::convex_polygon(
            vec![
                fc + egui::vec2(-4.0, 2.5),
                fc + egui::vec2(4.0, 2.5),
                fc + egui::vec2(0.0, -3.0),
            ],
            arrow_col,
            egui::Stroke::NONE,
        ));
    }

    resp
}

/// 3. LayerRow: แถวเลเยอร์ thumbnail 40x40, แถบอำพัน 3px ด้านซ้ายเมื่อ active, หลอด opacity
pub fn draw_layer_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    name: &str,
    is_active: bool,
    opacity_percent: u8,
    visible: &mut bool,
    locked: &mut bool,
) -> bool {
    let avail_w = ui.available_width();
    let row_h = 44.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(avail_w, row_h), egui::Sense::click());
    let p = ui.painter();

    // พื้นหลังแถวเลเยอร์
    let bg_col = if is_active {
        theme.bg_layer_active
    } else if resp.hovered() {
        theme.bg_panel_header
    } else {
        theme.bg_panel
    };
    p.rect_filled(
        rect,
        egui::CornerRadius::same(theme.radius_input as u8),
        bg_col,
    );

    // แถบสีอำพัน 3px ที่ขอบซ้ายเมื่อเป็น Active Layer
    if is_active {
        let bar_rect = egui::Rect::from_min_size(rect.min, egui::vec2(3.0, row_h));
        p.rect_filled(bar_rect, egui::CornerRadius::ZERO, theme.accent);
    }

    // Thumbnail จำลอง 34x34 px
    let thumb_rect =
        egui::Rect::from_min_size(rect.min + egui::vec2(8.0, 5.0), egui::vec2(34.0, 34.0));
    p.rect_filled(
        thumb_rect,
        egui::CornerRadius::same(4),
        egui::Color32::from_rgb(0x16, 0x17, 0x1B),
    );
    p.rect_stroke(
        thumb_rect,
        egui::CornerRadius::same(4),
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );
    p.circle_filled(
        thumb_rect.center(),
        5.0,
        egui::Color32::from_rgba_unmultiplied(242, 169, 59, 120),
    );

    // ชื่อเลเยอร์
    p.text(
        rect.min + egui::vec2(48.0, 14.0),
        egui::Align2::LEFT_CENTER,
        name,
        egui::FontId::proportional(theme.font_size_body),
        if is_active {
            theme.accent
        } else {
            theme.text_primary
        },
    );

    // แถบบางแสดง Opacity
    let bar_w = 60.0;
    let op_bar_rect =
        egui::Rect::from_min_size(rect.min + egui::vec2(48.0, 28.0), egui::vec2(bar_w, 4.0));
    p.rect_filled(op_bar_rect, egui::CornerRadius::same(2), theme.border);
    let fill_w = (bar_w * (opacity_percent as f32 / 100.0)).clamp(0.0, bar_w);
    p.rect_filled(
        egui::Rect::from_min_size(op_bar_rect.min, egui::vec2(fill_w, 4.0)),
        egui::CornerRadius::same(2),
        theme.accent,
    );

    // ตัวเลขเปอร์เซ็นต์แบบ Tabular font
    p.text(
        rect.min + egui::vec2(114.0, 30.0),
        egui::Align2::LEFT_CENTER,
        format!("{}%", opacity_percent),
        egui::FontId::monospace(10.0),
        theme.text_secondary,
    );

    // ไอคอนตา (เปิด/ปิดการมองเห็น)
    let eye_rect = egui::Rect::from_center_size(
        egui::pos2(rect.max.x - 36.0, rect.center().y),
        egui::vec2(20.0, 20.0),
    );
    let eye_resp = ui.interact(
        eye_rect,
        ui.id().with(format!("eye_{}", name)),
        egui::Sense::click(),
    );
    if eye_resp.clicked() {
        *visible = !*visible;
    }
    let eye_col = if *visible {
        theme.text_primary
    } else {
        theme.text_disabled
    };
    p.circle_filled(eye_rect.center(), 3.5, eye_col);
    p.circle_stroke(eye_rect.center(), 6.5, egui::Stroke::new(1.2, eye_col));

    // ไอคอนกุญแจ (ล็อกเลเยอร์)
    let lock_rect = egui::Rect::from_center_size(
        egui::pos2(rect.max.x - 14.0, rect.center().y),
        egui::vec2(20.0, 20.0),
    );
    let lock_resp = ui.interact(
        lock_rect,
        ui.id().with(format!("lock_{}", name)),
        egui::Sense::click(),
    );
    if lock_resp.clicked() {
        *locked = !*locked;
    }
    let lock_col = if *locked {
        theme.accent
    } else {
        theme.text_disabled
    };
    p.rect_stroke(
        egui::Rect::from_center_size(
            lock_rect.center() + egui::vec2(0.0, 1.5),
            egui::vec2(8.0, 7.0),
        ),
        egui::CornerRadius::same(1),
        egui::Stroke::new(1.2, lock_col),
        egui::StrokeKind::Inside,
    );

    resp.clicked()
}

/// 4. LabeledSlider: สไลเดอร์พร้อมช่องพิมพ์ตัวเลข Tabular และกด Shift ค้างเพื่อปรับละเอียด
pub fn draw_labeled_slider(
    ui: &mut egui::Ui,
    theme: &Theme,
    label: &str,
    val: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    unit: &str,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .size(theme.font_size_body)
                .color(theme.text_secondary),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // ช่องป้อนตัวเลข Tabular
            let mut num_val = *val;
            if ui
                .add(
                    egui::DragValue::new(&mut num_val)
                        .range(range.clone())
                        .speed(if ui.input(|i| i.modifiers.shift) {
                            0.1
                        } else {
                            1.0
                        })
                        .suffix(unit),
                )
                .changed()
            {
                *val = num_val;
                changed = true;
            }

            // ตัวสไลเดอร์
            let slider_w = (ui.available_width() - 8.0).max(80.0);
            if ui
                .add_sized(
                    egui::vec2(slider_w, 18.0),
                    egui::Slider::new(val, range).show_value(false),
                )
                .changed()
            {
                changed = true;
            }
        });
    });
    changed
}

/// 5. Chip: ป้ายสถานะ/ตัวเลือกโค้งมนนุ่มนวล
#[allow(dead_code)]
pub fn draw_chip(ui: &mut egui::Ui, theme: &Theme, text: &str, selected: bool) -> egui::Response {
    let padding = egui::vec2(theme.space_8, theme.space_4);
    let galley = ui.painter().layout_no_wrap(
        text.to_string(),
        egui::FontId::proportional(theme.font_size_body - 1.0),
        if selected {
            theme.bg_panel
        } else {
            theme.text_primary
        },
    );
    let size = galley.size() + padding * 2.0;
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    let p = ui.painter();
    let bg = if selected {
        theme.accent
    } else if resp.hovered() {
        theme.bg_panel_header
    } else {
        theme.bg_panel
    };
    p.rect_filled(
        rect,
        egui::CornerRadius::same(theme.radius_button as u8),
        bg,
    );
    p.rect_stroke(
        rect,
        egui::CornerRadius::same(theme.radius_button as u8),
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );
    p.galley(
        rect.min + padding,
        galley,
        if selected {
            theme.bg_panel
        } else {
            theme.text_primary
        },
    );

    resp
}

/// 6. Tooltip พิเศษหน่วงเวลา พร้อม Shortcut และคำอธิบาย
fn show_custom_tooltip(
    _ui: &egui::Ui,
    resp: &egui::Response,
    theme: &Theme,
    title: &str,
    shortcut: &str,
    desc: &str,
    disabled_reason: Option<&str>,
) {
    resp.clone().on_hover_ui(|ui| {
        ui.set_max_width(200.0);
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .size(theme.font_size_body)
                    .color(theme.text_primary),
            );
            if !shortcut.is_empty() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("[{}]", shortcut))
                            .monospace()
                            .size(11.0)
                            .color(theme.accent),
                    );
                });
            }
        });
        ui.add_space(2.0);
        ui.label(
            egui::RichText::new(desc)
                .size(11.5)
                .color(theme.text_secondary),
        );
        if let Some(reason) = disabled_reason {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(format!("⚠️ {}", reason))
                    .size(11.0)
                    .color(theme.alert),
            );
        }
    });
}

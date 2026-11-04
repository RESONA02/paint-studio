// start_screen.rs — หน้าเริ่มต้นสไตล์ "Warm Ink" (Start Screen & Brand Hub)
// ประกอบด้วย Monoline Logo, ปุ่มสร้างใหม่, รายการไฟล์ล่าสุด 6 รายการพร้อม Thumbnail วาดด้วยโค้ด

use super::theme::{draw_warm_ink_logo, Theme};
use super::warm_ink_icons::{draw_icon_widget, IconState, WarmToolIcon};
use crate::app::App;
use eframe::egui;

/// ข้อมูลเอกสารล่าสุดจำลอง
#[derive(Clone, Debug)]
pub struct RecentItem {
    pub title: &'static str,
    pub date: &'static str,
    pub dimensions: &'static str,
    pub preview_kind: u8,
}

pub const RECENT_PROJECTS: [RecentItem; 6] = [
    RecentItem {
        title: "Night_Sketch_01.ink",
        date: "แก้ไขเมื่อ 10 นาทีที่แล้ว",
        dimensions: "3840 × 2160 px",
        preview_kind: 0, // Landscape ink
    },
    RecentItem {
        title: "Character_Pose_Amber.ink",
        date: "เมื่อวานนี้ 21:40",
        dimensions: "2480 × 3508 px (A4)",
        preview_kind: 1, // Character silhouette
    },
    RecentItem {
        title: "Comic_Episode_04_P01.ink",
        date: "3 วันที่แล้ว",
        dimensions: "1600 × 2560 px",
        preview_kind: 2, // Comic page panels
    },
    RecentItem {
        title: "Warm_Ink_Studies.ink",
        date: "สัปดาห์ที่แล้ว",
        dimensions: "1920 × 1080 px",
        preview_kind: 3, // Abstract brush
    },
    RecentItem {
        title: "Concept_Monoline_Logo.ink",
        date: "2 สัปดาห์ก่อน",
        dimensions: "2000 × 2000 px",
        preview_kind: 4, // Vector logo curve
    },
    RecentItem {
        title: "Webtoon_Prologue_Draft.ink",
        date: "เดือนที่แล้ว",
        dimensions: "800 × 7680 px",
        preview_kind: 5, // Webtoon strip
    },
];

impl App {
    /// หน้าต่างเริ่มต้น (Start Screen Hub)
    pub(crate) fn show_start_screen_modal(&mut self, ctx: &egui::Context, theme: &Theme) {
        if !self.show_start_screen {
            return;
        }

        egui::Area::new(egui::Id::new("start_screen_overlay"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let avail = ctx.available_rect();
                let p = ui.painter();

                // พื้นหลังโปร่งใสเข้มคลุมทั้งจอ
                p.rect_filled(
                    avail,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_rgba_unmultiplied(15, 16, 20, 235),
                );

                // การ์ดลอยศูนย์กลางหน้าจอ
                let card_w = 860.0_f32.min(avail.width() - 40.0);
                let card_h = 600.0_f32.min(avail.height() - 40.0);
                let card_rect =
                    egui::Rect::from_center_size(avail.center(), egui::vec2(card_w, card_h));

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

                // ขอบบนการ์ด
                let top_bar = egui::Rect::from_min_size(card_rect.min, egui::vec2(card_w, 56.0));
                p.rect_filled(
                    top_bar,
                    egui::CornerRadius {
                        nw: theme.radius_panel as u8,
                        ne: theme.radius_panel as u8,
                        sw: 0,
                        se: 0,
                    },
                    theme.bg_panel_header,
                );
                p.line_segment(
                    [
                        egui::pos2(top_bar.min.x, top_bar.max.y),
                        egui::pos2(top_bar.max.x, top_bar.max.y),
                    ],
                    egui::Stroke::new(1.0, theme.border),
                );

                // วาดโลโก้ Monoline Warm Ink ตรงมุมบนซ้าย
                draw_warm_ink_logo(p, top_bar.min + egui::vec2(28.0, 28.0), 28.0, theme.accent);

                p.text(
                    top_bar.min + egui::vec2(52.0, 28.0),
                    egui::Align2::LEFT_CENTER,
                    "WARM INK • STUDIO",
                    egui::FontId::proportional(15.0),
                    theme.text_primary,
                );

                // ปุ่มปิดมุมขวาบน
                let close_rect = egui::Rect::from_center_size(
                    egui::pos2(top_bar.max.x - 24.0, top_bar.center().y),
                    egui::vec2(28.0, 28.0),
                );
                let close_resp = ui.interact(
                    close_rect,
                    ui.id().with("close_start"),
                    egui::Sense::click(),
                );
                if close_resp.clicked() {
                    self.show_start_screen = false;
                }
                let close_col = if close_resp.hovered() {
                    theme.accent_hover
                } else {
                    theme.text_secondary
                };
                p.text(
                    close_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "✕",
                    egui::FontId::proportional(14.0),
                    close_col,
                );

                // เนื้อหาในการ์ด
                let content_rect = egui::Rect::from_min_max(
                    top_bar.left_bottom() + egui::vec2(theme.space_16, theme.space_16),
                    card_rect.max - egui::vec2(theme.space_16, theme.space_16),
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.horizontal_top(|ui| {
                        // คอลัมน์ซ้าย: ปุ่มคำสั่งหลัก
                        ui.allocate_ui_with_layout(
                            egui::vec2(210.0, content_rect.height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new("เริ่มต้นสร้างงาน")
                                        .size(theme.font_size_header)
                                        .color(theme.text_secondary),
                                );
                                ui.add_space(12.0);

                                // ปุ่มสร้างใหม่
                                if ui
                                    .add_sized(
                                        egui::vec2(195.0, 36.0),
                                        egui::Button::new(
                                            egui::RichText::new("＋ สร้างภาพวาดใหม่")
                                                .strong()
                                                .color(theme.bg_panel),
                                        )
                                        .fill(theme.accent)
                                        .corner_radius(
                                            egui::CornerRadius::same(theme.radius_button as u8),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.clear_active_layer();
                                    self.show_start_screen = false;
                                    self.saved = "สร้างผืนผ้าใบใหม่เรียบร้อย".into();
                                }

                                ui.add_space(8.0);

                                // ปุ่มเปิดไฟล์
                                if ui
                                    .add_sized(
                                        egui::vec2(195.0, 34.0),
                                        egui::Button::new("📂 เปิดไฟล์จากเครื่อง...").corner_radius(
                                            egui::CornerRadius::same(theme.radius_button as u8),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.show_start_screen = false;
                                }

                                ui.add_space(16.0);
                                ui.separator();
                                ui.add_space(12.0);

                                // ปุ่มเปิดหน้าตรวจความต่างไอคอน (Icon Preview Suite)
                                if ui
                                    .add_sized(
                                        egui::vec2(195.0, 32.0),
                                        egui::Button::new("🔍 ตรวจระบบไอคอน 12 ชนิด").corner_radius(
                                            egui::CornerRadius::same(theme.radius_button as u8),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.show_icon_preview = true;
                                }

                                ui.add_space(8.0);

                                // ตัวเลือกสลับธีม Dark / Light
                                let theme_text = if self.is_dark_theme {
                                    "☀️ สลับเป็นโหมดสว่าง (Light)"
                                } else {
                                    "🌙 สลับเป็นโหมดมืด (Dark)"
                                };
                                if ui
                                    .add_sized(
                                        egui::vec2(195.0, 32.0),
                                        egui::Button::new(theme_text).corner_radius(
                                            egui::CornerRadius::same(theme.radius_button as u8),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.is_dark_theme = !self.is_dark_theme;
                                }
                            },
                        );

                        ui.separator();

                        // คอลัมน์ขวา: ผลงานล่าสุด 6 รายการ พร้อม Thumbnail วาดด้วยโค้ด
                        ui.allocate_ui_with_layout(
                            egui::vec2(content_rect.width() - 230.0, content_rect.height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new("ไฟล์ล่าสุด (Recent Projects)")
                                        .size(theme.font_size_header)
                                        .color(theme.text_secondary),
                                );
                                ui.add_space(10.0);

                                egui::ScrollArea::vertical().max_height(480.0).show(ui, |ui| {
                                    egui::Grid::new("recent_grid")
                                        .spacing(egui::vec2(12.0, 12.0))
                                        .show(ui, |ui| {
                                            for (idx, item) in RECENT_PROJECTS.iter().enumerate() {
                                                draw_recent_card(
                                                    ui,
                                                    theme,
                                                    item,
                                                    &mut self.show_start_screen,
                                                );
                                                if (idx + 1) % 3 == 0 {
                                                    ui.end_row();
                                                }
                                            }
                                        });
                                });
                            },
                        );
                    });
                });
            });
    }

    /// หน้าต่างแสดงผลทดสอบไอคอนทั้ง 12 ตัว ในทุก 4 สถานะ เพื่อยืนยันความแตกต่างทางสัณฐาน
    pub(crate) fn show_icon_preview_window(&mut self, ctx: &egui::Context, theme: &Theme) {
        if !self.show_icon_preview {
            return;
        }

        let mut open = self.show_icon_preview;
        egui::Window::new("Warm Ink Icon Suite — ตรวจสอบไอคอน 12 ชนิดใน 4 สถานะ")
            .open(&mut open)
            .default_width(620.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("ระบบไอคอนเวกเตอร์วาดด้วยโค้ด 100% ไม่พึ่งพา SVG หรือ Icon Pack ภายนอก ตรวจสอบความชัดเจนและสัณฐานไม่ซ้ำซ้อน")
                        .size(11.5)
                        .color(theme.text_secondary),
                );
                ui.add_space(10.0);

                egui::Grid::new("icon_matrix_grid")
                    .spacing(egui::vec2(12.0, 8.0))
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("เครื่องมือ").strong().color(theme.text_secondary));
                        ui.label(egui::RichText::new("Normal (#8E9099)").color(theme.text_secondary));
                        ui.label(egui::RichText::new("Hover (Overlay 12%)").color(theme.text_secondary));
                        ui.label(egui::RichText::new("Active (Amber #F2A93B)").color(theme.accent));
                        ui.label(egui::RichText::new("Disabled (#4A4D57)").color(theme.text_disabled));
                        ui.end_row();

                        for icon in WarmToolIcon::ALL {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon.name_th()).size(12.0));
                                ui.label(egui::RichText::new(format!("[{}]", icon.shortcut())).monospace().size(10.5).color(theme.accent));
                            });

                            // 4 สถานะ
                            for state in [IconState::Normal, IconState::Hovered, IconState::Active, IconState::Disabled] {
                                let (rect, _) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::hover());
                                draw_icon_widget(ui.painter(), rect, icon, state, theme);
                            }
                            ui.end_row();
                        }
                    });
            });
        self.show_icon_preview = open;
    }
}

/// วาดการ์ดเอกสารล่าสุด 1 ช่อง พร้อม Thumbnail แบบเวกเตอร์โค้ด
fn draw_recent_card(ui: &mut egui::Ui, theme: &Theme, item: &RecentItem, close_start: &mut bool) {
    let card_w = 175.0;
    let card_h = 145.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());
    let p = ui.painter();

    let is_hovered = resp.hovered();
    let bg_col = if is_hovered {
        theme.bg_panel_header
    } else {
        theme.bg_app
    };
    let border_col = if is_hovered {
        theme.accent
    } else {
        theme.border
    };

    p.rect_filled(
        rect,
        egui::CornerRadius::same(theme.radius_input as u8),
        bg_col,
    );
    p.rect_stroke(
        rect,
        egui::CornerRadius::same(theme.radius_input as u8),
        egui::Stroke::new(1.0, border_col),
        egui::StrokeKind::Inside,
    );

    // ส่วน Thumbnail ด้านบน (ขนาด 159x85 px)
    let thumb_rect = egui::Rect::from_min_size(
        rect.min + egui::vec2(8.0, 8.0),
        egui::vec2(card_w - 16.0, 78.0),
    );
    p.rect_filled(
        thumb_rect,
        egui::CornerRadius::same(4),
        egui::Color32::from_rgb(0x13, 0x14, 0x18),
    );
    p.rect_stroke(
        thumb_rect,
        egui::CornerRadius::same(4),
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );

    // วาดภาพพรีวิวด้วยโค้ดตามประเภท (Code-drawn artistic illustration)
    draw_procedural_thumbnail(p, thumb_rect, item.preview_kind, theme);

    // ชื่อไฟล์
    p.text(
        rect.min + egui::vec2(8.0, 96.0),
        egui::Align2::LEFT_TOP,
        item.title,
        egui::FontId::proportional(11.5),
        if is_hovered {
            theme.accent
        } else {
            theme.text_primary
        },
    );

    // ขนาดและเวลา
    p.text(
        rect.min + egui::vec2(8.0, 114.0),
        egui::Align2::LEFT_TOP,
        format!("{} • {}", item.dimensions, item.date),
        egui::FontId::proportional(9.5),
        theme.text_secondary,
    );

    if resp.clicked() {
        *close_start = false;
    }
}

/// วาดภาพประกอบพรีวิวด้วยโค้ดล้วน (ไม่มีรูป Stock ภายนอก)
fn draw_procedural_thumbnail(p: &egui::Painter, rect: egui::Rect, kind: u8, theme: &Theme) {
    let c = rect.center();
    let s = egui::Stroke::new(1.4, theme.accent);
    match kind {
        0 => {
            // Landscape ink curves
            p.line_segment(
                [
                    rect.left_bottom() + egui::vec2(10.0, -10.0),
                    c + egui::vec2(-10.0, 5.0),
                ],
                s,
            );
            p.line_segment([c + egui::vec2(-10.0, 5.0), c + egui::vec2(15.0, -8.0)], s);
            p.line_segment(
                [
                    c + egui::vec2(15.0, -8.0),
                    rect.right_bottom() + egui::vec2(-10.0, -14.0),
                ],
                s,
            );
            p.circle_filled(c + egui::vec2(25.0, -18.0), 5.0, theme.accent);
        }
        1 => {
            // Character silhouette head & neck
            p.circle_stroke(c + egui::vec2(0.0, -8.0), 12.0, s);
            p.line_segment([c + egui::vec2(-7.0, 4.0), c + egui::vec2(-14.0, 24.0)], s);
            p.line_segment([c + egui::vec2(7.0, 4.0), c + egui::vec2(14.0, 24.0)], s);
        }
        2 => {
            // Comic page 4 panels
            let r1 =
                egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 8.0), egui::vec2(60.0, 26.0));
            let r2 =
                egui::Rect::from_min_size(rect.min + egui::vec2(78.0, 8.0), egui::vec2(65.0, 26.0));
            let r3 = egui::Rect::from_min_size(
                rect.min + egui::vec2(12.0, 40.0),
                egui::vec2(131.0, 30.0),
            );
            p.rect_stroke(r1, egui::CornerRadius::ZERO, s, egui::StrokeKind::Inside);
            p.rect_stroke(r2, egui::CornerRadius::ZERO, s, egui::StrokeKind::Inside);
            p.rect_stroke(r3, egui::CornerRadius::ZERO, s, egui::StrokeKind::Inside);
        }
        3 => {
            // Abstract ink spiral
            for i in 1..=4 {
                let r = i as f32 * 5.0;
                p.circle_stroke(c, r, egui::Stroke::new(1.0, theme.text_secondary));
            }
            p.circle_filled(c, 2.5, theme.accent);
        }
        4 => {
            // Monoline logo
            draw_warm_ink_logo(p, c, 22.0, theme.accent);
        }
        _ => {
            // Webtoon vertical flow
            p.line_segment(
                [c + egui::vec2(-20.0, -22.0), c + egui::vec2(-20.0, 22.0)],
                s,
            );
            p.line_segment([c + egui::vec2(20.0, -22.0), c + egui::vec2(20.0, 22.0)], s);
            p.rect_filled(
                egui::Rect::from_center_size(c, egui::vec2(24.0, 16.0)),
                egui::CornerRadius::same(2),
                theme.border,
            );
        }
    }
}

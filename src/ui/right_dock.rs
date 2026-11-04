// right_dock.rs — พาเนลด้านขวาสไตล์ "Warm Ink" (Navigator, Tool Properties & Layers Floating Cards)
// ออกแบบเป็นการ์ดลอย 10px radius, Panel Header 32px, LayerRow 40x40 thumbnail & 3px amber active strip

use super::theme::Theme;
use super::widgets::{draw_labeled_slider, draw_layer_row, draw_panel_header};
use crate::app::App;
use crate::state::store::{Layer, LayerId};
use eframe::egui;

impl App {
    /// แสดงชุดการ์ดลอยด้านขวา (Navigator, คุณสมบัติเครื่องมือ, เลเยอร์)
    pub(crate) fn show_right_dock(&mut self, ctx: &egui::Context) {
        let theme = if self.is_dark_theme {
            Theme::dark()
        } else {
            Theme::light()
        };

        egui::SidePanel::right("warm_right_dock")
            .exact_width(320.0)
            .frame(
                egui::Frame::NONE
                    .fill(theme.bg_app)
                    .inner_margin(egui::Margin::symmetric(8, 8)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 10.0);

                    // 1. การ์ดหน้าจอนำทาง (Navigator Card)
                    self.warm_navigator_card(ui, &theme);

                    // 2. การ์ดคุณสมบัติเครื่องมือ (Tool Properties Card)
                    self.warm_tool_properties_card(ui, &theme);

                    // 3. การ์ดจัดการเลเยอร์ (Layers Card)
                    self.warm_layers_card(ui, &theme);
                });
            });
    }

    /// การ์ดหน้าจอนำทาง (Navigator Floating Card)
    fn warm_navigator_card(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folded = false;
        egui::Frame::NONE
            .fill(theme.bg_panel)
            .stroke(egui::Stroke::new(1.0, theme.border))
            .corner_radius(egui::CornerRadius::same(theme.radius_panel as u8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    draw_panel_header(ui, theme, "หน้าจอนำทาง (Navigator)", &mut folded);
                    ui.add_space(8.0);

                    let avail_w = ui.available_width();
                    let (preview_rect, _) = ui.allocate_exact_size(
                        egui::vec2(avail_w - 16.0, 120.0),
                        egui::Sense::hover(),
                    );
                    let pp = ui.painter();
                    pp.rect_filled(
                        preview_rect,
                        egui::CornerRadius::same(6),
                        egui::Color32::from_rgb(0x13, 0x14, 0x18),
                    );
                    pp.rect_stroke(
                        preview_rect,
                        egui::CornerRadius::same(6),
                        egui::Stroke::new(1.0, theme.border),
                        egui::StrokeKind::Inside,
                    );

                    if let Some(tex) = &self.texture {
                        let mini_rect = egui::Rect::from_center_size(
                            preview_rect.center(),
                            egui::vec2(160.0, 90.0),
                        );
                        pp.image(
                            tex.id(),
                            mini_rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                        pp.rect_stroke(
                            mini_rect,
                            egui::CornerRadius::ZERO,
                            egui::Stroke::new(1.0, theme.accent),
                            egui::StrokeKind::Inside,
                        );
                    }

                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        if ui.small_button("－").clicked() {
                            self.zoom = (self.zoom - 0.1).max(0.2);
                        }
                        ui.add(egui::Slider::new(&mut self.zoom, 0.2..=3.0).show_value(false));
                        if ui.small_button("＋").clicked() {
                            self.zoom = (self.zoom + 0.1).min(3.0);
                        }
                        if ui.small_button("พอดี").clicked() {
                            self.zoom = 1.0;
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    });
                    ui.add_space(8.0);
                });
            });
    }

    /// การ์ดคุณสมบัติเครื่องมือ (Tool Properties Floating Card)
    fn warm_tool_properties_card(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folded = false;
        egui::Frame::NONE
            .fill(theme.bg_panel)
            .stroke(egui::Stroke::new(1.0, theme.border))
            .corner_radius(egui::CornerRadius::same(theme.radius_panel as u8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    draw_panel_header(ui, theme, "คุณสมบัติเครื่องมือ (Tool Properties)", &mut folded);
                    ui.add_space(8.0);

                    ui.scope_builder(
                        egui::UiBuilder::new().max_rect(ui.max_rect().shrink2(egui::vec2(8.0, 0.0))),
                        |ui| {
                            ui.vertical(|ui| {
                                draw_labeled_slider(
                                    ui,
                                    theme,
                                    "ขนาดแปรง",
                                    &mut self.size,
                                    1.0..=120.0,
                                    " px",
                                );
                                ui.add_space(6.0);
                                draw_labeled_slider(
                                    ui,
                                    theme,
                                    "ความทึบ",
                                    &mut self.opacity,
                                    0.05..=1.0,
                                    "",
                                );
                                ui.add_space(6.0);
                                draw_labeled_slider(
                                    ui,
                                    theme,
                                    "ความแข็ง",
                                    &mut self.hardness,
                                    0.05..=1.0,
                                    "",
                                );
                            });
                        },
                    );
                    ui.add_space(8.0);
                });
            });
    }

    /// การ์ดจัดการเลเยอร์ (Layers Floating Card)
    fn warm_layers_card(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folded = false;
        egui::Frame::NONE
            .fill(theme.bg_panel)
            .stroke(egui::Stroke::new(1.0, theme.border))
            .corner_radius(egui::CornerRadius::same(theme.radius_panel as u8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    draw_panel_header(ui, theme, "เลเยอร์ (Layers)", &mut folded);
                    ui.add_space(8.0);

                    ui.scope_builder(
                        egui::UiBuilder::new().max_rect(ui.max_rect().shrink2(egui::vec2(8.0, 0.0))),
                        |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    egui::ComboBox::from_id_salt("warm_blend_mode")
                                        .selected_text(&self.layers[self.active].blend_mode)
                                        .show_ui(ui, |ui| {
                                            for mode in ["ปกติ", "คูณ", "สกรีน", "ซ้อนทับ", "บวก"] {
                                                ui.selectable_value(
                                                    &mut self.layers[self.active].blend_mode,
                                                    mode.to_string(),
                                                    mode,
                                                );
                                            }
                                        });

                                    let mut op_percent = self.layers[self.active].opacity_percent;
                                    if ui
                                        .add(egui::Slider::new(&mut op_percent, 0..=100).text("%"))
                                        .changed()
                                    {
                                        self.layers[self.active].opacity_percent = op_percent;
                                        self.texture_dirty = true;
                                    }
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui.button("＋ เลเยอร์ใหม่").clicked() {
                                        self.add_dock_layer();
                                    }
                                    if ui.button("🗑️ ลบ").clicked() {
                                        self.delete_active_layer();
                                    }
                                });

                                ui.add_space(6.0);
                                ui.separator();
                                ui.add_space(6.0);

                                egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                                    for i in (0..self.layers.len()).rev() {
                                        let is_active = i == self.active;
                                        let name = self.layers[i].name.clone();
                                        let op = self.layers[i].opacity_percent;
                                        let mut vis = self.layers[i].visible;
                                        let mut lock = self.layers[i].locked;

                                        if draw_layer_row(
                                            ui,
                                            theme,
                                            &name,
                                            is_active,
                                            op,
                                            &mut vis,
                                            &mut lock,
                                        ) {
                                            self.active = i;
                                        }

                                        if vis != self.layers[i].visible {
                                            self.layers[i].visible = vis;
                                            self.texture_dirty = true;
                                        }
                                        self.layers[i].locked = lock;

                                        ui.add_space(4.0);
                                    }
                                });
                            });
                        },
                    );
                    ui.add_space(8.0);
                });
            });
    }

    fn add_dock_layer(&mut self) {
        let k = self.layers.len();
        let (w, h) = (self.w, self.h);
        self.layers
            .push(Layer::new(LayerId(k), format!("เลเยอร์ {}", k), w, h, false));
        self.active = self.layers.len() - 1;
        self.texture_dirty = true;
        self.push_hist("เพิ่มเลเยอร์");
    }
}

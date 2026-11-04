// canvas_view.rs — central workspace: canvas rendering, input and status bar
use crate::app::comic::{Balloon, ComicFrame, PREVIEW_RED};
use crate::app::App;
use crate::state::store::{IVec2, SelectionRect};
use crate::tools::Tool;
use eframe::egui;

use super::icons::draw_marching_ants;

impl App {
    pub(crate) fn show_canvas(&mut self, ctx: &egui::Context) {
        let theme = if self.is_dark_theme {
            super::theme::Theme::dark()
        } else {
            super::theme::Theme::light()
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.bg_app))
            .show(ctx, |ui| {
                let avail = ui.available_size();
                let canvas_rect = self.canvas_rect(ui, avail);
                self.paint_canvas(ui, canvas_rect);
                self.paint_selection_overlay(ui, canvas_rect);
                self.paint_comic_overlay(ui, canvas_rect);
                self.paint_eyedropper_loupe(ui, canvas_rect, ui.input(|i| i.pointer.hover_pos()));
                self.handle_canvas_input(ui, canvas_rect);
                self.status_bar(ui, avail, &theme);
            });
    }

    fn canvas_rect(&self, ui: &egui::Ui, avail: egui::Vec2) -> egui::Rect {
        let cw = self.w as f32 * self.zoom;
        let ch = self.h as f32 * self.zoom;
        let offset_x = (avail.x - cw).max(0.0_f32) / 2.0_f32 + self.pan_offset.x;
        let offset_y = (avail.y - ch - 30.0_f32).max(0.0_f32) / 2.0_f32 + self.pan_offset.y;
        egui::Rect::from_min_size(
            ui.cursor().min + egui::vec2(offset_x, offset_y + 10.0_f32),
            egui::vec2(cw, ch),
        )
    }

    fn paint_canvas(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let p = ui.painter();
        p.rect_filled(
            canvas_rect.translate(egui::vec2(2.0_f32, 2.0_f32)),
            egui::CornerRadius::ZERO,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 35),
        );
        p.rect_filled(canvas_rect, egui::CornerRadius::ZERO, egui::Color32::WHITE);

        if let Some(tex) = &self.texture {
            p.image(
                tex.id(),
                canvas_rect,
                egui::Rect::from_min_max(
                    egui::pos2(0.0_f32, 0.0_f32),
                    egui::pos2(1.0_f32, 1.0_f32),
                ),
                egui::Color32::WHITE,
            );
        }

        p.rect_stroke(
            canvas_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60, 60, 60)),
            egui::StrokeKind::Inside,
        );

        // ไม้บรรทัดสมมาตร (Symmetry Ruler vertical axis)
        if self.symmetry_enabled {
            let axis_screen_x = canvas_rect.min.x + (self.symmetry_axis_x as f32 * self.zoom);
            if axis_screen_x >= canvas_rect.min.x && axis_screen_x <= canvas_rect.max.x {
                p.line_segment(
                    [
                        egui::pos2(axis_screen_x, canvas_rect.min.y),
                        egui::pos2(axis_screen_x, canvas_rect.max.y),
                    ],
                    egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgba_unmultiplied(184, 115, 58, 200),
                    ),
                );
                // จุดกึ่งกลางและปลายแกนสมมาตร
                p.circle_filled(
                    egui::pos2(axis_screen_x, canvas_rect.min.y + 4.0_f32),
                    2.5_f32,
                    egui::Color32::from_rgb(184, 115, 58),
                );
                p.circle_filled(
                    egui::pos2(axis_screen_x, canvas_rect.max.y - 4.0_f32),
                    2.5_f32,
                    egui::Color32::from_rgb(184, 115, 58),
                );
            }
        }
    }

    /// วาดเส้นประรอบ Floating Selection (พร้อม Handles) หรือ Active Selection
    fn paint_selection_overlay(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let p = ui.painter();
        if self.selecting && self.lasso_points.len() > 1 {
            let pts: Vec<egui::Pos2> = self
                .lasso_points
                .iter()
                .map(|pt| {
                    canvas_rect.min + egui::vec2(pt.x as f32 * self.zoom, pt.y as f32 * self.zoom)
                })
                .collect();
            for w in pts.windows(2) {
                p.line_segment(
                    [w[0], w[1]],
                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(40, 120, 220)),
                );
            }
        }

        if let Some(fl) = &self.floating {
            let rect = egui::Rect::from_min_size(
                canvas_rect.min
                    + egui::vec2(
                        fl.origin.x as f32 * self.zoom,
                        fl.origin.y as f32 * self.zoom,
                    ),
                egui::vec2(
                    fl.pixels.width as f32 * self.zoom,
                    fl.pixels.height as f32 * self.zoom,
                ),
            );
            draw_marching_ants(p, rect, true);
        } else if let Some(sel) = &self.selection {
            let rect = egui::Rect::from_min_size(
                canvas_rect.min
                    + egui::vec2(sel.min.x as f32 * self.zoom, sel.min.y as f32 * self.zoom),
                egui::vec2(
                    sel.width() as f32 * self.zoom,
                    sel.height() as f32 * self.zoom,
                ),
            );
            draw_marching_ants(p, rect, false);
        }
    }

    /// วาดกรอบช่องการ์ตูนและบอลลูนเวกเตอร์ทับบนผ้าใบ
    fn paint_comic_overlay(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let p = ui.painter();
        for frame in &self.comic_frames {
            let rect = self.comic_rect(canvas_rect, frame.min, frame.max);
            p.rect_stroke(
                rect,
                egui::CornerRadius::ZERO,
                egui::Stroke::new(3.0_f32, egui::Color32::from_rgb(20, 20, 24)),
                egui::StrokeKind::Inside,
            );
        }
        for balloon in &self.balloons {
            draw_balloon_shape(p, canvas_rect, balloon, self.zoom);
        }
        if let (Some(a), Some(b)) = (self.frame_start, self.frame_current) {
            let frame = ComicFrame::new(a, b);
            let rect = self.comic_rect(canvas_rect, frame.min, frame.max);
            p.rect_stroke(
                rect,
                egui::CornerRadius::ZERO,
                egui::Stroke::new(1.0_f32, PREVIEW_RED),
                egui::StrokeKind::Inside,
            );
        }
    }

    fn comic_rect(&self, canvas_rect: egui::Rect, min: IVec2, max: IVec2) -> egui::Rect {
        egui::Rect::from_min_max(
            canvas_rect.min + egui::vec2(min.x as f32, min.y as f32) * self.zoom,
            canvas_rect.min + egui::vec2(max.x as f32, max.y as f32) * self.zoom,
        )
    }

    fn pointer_pixel(&self, ptr: Option<egui::Pos2>, rect: egui::Rect) -> Option<IVec2> {
        let pos = ptr?;
        let (x, y) = self.canvas_pos(pos, rect);
        Some(IVec2::new(x, y))
    }

    fn drag_comic_frame(
        &mut self,
        resp: &egui::Response,
        ptr: Option<egui::Pos2>,
        canvas_rect: egui::Rect,
    ) {
        let Some(pixel) = self.pointer_pixel(ptr, canvas_rect) else {
            if resp.drag_stopped() {
                self.commit_frame_drag();
            }
            return;
        };
        if resp.drag_started() {
            self.begin_frame_drag(pixel);
        }
        if resp.dragged() {
            self.update_frame_drag(pixel);
        }
        if resp.drag_stopped() {
            self.commit_frame_drag();
        }
    }

    fn drag_balloon(
        &mut self,
        resp: &egui::Response,
        ptr: Option<egui::Pos2>,
        canvas_rect: egui::Rect,
    ) {
        let Some(pixel) = self.pointer_pixel(ptr, canvas_rect) else {
            return;
        };
        if resp.drag_started() {
            self.begin_balloon_drag(pixel);
        }
        if resp.drag_stopped() {
            self.commit_balloon_drag(pixel);
        } else if resp.clicked() {
            self.selected_balloon = self.balloon_at(pixel);
        }
    }

    fn handle_canvas_input(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let resp = ui.interact(
            canvas_rect,
            egui::Id::new("main_canvas"),
            egui::Sense::click_and_drag(),
        );
        let ptr = resp.interact_pointer_pos();

        if self.space_held || self.tool == Tool::Hand {
            if resp.dragged() {
                self.pan_offset += resp.drag_delta();
            }
        } else if self.alt_held || self.tool == Tool::EyeDropper {
            self.pick_color(&resp, ptr, canvas_rect);
        } else if self.floating.is_some() {
            self.drag_floating(&resp);
        } else if self.tool == Tool::Lasso {
            self.drag_selection(&resp, ptr, canvas_rect);
        } else if self.tool == Tool::ComicFrame {
            self.drag_comic_frame(&resp, ptr, canvas_rect);
        } else if self.tool == Tool::Balloon {
            self.drag_balloon(&resp, ptr, canvas_rect);
        } else {
            self.drag_brush(&resp, ptr, canvas_rect);
        }
    }

    fn pick_color(
        &mut self,
        resp: &egui::Response,
        ptr: Option<egui::Pos2>,
        canvas_rect: egui::Rect,
    ) {
        if !(resp.clicked() || (resp.dragged() && ptr.is_some())) {
            return;
        }
        let Some(pos) = ptr else {
            return;
        };
        let (x, y) = self.canvas_pos(pos, canvas_rect);
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            let (r, g, b, _) = self.layers[self.active].pixels[y as usize][x as usize];
            self.main_color = egui::Color32::from_rgb(r, g, b);
        }
    }

    /// การ Drag ด้วยเมาส์จะเลื่อนตำแหน่งของ Floating Layer
    fn drag_floating(&mut self, resp: &egui::Response) {
        if !resp.dragged() {
            return;
        }
        let delta = resp.drag_delta() / self.zoom;
        if delta.length_sq() <= 0.0 {
            return;
        }
        let iv_delta = IVec2::new(delta.x.round() as isize, delta.y.round() as isize);
        if iv_delta.x == 0 && iv_delta.y == 0 {
            return;
        }
        if let Some(fl) = &mut self.floating {
            fl.origin += iv_delta;
            self.texture_dirty = true;
        }
    }

    /// โหมดเลือกขอบเขตสี่เหลี่ยม หรือ Lasso อิสระ
    fn drag_selection(
        &mut self,
        resp: &egui::Response,
        ptr: Option<egui::Pos2>,
        canvas_rect: egui::Rect,
    ) {
        if resp.drag_started() {
            if let Some(pos) = ptr {
                let (x, y) = self.canvas_pos(pos, canvas_rect);
                self.selecting = true;
                self.selection_start = Some(IVec2::new(x, y));
                self.lasso_points.clear();
                self.lasso_points.push(IVec2::new(x, y));
            }
        }
        if self.selecting {
            if let Some(pos) = ptr {
                let (cx, cy) = self.canvas_pos(pos, canvas_rect);
                let current_pt = IVec2::new(cx, cy);
                if self.lasso_points.last() != Some(&current_pt) {
                    self.lasso_points.push(current_pt);
                }
                if let Some(start) = self.selection_start {
                    let mut min_x = start.x.min(cx);
                    let mut min_y = start.y.min(cy);
                    let mut max_x = start.x.max(cx);
                    let mut max_y = start.y.max(cy);
                    for pt in &self.lasso_points {
                        min_x = min_x.min(pt.x);
                        min_y = min_y.min(pt.y);
                        max_x = max_x.max(pt.x);
                        max_y = max_y.max(pt.y);
                    }
                    self.selection = Some(SelectionRect {
                        min: IVec2::new(min_x, min_y),
                        max: IVec2::new(max_x, max_y),
                    });
                }
            }
        }
        if resp.drag_stopped() {
            self.selecting = false;
            self.selection_start = None;
        }
    }

    /// โหมดวาดปกติ (Brush Stroke) พร้อมบันทึก Snapshot Undo/Redo
    fn drag_brush(
        &mut self,
        resp: &egui::Response,
        ptr: Option<egui::Pos2>,
        canvas_rect: egui::Rect,
    ) {
        if resp.drag_started() {
            self.drawing = true;
            let snapshot = crate::app::PixelSnapshot {
                layer_idx: self.active,
                pixels: self.layers[self.active].pixels.clone(),
            };
            self.undo_snapshots.push(snapshot);
            if self.undo_snapshots.len() > 30 {
                self.undo_snapshots.remove(0);
            }
            self.redo_snapshots.clear();
        }
        if resp.drag_stopped() {
            self.drawing = false;
            self.last = None;
            self.push_hist("วาดเส้น");
        }
        if !self.drawing {
            return;
        }
        if let Some(pos) = ptr {
            let c = self.canvas_pos(pos, canvas_rect);
            if let Some(l) = self.last {
                self.stroke(l, c);
            } else {
                self.stroke(c, c);
            }
            self.last = Some(c);
        }
    }

    /// แว่นขยายดูดสีพิกเซล (Eyedropper Loupe) ตามมาตรฐาน Anti-AI-Slop
    fn paint_eyedropper_loupe(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        ptr: Option<egui::Pos2>,
    ) {
        if !(self.alt_held || self.tool == Tool::EyeDropper) {
            self.hovered_canvas_pixel = None;
            return;
        }
        let Some(pos) = ptr else {
            self.hovered_canvas_pixel = None;
            return;
        };
        if !canvas_rect.contains(pos) {
            self.hovered_canvas_pixel = None;
            return;
        }
        let (cx, cy) = self.canvas_pos(pos, canvas_rect);
        if cx < 0 || cy < 0 || (cx as usize) >= self.w || (cy as usize) >= self.h {
            self.hovered_canvas_pixel = None;
            return;
        }
        self.hovered_canvas_pixel = Some((cx, cy));

        let p = ui.painter();
        let (cr, cg, cb, _) = self.layers[self.active].pixels[cy as usize][cx as usize];
        let current_col = egui::Color32::from_rgb(cr, cg, cb);

        // ตำแหน่งของกล่องแว่นขยาย
        let loupe_w = 110.0_f32;
        let loupe_h = 138.0_f32;
        let mut top_left = pos + egui::vec2(22.0_f32, -70.0_f32);
        if top_left.x + loupe_w > ui.max_rect().max.x {
            top_left.x = pos.x - loupe_w - 22.0_f32;
        }
        if top_left.y < ui.max_rect().min.y + 30.0_f32 {
            top_left.y = ui.max_rect().min.y + 30.0_f32;
        }
        if top_left.y + loupe_h > ui.max_rect().max.y - 30.0_f32 {
            top_left.y = ui.max_rect().max.y - loupe_h - 30.0_f32;
        }
        let loupe_rect = egui::Rect::from_min_size(top_left, egui::vec2(loupe_w, loupe_h));

        // กล่องพื้นหลังแบบ Matte Charcoal (#1a1a1e)
        p.rect_filled(
            loupe_rect,
            egui::CornerRadius::same(6),
            egui::Color32::from_rgb(26, 26, 30),
        );
        p.rect_stroke(
            loupe_rect,
            egui::CornerRadius::same(6),
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60, 60, 68)),
            egui::StrokeKind::Inside,
        );

        // ตารางพิกเซลขยาย 5x5 (ช่องละ 14x14 px)
        let grid_size = 70.0_f32;
        let cell_size = 14.0_f32;
        let grid_origin = egui::pos2(
            loupe_rect.min.x + (loupe_w - grid_size) / 2.0_f32,
            loupe_rect.min.y + 8.0_f32,
        );

        for dy in -2..=2 {
            for dx in -2..=2 {
                let sx = cx + dx;
                let sy = cy + dy;
                let cell_col =
                    if sx >= 0 && sy >= 0 && (sx as usize) < self.w && (sy as usize) < self.h {
                        let (r, g, b, _) =
                            self.layers[self.active].pixels[sy as usize][sx as usize];
                        egui::Color32::from_rgb(r, g, b)
                    } else {
                        egui::Color32::from_rgb(40, 40, 44)
                    };
                let cell_rect = egui::Rect::from_min_size(
                    grid_origin
                        + egui::vec2((dx + 2) as f32 * cell_size, (dy + 2) as f32 * cell_size),
                    egui::vec2(cell_size, cell_size),
                );
                p.rect_filled(cell_rect, egui::CornerRadius::ZERO, cell_col);
                p.rect_stroke(
                    cell_rect,
                    egui::CornerRadius::ZERO,
                    egui::Stroke::new(0.5_f32, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80)),
                    egui::StrokeKind::Inside,
                );
            }
        }

        // Reticle กึ่งกลางเล็งพิกเซลปัจจุบัน
        let center_rect = egui::Rect::from_min_size(
            grid_origin + egui::vec2(2.0_f32 * cell_size, 2.0_f32 * cell_size),
            egui::vec2(cell_size, cell_size),
        );
        p.rect_stroke(
            center_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(1.5_f32, egui::Color32::WHITE),
            egui::StrokeKind::Inside,
        );
        p.rect_stroke(
            center_rect.expand(1.0_f32),
            egui::CornerRadius::ZERO,
            egui::Stroke::new(1.0_f32, egui::Color32::BLACK),
            egui::StrokeKind::Outside,
        );

        // Split swatch เปรียบเทียบสีเดิม vs สีใหม่ที่ดูด
        let swatch_rect = egui::Rect::from_min_size(
            egui::pos2(grid_origin.x, grid_origin.y + grid_size + 6.0_f32),
            egui::vec2(grid_size, 16.0_f32),
        );
        let half_w = grid_size / 2.0_f32;
        let prev_rect = egui::Rect::from_min_size(swatch_rect.min, egui::vec2(half_w, 16.0_f32));
        let curr_rect = egui::Rect::from_min_size(
            egui::pos2(swatch_rect.min.x + half_w, swatch_rect.min.y),
            egui::vec2(half_w, 16.0_f32),
        );
        p.rect_filled(
            prev_rect,
            egui::CornerRadius {
                nw: 3,
                ne: 0,
                sw: 3,
                se: 0,
            },
            self.main_color,
        );
        p.rect_filled(
            curr_rect,
            egui::CornerRadius {
                nw: 0,
                ne: 3,
                sw: 0,
                se: 3,
            },
            current_col,
        );
        p.rect_stroke(
            swatch_rect,
            egui::CornerRadius::same(3),
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(50, 50, 58)),
            egui::StrokeKind::Inside,
        );

        // ป้ายรหัสสี Hex
        let hex_text = format!("#{:02X}{:02X}{:02X}", cr, cg, cb);
        p.text(
            egui::pos2(loupe_rect.center().x, swatch_rect.max.y + 16.0_f32),
            egui::Align2::CENTER_CENTER,
            hex_text,
            egui::FontId::monospace(10.5_f32),
            egui::Color32::from_rgb(220, 220, 225),
        );
    }

    fn status_bar(&mut self, ui: &mut egui::Ui, avail: egui::Vec2, theme: &super::theme::Theme) {
        let bot_rect = egui::Rect::from_min_size(
            egui::pos2(ui.cursor().min.x, ui.max_rect().max.y - 24.0_f32),
            egui::vec2(avail.x, 24.0_f32),
        );
        let p = ui.painter();
        p.rect_filled(bot_rect, egui::CornerRadius::ZERO, theme.bg_panel_header);
        p.rect_stroke(
            bot_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(1.0_f32, theme.border),
            egui::StrokeKind::Inside,
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(bot_rect), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(8.0_f32);

                // 1. พิกัดเคอร์เซอร์ (Tabular monospace)
                let coord_text = if let Some(pt) = self.hovered_canvas_pixel {
                    format!("X: {:4}  Y: {:4}", pt.0, pt.1)
                } else {
                    "X: ----  Y: ----".to_string()
                };
                ui.label(
                    egui::RichText::new(coord_text)
                        .monospace()
                        .size(11.0_f32)
                        .color(theme.text_secondary),
                );
                ui.separator();

                // 2. ขนาดภาพ (Dimensions)
                ui.label(
                    egui::RichText::new(format!("{} × {} px", self.w, self.h))
                        .monospace()
                        .size(11.0_f32)
                        .color(theme.text_secondary),
                );
                ui.separator();

                // 3. ซูม (Zoom %)
                ui.label(
                    egui::RichText::new(format!("{:.0}%", self.zoom * 100.0_f32))
                        .monospace()
                        .size(11.0_f32)
                        .color(theme.accent),
                );
                if ui.small_button("－").clicked() {
                    self.zoom = (self.zoom - 0.1_f32).max(0.2_f32);
                }
                if ui.small_button("＋").clicked() {
                    self.zoom = (self.zoom + 0.1_f32).min(3.0_f32);
                }
                ui.separator();

                // 4. มุมหมุน (Rotation Angle)
                ui.label(
                    egui::RichText::new(format!("{:.1}°", self.rotation))
                        .monospace()
                        .size(11.0_f32)
                        .color(theme.text_secondary),
                );
                ui.separator();

                // 5. ข้อความสถานะของเครื่องมือ
                self.selection_status(ui, theme);
            });
        });
    }

    /// แสดงสถานะเครื่องมือใน Status Bar
    fn selection_status(&self, ui: &mut egui::Ui, theme: &super::theme::Theme) {
        if self.floating.is_some() {
            ui.label(
                egui::RichText::new("กำลังลอย: กด Enter เพื่อยืนยัน หรือ Esc เพื่อยกเลิก")
                    .size(11.0_f32)
                    .strong()
                    .color(theme.alert),
            );
        } else if self.selection.is_some() {
            ui.label(
                egui::RichText::new("เลือกขอบเขตอยู่: กด Ctrl+X หรือ L เพื่อหยิบส่วนที่เลือก")
                    .size(11.0_f32)
                    .color(theme.accent),
            );
        } else {
            ui.label(
                egui::RichText::new(format!(
                    "เครื่องมือ: {:?} | แปรง: {:.0} px | หน้า {} / {}",
                    self.subtool,
                    self.size,
                    self.page_index + 1,
                    self.pages.len()
                ))
                .size(11.0_f32)
                .color(theme.text_secondary),
            );
        }
    }
}

/// วาดบอลลูนคำพูดแบบเวกเตอร์ (วงรี + หาง + ข้อความ)
fn draw_balloon_shape(p: &egui::Painter, canvas_rect: egui::Rect, b: &Balloon, zoom: f32) {
    let center = canvas_rect.min + egui::vec2(b.center.x as f32, b.center.y as f32) * zoom;
    let radius = egui::vec2(b.rx as f32, b.ry as f32) * zoom;
    if radius.x < 2.0_f32 || radius.y < 2.0_f32 {
        return;
    }
    let stroke = egui::Stroke::new(1.6_f32, egui::Color32::from_rgb(25, 25, 30));
    p.add(egui::Shape::ellipse_filled(
        center,
        radius,
        egui::Color32::WHITE,
    ));
    p.add(egui::Shape::ellipse_stroke(center, radius, stroke));
    let tail = canvas_rect.min + egui::vec2(b.tail.x as f32, b.tail.y as f32) * zoom;
    let left = center + egui::vec2(-radius.x * 0.35_f32, radius.y * 0.75_f32);
    let right = center + egui::vec2(radius.x * 0.35_f32, radius.y * 0.75_f32);
    p.add(egui::Shape::convex_polygon(
        vec![left, right, tail],
        egui::Color32::WHITE,
        stroke,
    ));
    p.text(
        center,
        egui::Align2::CENTER_CENTER,
        &b.text,
        egui::FontId::proportional((13.0_f32 * zoom).clamp(7.0_f32, 42.0_f32)),
        egui::Color32::from_rgb(20, 20, 20),
    );
}

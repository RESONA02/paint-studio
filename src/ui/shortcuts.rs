// shortcuts.rs — keyboard shortcut handling (CSP-style bindings)
use crate::state::store::{Action, IVec2};
use crate::tools::Tool;
use eframe::egui;

use super::super::App;

/// ผลลัพธ์ของการจับคู่คีย์ลัดหนึ่งกลุ่ม
enum KeyResult {
    Ignored,
    Handled,
    Action(Action),
}

impl App {
    pub(crate) fn handle_keyboard(&mut self, ctx: &egui::Context) {
        let wants_keyboard = ctx.wants_keyboard_input();
        let mut action = None;

        ctx.input(|i| {
            self.space_held = i.key_down(egui::Key::Space);
            self.alt_held = i.modifiers.alt;
            self.shift_held = i.modifiers.shift;

            if !wants_keyboard {
                action = self.handle_shortcuts(i);
            }
        });

        if let Some(act) = action {
            self.dispatch(act);
        }
    }

    fn handle_shortcuts(&mut self, i: &egui::InputState) -> Option<Action> {
        // Tab : ซ่อน/แสดงพาเนลด้านข้างทั้งหมดชั่วคราว
        if i.key_pressed(egui::Key::Tab) {
            self.hide_side_panels = !self.hide_side_panels;
        }

        if i.modifiers.ctrl || i.modifiers.command {
            return self.ctrl_shortcuts(i);
        }

        // Alt + Backspace หรือ Alt + Delete (เทสีลงเลเยอร์/Selection)
        if i.modifiers.alt
            && (i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Delete))
        {
            self.fill_selection_or_layer();
            return None;
        }

        // คำสั่ง Floating Selection เมื่อกำลังลอยอยู่
        if self.floating.is_some() {
            return self.floating_shortcuts(i);
        }

        match self.tool_shortcuts(i) {
            KeyResult::Action(a) => Some(a),
            _ => None,
        }
    }

    fn ctrl_shortcuts(&mut self, i: &egui::InputState) -> Option<Action> {
        if self.ctrl_panel_shortcuts(i) {
            return None;
        }
        if i.key_pressed(egui::Key::T) {
            return self.start_transform();
        }
        if self.ctrl_edit_shortcuts(i) {
            return None;
        }
        self.ctrl_view_shortcuts(i)
    }

    /// กลุ่มคำสั่งควบคุมพาเนล/เลเยอร์ (คืน true เมื่อจัดการแล้ว)
    fn ctrl_panel_shortcuts(&mut self, i: &egui::InputState) -> bool {
        if i.modifiers.alt && i.modifiers.shift && i.key_pressed(egui::Key::K) {
            self.show_shortcut_dialog = true;
        } else if i.modifiers.shift && i.key_pressed(egui::Key::H) {
            self.hide_all_ui = !self.hide_all_ui;
        } else if i.modifiers.shift && i.modifiers.alt && i.key_pressed(egui::Key::E) {
            self.merge_visible_to_new_layer();
        } else if i.modifiers.shift && i.key_pressed(egui::Key::N) {
            self.add_raster_layer();
        } else if i.modifiers.shift && i.key_pressed(egui::Key::I) {
            self.inverse_selection();
        } else {
            return false;
        }
        true
    }

    fn start_transform(&mut self) -> Option<Action> {
        let action = if self.selection.is_some() && self.floating.is_none() {
            Some(Action::LiftSelection)
        } else {
            None
        };
        self.saved = "โหมดปรับขนาด/ย้ายตำแหน่ง (Transform)".into();
        action
    }

    /// กลุ่มคำสั่งแก้ไข (Ctrl+C/V/A/D/E) — คืน true เมื่อจัดการแล้ว
    fn ctrl_edit_shortcuts(&mut self, i: &egui::InputState) -> bool {
        if i.key_pressed(egui::Key::C) {
            self.copy_selection();
        } else if i.key_pressed(egui::Key::V) {
            self.paste_clipboard();
        } else if i.key_pressed(egui::Key::A) {
            self.select_all();
        } else if i.key_pressed(egui::Key::D) {
            self.deselect();
        } else if i.key_pressed(egui::Key::E) {
            self.merge_layer_down();
        } else {
            return false;
        }
        true
    }

    /// กลุ่มคำสั่งมุมมอง/ประวัติ/บันทึก (Ctrl+0/+/-/Z/Y/S/X)
    fn ctrl_view_shortcuts(&mut self, i: &egui::InputState) -> Option<Action> {
        if i.key_pressed(egui::Key::Num0) {
            self.zoom = 1.0_f32;
            self.pan_offset = egui::Vec2::ZERO;
            self.texture_dirty = true;
        } else if i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals) {
            self.zoom = (self.zoom * 1.25_f32).min(32.0_f32);
            self.texture_dirty = true;
        } else if i.key_pressed(egui::Key::Minus) {
            self.zoom = (self.zoom / 1.25_f32).max(0.05_f32);
            self.texture_dirty = true;
        } else if i.key_pressed(egui::Key::Z) {
            if i.modifiers.shift {
                self.redo();
            } else {
                self.undo();
            }
        } else if i.key_pressed(egui::Key::Y) {
            self.redo();
        } else if i.key_pressed(egui::Key::S) {
            let _ = self.save_png();
            self.saved = "บันทึกเรียบร้อย".into();
        } else if i.key_pressed(egui::Key::X) && self.selection.is_some() && self.floating.is_none()
        {
            return Some(Action::LiftSelection);
        }
        None
    }

    fn floating_shortcuts(&mut self, i: &egui::InputState) -> Option<Action> {
        if i.key_pressed(egui::Key::Enter) {
            return Some(Action::CommitFloating);
        }
        if i.key_pressed(egui::Key::Escape) {
            return Some(Action::CancelFloating);
        }
        if i.key_pressed(egui::Key::Delete) {
            return Some(Action::DeleteFloating);
        }
        self.floating_arrow(i)
    }

    fn floating_arrow(&mut self, i: &egui::InputState) -> Option<Action> {
        let step = if i.modifiers.shift { 10 } else { 1 };
        if i.key_pressed(egui::Key::ArrowLeft) {
            return Some(Action::MoveFloating(IVec2::new(-step, 0)));
        }
        if i.key_pressed(egui::Key::ArrowRight) {
            return Some(Action::MoveFloating(IVec2::new(step, 0)));
        }
        if i.key_pressed(egui::Key::ArrowUp) {
            return Some(Action::MoveFloating(IVec2::new(0, -step)));
        }
        if i.key_pressed(egui::Key::ArrowDown) {
            return Some(Action::MoveFloating(IVec2::new(0, step)));
        }
        None
    }

    fn tool_shortcuts(&mut self, i: &egui::InputState) -> KeyResult {
        match self.paint_tool_shortcuts(i) {
            KeyResult::Ignored => self.utility_tool_shortcuts(i),
            other => other,
        }
    }

    /// Shift+P, B, P, E, U, M, L, W, G
    fn paint_tool_shortcuts(&mut self, i: &egui::InputState) -> KeyResult {
        if i.modifiers.shift && i.key_pressed(egui::Key::P) {
            self.symmetry_enabled = !self.symmetry_enabled;
            self.saved = if self.symmetry_enabled {
                "เปิดไม้บรรทัดสมมาตร (Symmetry Ruler ON)".into()
            } else {
                "ปิดไม้บรรทัดสมมาตร (Symmetry Ruler OFF)".into()
            };
            self.texture_dirty = true;
            return KeyResult::Handled;
        } else if i.key_pressed(egui::Key::B) {
            self.set_tool(Tool::Brush);
        } else if i.key_pressed(egui::Key::P) {
            self.set_tool(Tool::Pen);
        } else if i.key_pressed(egui::Key::E) {
            self.set_tool(Tool::Eraser);
        } else if i.key_pressed(egui::Key::U) {
            self.set_tool(Tool::Figure);
        } else if i.key_pressed(egui::Key::M) {
            self.set_tool(Tool::Lasso);
        } else if i.key_pressed(egui::Key::L) {
            return self.lasso_or_lift();
        } else if i.key_pressed(egui::Key::W) {
            self.tool = Tool::MagicWand;
        } else if i.key_pressed(egui::Key::G) {
            self.tool = if self.tool == Tool::Fill {
                Tool::Gradient
            } else {
                Tool::Fill
            };
            self.reset_subtool_to_first();
        } else {
            return KeyResult::Ignored;
        }
        KeyResult::Handled
    }

    /// O, J, I, K, H, Z
    fn utility_tool_shortcuts(&mut self, i: &egui::InputState) -> KeyResult {
        if i.key_pressed(egui::Key::O) {
            self.set_tool(Tool::Blend);
        } else if i.key_pressed(egui::Key::J) {
            self.tool = Tool::Blend;
        } else if i.key_pressed(egui::Key::I) {
            self.tool = Tool::EyeDropper;
        } else if i.key_pressed(egui::Key::K) {
            self.set_tool(Tool::Airbrush);
        } else if i.key_pressed(egui::Key::H) {
            self.tool = Tool::Hand;
        } else if i.key_pressed(egui::Key::Z) {
            self.tool = Tool::Zoom;
        } else {
            return KeyResult::Ignored;
        }
        KeyResult::Handled
    }

    fn lasso_or_lift(&mut self) -> KeyResult {
        if self.selection.is_some() {
            return KeyResult::Action(Action::LiftSelection);
        }
        self.set_tool(Tool::Lasso);
        KeyResult::Handled
    }
}

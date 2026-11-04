// floating.rs — Floating Selection command reducer and undo/redo stack
use crate::history::command::FloatingCommand;
use crate::state::store::{Action, FloatingSelection, TileBuffer};

use super::{App, PixelSnapshot};

impl App {
    /// จัดการการกระทำ (Action Reducer) ตามสถาปัตยกรรมที่กำหนด
    pub(crate) fn dispatch(&mut self, action: Action) {
        match action {
            Action::LiftSelection => self.lift_selection(),
            Action::MoveFloating(delta) => self.move_floating(delta),
            Action::CommitFloating => self.commit_floating(),
            Action::CancelFloating => self.cancel_floating(),
            Action::DeleteFloating => self.delete_floating(),
        }
    }

    fn lift_selection(&mut self) {
        if self.floating.is_some() {
            return;
        }
        let Some(sel) = self.selection else {
            return;
        };
        let li = self.active;
        let target_id = self.layers[li].id;
        let Some((cmd, fl)) = FloatingCommand::create_lift(
            target_id,
            &mut self.layers[li],
            self.w,
            self.h,
            sel.min,
            sel.max,
        ) else {
            return;
        };
        self.floating = Some(fl);
        self.selection = None;
        self.record_command(cmd);
        self.push_hist("หยิบส่วนที่เลือก (Lift)");
    }

    fn move_floating(&mut self, delta: crate::state::store::IVec2) {
        let Some(fl) = &mut self.floating else {
            return;
        };
        fl.origin += delta;
        let cmd = FloatingCommand::Move { delta };
        self.record_command(cmd);
    }

    fn commit_floating(&mut self) {
        let Some(fl) = self.floating.clone() else {
            return;
        };
        let li = self.active;
        let overwritten = self.capture_overwritten(&fl);
        let commit_cmd = FloatingCommand::Commit {
            target_layer: self.layers[li].id,
            floating: fl,
            overwritten_pixels: overwritten,
            commit_origin: self.floating.as_ref().unwrap().origin,
        };
        commit_cmd.apply(&mut self.layers, &mut self.floating, self.w, self.h);
        self.record_command(commit_cmd);
        self.push_hist("รวมส่วนที่ลอย (Commit)");
    }

    fn cancel_floating(&mut self) {
        let Some(fl) = self.floating.clone() else {
            return;
        };
        let cancel_cmd = FloatingCommand::Cancel {
            source_layer: fl.source_layer,
            floating: fl,
        };
        cancel_cmd.apply(&mut self.layers, &mut self.floating, self.w, self.h);
        self.record_command(cancel_cmd);
        self.push_hist("ยกเลิกการลอย (Cancel)");
    }

    fn delete_floating(&mut self) {
        let Some(fl) = self.floating.clone() else {
            return;
        };
        let del_cmd = FloatingCommand::Delete { floating: fl };
        del_cmd.apply(&mut self.layers, &mut self.floating, self.w, self.h);
        self.record_command(del_cmd);
        self.push_hist("ลบส่วนที่ลอย (Delete)");
    }

    /// เก็บพิกเซลเดิมบนเลเยอร์เป้าหมาย ณ ตำแหน่งที่จะถูกทับ (สำหรับ Undo)
    fn capture_overwritten(&self, fl: &FloatingSelection) -> TileBuffer {
        let li = self.active;
        let mut overwritten = TileBuffer::new(fl.pixels.width, fl.pixels.height, (0, 0, 0, 0));
        for dy in 0..fl.pixels.height {
            for dx in 0..fl.pixels.width {
                let y = fl.origin.y + dy as isize;
                let x = fl.origin.x + dx as isize;
                if y >= 0 && (y as usize) < self.h && x >= 0 && (x as usize) < self.w {
                    overwritten.set(dx, dy, self.layers[li].pixels[y as usize][x as usize]);
                }
            }
        }
        overwritten
    }

    fn record_command(&mut self, cmd: FloatingCommand) {
        self.commands.truncate(self.cmd_index);
        self.commands.push(cmd);
        self.cmd_index = self.commands.len();
        self.texture_dirty = true;
    }

    pub(crate) fn undo(&mut self) {
        if self.cmd_index > 0 {
            self.cmd_index -= 1;
            let cmd = self.commands[self.cmd_index].clone();
            cmd.revert(&mut self.layers, &mut self.floating, self.w, self.h);
            self.texture_dirty = true;
            if self.hi > 0 {
                self.hi -= 1;
            }
        } else if let Some(snap) = self.undo_snapshots.pop() {
            let current_pixels = self.layers[snap.layer_idx].pixels.clone();
            self.redo_snapshots.push(PixelSnapshot {
                layer_idx: snap.layer_idx,
                pixels: current_pixels,
            });
            self.layers[snap.layer_idx].pixels = snap.pixels;
            self.texture_dirty = true;
            if self.hi > 0 {
                self.hi -= 1;
            }
            self.saved = "ย้อนกลับเส้นวาด (Undo)".into();
        }
    }

    pub(crate) fn redo(&mut self) {
        if self.cmd_index < self.commands.len() {
            let cmd = self.commands[self.cmd_index].clone();
            cmd.apply(&mut self.layers, &mut self.floating, self.w, self.h);
            self.cmd_index += 1;
            self.texture_dirty = true;
            if self.hi + 1 < self.history_labels.len() {
                self.hi += 1;
            }
        } else if let Some(snap) = self.redo_snapshots.pop() {
            let current_pixels = self.layers[snap.layer_idx].pixels.clone();
            self.undo_snapshots.push(PixelSnapshot {
                layer_idx: snap.layer_idx,
                pixels: current_pixels,
            });
            self.layers[snap.layer_idx].pixels = snap.pixels;
            self.texture_dirty = true;
            if self.hi + 1 < self.history_labels.len() {
                self.hi += 1;
            }
            self.saved = "ทำซ้ำเส้นวาด (Redo)".into();
        }
    }
}

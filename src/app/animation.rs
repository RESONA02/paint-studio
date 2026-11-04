use super::{AnimationFrame, App, AudioTrack, CameraKey};
use crate::state::store::Layer;
use eframe::egui;

impl App {
    pub(crate) fn save_current_cel(&mut self) {
        if let Some(frame) = self.animation_frames.get_mut(self.current_frame) {
            frame.layers = self.layers.clone();
        }
    }

    pub(crate) fn set_current_frame(&mut self, frame: usize) {
        if frame >= self.animation_frames.len() || frame == self.current_frame {
            return;
        }
        self.save_current_cel();
        self.current_frame = frame;
        self.layers = self.animation_frames[frame].layers.clone();
        self.active = self.active.min(self.layers.len().saturating_sub(1));
        self.texture_dirty = true;
    }

    fn add_frame(&mut self, duplicate: bool) {
        self.save_current_cel();
        let mut layers = self.layers.clone();
        if !duplicate {
            for layer in &mut layers {
                if !layer.is_paper {
                    for row in &mut layer.pixels {
                        row.fill((0, 0, 0, 0));
                    }
                }
            }
        }
        let next = self.current_frame + 1;
        self.animation_frames.insert(
            next,
            AnimationFrame {
                layers: layers.clone(),
            },
        );
        self.current_frame = next;
        self.layers = layers;
        self.texture_dirty = true;
    }

    pub(crate) fn animation_tick(&mut self, ctx: &egui::Context) {
        if !self.playback {
            return;
        }
        let now = ctx.input(|i| i.time);
        if now - self.last_tick >= 1.0 / self.fps.clamp(1, 60) as f64 {
            self.last_tick = now;
            if self.current_frame + 1 < self.animation_frames.len() {
                self.set_current_frame(self.current_frame + 1);
            } else if self.loop_playback {
                self.set_current_frame(0);
            } else {
                self.playback = false;
            }
            self.apply_camera_keyframes();
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(4));
    }

    fn apply_camera_keyframes(&mut self) {
        let Some((&left_frame, left)) = self.camera_keys.range(..=self.current_frame).next_back()
        else {
            return;
        };
        let (zoom, pan_x, pan_y, rotation, depth) = if let Some((&right_frame, right)) =
            self.camera_keys.range(self.current_frame..).next()
        {
            if right_frame == left_frame {
                (left.zoom, left.pan_x, left.pan_y, left.rotation, left.depth)
            } else {
                let t =
                    (self.current_frame - left_frame) as f32 / (right_frame - left_frame) as f32;
                (
                    lerp(left.zoom, right.zoom, t),
                    lerp(left.pan_x, right.pan_x, t),
                    lerp(left.pan_y, right.pan_y, t),
                    lerp(left.rotation, right.rotation, t),
                    lerp(left.depth, right.depth, t),
                )
            }
        } else {
            (left.zoom, left.pan_x, left.pan_y, left.rotation, left.depth)
        };
        self.camera_zoom = zoom;
        self.camera_pan = egui::vec2(pan_x, pan_y);
        self.camera_rotation = rotation;
        self.camera_depth = depth;
        self.zoom = zoom;
        self.pan_offset = self.camera_pan;
        self.rotation = rotation;
    }

    pub(crate) fn show_timeline(&mut self, ctx: &egui::Context) {
        let onion_state = (
            self.onion_enabled,
            self.onion_previous_alpha,
            self.onion_next_alpha,
        );
        egui::TopBottomPanel::bottom("animation_timeline")
            .resizable(true)
            .default_height(230.0)
            .min_height(170.0)
            .show(ctx, |ui| {
                ui.add_space(3.0);
                ui.horizontal(|ui| {
                    ui.strong("TIMELINE");
                    if ui.button("⏮").on_hover_text("Previous frame").clicked() {
                        self.set_current_frame(self.current_frame.saturating_sub(1));
                    }
                    if ui
                        .button(if self.playback {
                            "⏸ Pause"
                        } else {
                            "▶ Play"
                        })
                        .clicked()
                    {
                        self.playback = !self.playback;
                        self.last_tick = ctx.input(|i| i.time);
                    }
                    if ui.button("⏭").on_hover_text("Next frame").clicked() {
                        if self.current_frame + 1 == self.animation_frames.len() {
                            self.add_frame(false);
                        } else {
                            self.set_current_frame(self.current_frame + 1);
                        }
                    }
                    ui.checkbox(&mut self.loop_playback, "Loop");
                    ui.add(
                        egui::DragValue::new(&mut self.fps)
                            .range(1..=60)
                            .suffix(" FPS"),
                    );
                    ui.separator();
                    if ui.button("＋ Cel").clicked() {
                        self.add_frame(false);
                    }
                    if ui.button("Duplicate").clicked() {
                        self.add_frame(true);
                    }
                    ui.separator();
                    ui.checkbox(&mut self.onion_enabled, "Onion skin");
                    ui.add_enabled(
                        self.onion_enabled,
                        egui::Slider::new(&mut self.onion_previous_alpha, 0.0..=0.8).text("Prev"),
                    );
                    ui.add_enabled(
                        self.onion_enabled,
                        egui::Slider::new(&mut self.onion_next_alpha, 0.0..=0.8).text("Next"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Frame {} / {}",
                        self.current_frame + 1,
                        self.animation_frames.len()
                    ));
                    ui.separator();
                    ui.label("Layer / Cel");
                    egui::ScrollArea::horizontal()
                        .id_salt("timeline_frames")
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(72.0)
                                .show(ui, |ui| {
                                    let track_names = self
                                        .layers
                                        .iter()
                                        .map(|l| l.name.clone())
                                        .collect::<Vec<_>>();
                                    for (track, name) in track_names.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.add_sized(
                                                [115.0, 24.0],
                                                egui::Label::new(format!("▤ {}", name)),
                                            );
                                            for frame in 0..self.animation_frames.len() {
                                                let selected = frame == self.current_frame;
                                                let filled = self.animation_frames[frame]
                                                    .layers
                                                    .get(track)
                                                    .is_some_and(layer_has_pixels);
                                                let text = if self.camera_keys.contains_key(&frame)
                                                {
                                                    "•"
                                                } else if filled {
                                                    "◆"
                                                } else {
                                                    "◇"
                                                };
                                                if ui
                                                    .add_sized(
                                                        [42.0, 24.0],
                                                        egui::Button::new(format!(
                                                            "{} {}",
                                                            text,
                                                            frame + 1
                                                        ))
                                                        .selected(selected),
                                                    )
                                                    .clicked()
                                                {
                                                    self.set_current_frame(frame);
                                                }
                                            }
                                        });
                                    }
                                });
                        });
                });

                ui.collapsing(
                    "2D Camera Folder  |  zoom • pan • rotation • parallax",
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Slider::new(&mut self.camera_zoom, 0.1..=4.0).text("Zoom"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.camera_pan.x, -1000.0..=1000.0)
                                    .text("Pan X"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.camera_pan.y, -1000.0..=1000.0)
                                    .text("Pan Y"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.camera_rotation, -180.0..=180.0)
                                    .text("Rotate"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.camera_depth, -100.0..=100.0)
                                    .text("Depth"),
                            );
                            if ui.button("◆ Keyframe").clicked() {
                                self.camera_keys.insert(
                                    self.current_frame,
                                    CameraKey {
                                        zoom: self.camera_zoom,
                                        pan_x: self.camera_pan.x,
                                        pan_y: self.camera_pan.y,
                                        rotation: self.camera_rotation,
                                        depth: self.camera_depth,
                                    },
                                );
                            }
                            ui.label(format!("{} keys", self.camera_keys.len()));
                        });
                        self.zoom = self.camera_zoom;
                        self.pan_offset = self.camera_pan;
                        self.rotation = self.camera_rotation;
                    },
                );

                ui.collapsing("Audio tracks  |  voice and music", |ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.audio_name_input);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.audio_path_input)
                                .desired_width(260.0)
                                .hint_text("audio file path"),
                        );
                        if ui.button("＋ Track").clicked()
                            && !self.audio_path_input.trim().is_empty()
                        {
                            let name = if self.audio_name_input.trim().is_empty() {
                                "Audio".to_owned()
                            } else {
                                self.audio_name_input.trim().to_owned()
                            };
                            self.audio_tracks.push(AudioTrack {
                                name,
                                path: self.audio_path_input.trim().to_owned(),
                                gain: 1.0,
                            });
                            self.audio_name_input.clear();
                            self.audio_path_input.clear();
                        }
                    });
                    for track in &mut self.audio_tracks {
                        ui.horizontal(|ui| {
                            ui.label(&track.name);
                            ui.label(&track.path);
                            ui.add(egui::Slider::new(&mut track.gain, 0.0..=2.0).text("Gain"));
                        });
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Export PNG Sequence").clicked() {
                        self.save_current_cel();
                        match self.export_image_sequence("shots/sequence") {
                            Ok(()) => self.saved = "PNG sequence exported to shots/sequence".into(),
                            Err(e) => self.saved = e,
                        }
                    }
                    if ui.button("OpenToonz PNG Sequence").clicked() {
                        self.save_current_cel();
                        match self.export_opentoonz_sequence("shots/opentoonz") {
                            Ok(()) => {
                                self.saved =
                                    "OpenToonz import sequence exported to shots/opentoonz".into()
                            }
                            Err(e) => self.saved = e,
                        }
                    }
                });
            });
        if onion_state
            != (
                self.onion_enabled,
                self.onion_previous_alpha,
                self.onion_next_alpha,
            )
        {
            self.texture_dirty = true;
        }
    }

    fn export_image_sequence(&self, directory: &str) -> Result<(), String> {
        let root = std::path::Path::new(directory);
        std::fs::create_dir_all(root).map_err(|e| e.to_string())?;
        for (i, frame) in self.animation_frames.iter().enumerate() {
            let image = composite_frame(&frame.layers, self.w, self.h);
            image
                .save(root.join(format!("frame_{:04}.png", i + 1)))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn export_opentoonz_sequence(&self, directory: &str) -> Result<(), String> {
        let root = std::path::Path::new(directory);
        std::fs::create_dir_all(root).map_err(|e| e.to_string())?;
        for (i, frame) in self.animation_frames.iter().enumerate() {
            let filename = format!("animation.{:04}.png", i + 1);
            composite_frame(&frame.layers, self.w, self.h)
                .save(root.join(&filename))
                .map_err(|e| e.to_string())?;
        }
        let audio = self
            .audio_tracks
            .iter()
            .map(|t| format!("# audio: {} | {} | gain={:.2}\n", t.name, t.path, t.gain))
            .collect::<String>();
        let manifest = format!("OpenToonz image sequence import\nFrame range: 1-{}\nFrame rate: {} fps\nCanvas: {}x{}\nImport animation.0001.png as an image sequence in OpenToonz.\nCamera keys: {}\n{}", self.animation_frames.len(), self.fps, self.w, self.h, self.camera_keys.len(), audio);
        std::fs::write(root.join("README.txt"), manifest).map_err(|e| e.to_string())
    }
}

fn layer_has_pixels(layer: &Layer) -> bool {
    !layer.is_paper && layer.pixels.iter().any(|row| row.iter().any(|p| p.3 > 0))
}

fn composite_frame(layers: &[Layer], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let mut color = [0u8; 4];
            for layer in layers.iter().filter(|l| l.visible && !l.is_paper) {
                let (r, g, b, a) = layer.pixels[y][x];
                let alpha = (a as f32 / 255.0) * layer.opacity_f32();
                let dst_alpha = color[3] as f32 / 255.0;
                let out_alpha = alpha + dst_alpha * (1.0 - alpha);
                if out_alpha > 0.0 {
                    color[0] = ((r as f32 * alpha + color[0] as f32 * dst_alpha * (1.0 - alpha))
                        / out_alpha) as u8;
                    color[1] = ((g as f32 * alpha + color[1] as f32 * dst_alpha * (1.0 - alpha))
                        / out_alpha) as u8;
                    color[2] = ((b as f32 * alpha + color[2] as f32 * dst_alpha * (1.0 - alpha))
                        / out_alpha) as u8;
                    color[3] = (out_alpha * 255.0) as u8;
                }
            }
            image.put_pixel(x as u32, y as u32, image::Rgba(color));
        }
    }
    image
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// app/mod.rs — App state, lifecycle and small shared helpers
use crate::history::command::FloatingCommand;
use crate::state::store::{FloatingSelection, IVec2, Layer, LayerId, SelectionRect, TileBuffer};
use crate::tools::{SubTool, SubToolDef, Tool};
use eframe::egui;

pub mod animation;
pub mod canvas;
pub mod comic;
pub mod floating;
pub mod layers;
pub mod lt;
pub mod pages;
pub mod production;
pub mod project;
pub mod selection;

use comic::{Balloon, ComicFrame};
use pages::{Binding, PageDoc};

#[derive(Default)]
pub struct App {
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) layers: Vec<Layer>,
    pub(crate) active: usize,
    pub(crate) tool: Tool,
    pub(crate) subtool: SubTool,
    pub(crate) main_color: egui::Color32,
    pub(crate) sub_color: egui::Color32,
    pub(crate) is_transparent: bool,
    pub(crate) size: f32,
    pub(crate) opacity: f32,
    pub(crate) hardness: f32,
    pub(crate) drawing: bool,
    pub(crate) last: Option<(isize, isize)>,
    pub(crate) history_labels: Vec<String>,
    pub(crate) hi: usize,
    pub(crate) zoom: f32,
    pub(crate) pan_offset: egui::Vec2,
    pub(crate) rotation: f32,
    pub(crate) saved: String,
    pub(crate) screenshot_path: Option<String>,
    pub(crate) frame_count: u32,
    pub(crate) screenshot_requested: bool,
    pub(crate) font_loaded: bool,
    pub(crate) texture: Option<egui::TextureHandle>,
    pub(crate) texture_dirty: bool,
    // Contextual Modifier Key states (CSP DefaultToolModifyKey)
    pub(crate) space_held: bool,
    pub(crate) alt_held: bool,
    pub(crate) shift_held: bool,
    // Floating Selection & Selection State
    pub(crate) selection: Option<SelectionRect>,
    pub(crate) selecting: bool,
    pub(crate) selection_start: Option<IVec2>,
    pub(crate) floating: Option<FloatingSelection>,
    pub(crate) commands: Vec<FloatingCommand>,
    pub(crate) cmd_index: usize,
    // View & Panels Toggle
    pub(crate) hide_all_ui: bool,
    pub(crate) hide_side_panels: bool,
    pub(crate) show_shortcut_dialog: bool,
    pub(crate) shortcut_category: usize,
    pub(crate) clipboard_buffer: Option<TileBuffer>,
    pub(crate) animation_frames: Vec<AnimationFrame>,
    pub(crate) current_frame: usize,
    pub(crate) playback: bool,
    pub(crate) loop_playback: bool,
    pub(crate) fps: u32,
    pub(crate) last_tick: f64,
    pub(crate) onion_enabled: bool,
    pub(crate) onion_previous_alpha: f32,
    pub(crate) onion_next_alpha: f32,
    pub(crate) camera_keys: std::collections::BTreeMap<usize, CameraKey>,
    pub(crate) camera_zoom: f32,
    pub(crate) camera_pan: egui::Vec2,
    pub(crate) camera_rotation: f32,
    pub(crate) camera_depth: f32,
    pub(crate) audio_tracks: Vec<AudioTrack>,
    pub(crate) audio_name_input: String,
    pub(crate) audio_path_input: String,
    // Phase 2: Artist Tools & Realtime Workflows
    pub(crate) symmetry_enabled: bool,
    pub(crate) symmetry_axis_x: isize,
    pub(crate) lasso_points: Vec<IVec2>,
    pub(crate) undo_snapshots: Vec<PixelSnapshot>,
    pub(crate) redo_snapshots: Vec<PixelSnapshot>,
    pub(crate) hovered_canvas_pixel: Option<(isize, isize)>,
    // Phase 4: Comic & Multi-Page EX
    pub(crate) pages: Vec<PageDoc>,
    pub(crate) page_index: usize,
    pub(crate) binding: Binding,
    pub(crate) page_marks: std::collections::BTreeSet<usize>,
    pub(crate) page_manager_open: bool,
    pub(crate) story_editor_open: bool,
    pub(crate) lt_dialog_open: bool,
    pub(crate) story_target: usize,
    pub(crate) story_batch: String,
    pub(crate) story_find: String,
    pub(crate) story_replace: String,
    pub(crate) story_status: String,
    pub(crate) lt_threshold: u8,
    pub(crate) lt_levels: u8,
    pub(crate) lt_dot: u8,
    pub(crate) comic_frames: Vec<ComicFrame>,
    pub(crate) balloons: Vec<Balloon>,
    pub(crate) selected_balloon: Option<usize>,
    pub(crate) frame_start: Option<IVec2>,
    pub(crate) frame_current: Option<IVec2>,
    pub(crate) balloon_start: Option<IVec2>,
    pub(crate) production_panel_open: bool,
    #[allow(dead_code)]
    pub(crate) bleed_mm: f32,
    #[allow(dead_code)]
    pub(crate) page_lock: Option<PageLock>,
    // Warm Ink UI Tokens & States
    pub(crate) is_dark_theme: bool,
    pub(crate) show_tool_names: bool,
    pub(crate) ui_scale: f32,
    pub(crate) show_start_screen: bool,
    pub(crate) show_icon_preview: bool,
    pub(crate) active_warm_tool: usize,
    pub(crate) subtool_popup_tool: Option<usize>,
}

pub(crate) struct PageLock {
    path: std::path::PathBuf,
}
impl Drop for PageLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

impl App {
    pub fn new(screenshot_path: Option<String>) -> Self {
        let (w, h) = (1463usize, 823usize);
        let layers = build_initial_layers(w, h);
        let mut app = Self {
            w,
            h,
            layers: layers.clone(),
            active: 1,
            size: 10.0_f32,
            opacity: 1.0_f32,
            hardness: 1.0_f32,
            main_color: egui::Color32::BLACK,
            sub_color: egui::Color32::WHITE,
            screenshot_path,
            texture_dirty: true,
            animation_frames: vec![AnimationFrame {
                layers: layers.clone(),
            }],
            pages: vec![PageDoc::new_page("หน้า 1", w, h, layers)],
            bleed_mm: 3.0,
            binding: Binding::LeftToRight,
            symmetry_axis_x: (w / 2) as isize,
            fps: 24,
            loop_playback: true,
            onion_previous_alpha: 0.22,
            onion_next_alpha: 0.18,
            lt_threshold: 32,
            lt_levels: 4,
            lt_dot: 2,
            is_dark_theme: true,
            ui_scale: 1.0,
            ..Self::default()
        };
        app.push_hist("เปิดภาพวาดใหม่");
        app.renumber_folios();
        match app.acquire_page_lock(app.page_index) {
            Ok(lock) => app.page_lock = Some(lock),
            Err(error) => app.saved = error,
        }
        app
    }

    pub(crate) fn push_hist(&mut self, s: &str) {
        self.history_labels.truncate(self.hi + 1);
        self.history_labels.push(s.into());
        self.hi = self.history_labels.len() - 1;
    }

    pub(crate) fn select_subtool(&mut self, def: SubToolDef) {
        self.subtool = def.subtool;
        self.size = def.default_size;
        self.opacity = def.default_opacity;
        self.hardness = def.hardness;
    }

    /// เลือกเครื่องมือหลักแล้วตั้งค่าเครื่องมือย่อยตัวแรกให้อัตโนมัติ
    pub(crate) fn set_tool(&mut self, tool: Tool) {
        self.tool = tool;
        self.reset_subtool_to_first();
    }

    pub(crate) fn reset_subtool_to_first(&mut self) {
        if let Some(def) = self.tool.subtools().first().copied() {
            self.select_subtool(def);
        }
    }

    pub(crate) fn canvas_pos(&self, p: egui::Pos2, rect: egui::Rect) -> (isize, isize) {
        let x = (p.x - rect.min.x) / self.zoom;
        let y = (p.y - rect.min.y) / self.zoom;
        (x as isize, y as isize)
    }

    /// เปิดหน้าต่างจาก command line (ใช้สำหรับเดโม/ทดสอบ)
    pub fn open_panel(&mut self, name: &str) {
        match name {
            "start" => self.show_start_screen = true,
            "icons" => self.show_icon_preview = true,
            "light" => self.is_dark_theme = false,
            "pages" => self.page_manager_open = true,
            "story" => self.story_editor_open = true,
            "lt" => self.lt_dialog_open = true,
            "comic" => self.set_tool(Tool::ComicFrame),
            _ => {}
        }
    }
}

fn build_initial_layers(w: usize, h: usize) -> Vec<Layer> {
    let paper = Layer::new(LayerId(0), "กระดาษ".to_string(), w, h, true);
    let mut layer1 = Layer::new(LayerId(1), "เลเยอร์ 1".to_string(), w, h, false);

    if !load_reference_art(&mut layer1, w, h) {
        draw_placeholder_art(&mut layer1, w, h);
    }

    vec![paper, layer1]
}

// โหลดรูปภาพอ้างอิงถ้ามี
fn load_reference_art(layer: &mut Layer, w: usize, h: usize) -> bool {
    for path in [
        "reference/picture-1.jpg",
        "../reference/picture-1.jpg",
        "attachments/picture-1.png",
    ] {
        if let Ok(img) = image::open(path) {
            let resized =
                img.resize_exact(w as u32, h as u32, image::imageops::FilterType::Triangle);
            let rgb = resized.to_rgb8();
            for y in 0..h {
                for x in 0..w {
                    let p = rgb.get_pixel(x as u32, y as u32);
                    if p[0] < 230 || p[1] < 230 || p[2] < 230 {
                        let lum = ((p[0] as u32 * 299 + p[1] as u32 * 587 + p[2] as u32 * 114)
                            / 1000) as u8;
                        let alpha = 255u8.saturating_sub(lum);
                        if alpha > 15 {
                            layer.pixels[y][x] = (p[0], p[1], p[2], alpha);
                        }
                    }
                }
            }
            return true;
        }
    }
    false
}

fn draw_placeholder_art(layer: &mut Layer, w: usize, h: usize) {
    let cx = w as isize / 2;
    let cy = h as isize / 2;
    let ink = (40u8, 40u8, 45u8, 240u8);

    for y_rel in -200isize..200 {
        let y = cy + y_rel;
        let width = 60.0_f32 + (y_rel as f32 * 0.05_f32).sin().abs() * 40.0_f32;
        for dx in [-(width as isize), width as isize] {
            let x = cx + dx;
            if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
                layer.pixels[y as usize][x as usize] = ink;
                if (x + 1) < w as isize {
                    layer.pixels[y as usize][(x + 1) as usize] = ink;
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct PixelSnapshot {
    pub(crate) layer_idx: usize,
    pub(crate) pixels: Vec<Vec<(u8, u8, u8, u8)>>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct LayerComp {
    pub(crate) name: String,
    pub(crate) visibility: Vec<bool>,
}

#[derive(Clone)]
pub(crate) struct AnimationFrame {
    pub(crate) layers: Vec<Layer>,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct CameraKey {
    pub(crate) zoom: f32,
    pub(crate) pan_x: f32,
    pub(crate) pan_y: f32,
    pub(crate) rotation: f32,
    pub(crate) depth: f32,
}

#[derive(Clone)]
pub(crate) struct AudioTrack {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) gain: f32,
}

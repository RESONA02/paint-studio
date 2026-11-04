// ui/mod.rs — eframe entry point and per-frame orchestration
mod canvas_view;
mod comic_panel;
mod dialog;
mod icons;
mod lt_dialog;
mod menu;
mod page_manager;
mod right_dock;
mod shortcuts;
mod start_screen;
mod story_editor;
mod theme;
mod toolbar;
mod tools_panel;
use icons as warm_ink_icons;
mod widgets;

use crate::app::canvas::ensure_parent_dir;
use crate::app::App;
use eframe::egui;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_style(ctx);
        self.handle_keyboard(ctx);
        self.animation_tick(ctx);
        if self.texture_dirty {
            self.save_current_cel();
        }
        self.update_texture(ctx);
        self.run_screenshot_hooks(ctx);

        if !self.hide_all_ui {
            menu::show(self, ctx);
            self.show_command_bar(ctx);
            self.show_document_tabs(ctx);
        }

        if !self.hide_all_ui && !self.hide_side_panels {
            self.show_tool_strip(ctx);
            self.show_right_dock(ctx);
        }

        if !self.hide_all_ui {
            self.show_timeline(ctx);
            self.show_production_panel(ctx);
        }
        self.show_canvas(ctx);

        if self.show_shortcut_dialog {
            self.shortcut_dialog(ctx);
        }
        let theme = if self.is_dark_theme {
            theme::Theme::dark()
        } else {
            theme::Theme::light()
        };
        theme.apply_to_visuals(ctx);
        self.show_start_screen_modal(ctx, &theme);
        self.show_icon_preview_window(ctx, &theme);
        if self.page_manager_open {
            self.page_manager_window(ctx);
        }
        if self.story_editor_open {
            self.story_editor_window(ctx);
        }
        if self.lt_dialog_open {
            self.lt_dialog_window(ctx);
        }
        self.comic_properties_window(ctx);
    }
}

impl App {
    fn init_style(&mut self, ctx: &egui::Context) {
        if self.font_loaded {
            return;
        }
        self.font_loaded = true;
        install_fonts(ctx);
        let theme = if self.is_dark_theme {
            theme::Theme::dark()
        } else {
            theme::Theme::light()
        };
        theme.apply_to_visuals(ctx);
    }

    fn run_screenshot_hooks(&mut self, ctx: &egui::Context) {
        if self.screenshot_path.is_some() {
            ctx.request_repaint();
            self.frame_count += 1;
            if self.frame_count >= 5 && !self.screenshot_requested {
                self.screenshot_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
        }
        self.save_viewport_screenshot(ctx);
    }

    fn save_viewport_screenshot(&mut self, ctx: &egui::Context) {
        for event in ctx.input(|i| i.raw.events.clone()) {
            let egui::Event::Screenshot { image, .. } = event else {
                continue;
            };
            if let Some(path) = &self.screenshot_path {
                self.write_screenshot(ctx, path, &image);
            }
        }
    }

    fn write_screenshot(&self, ctx: &egui::Context, path: &str, image: &egui::ColorImage) {
        ensure_parent_dir(std::path::Path::new(path));
        image::save_buffer(
            path,
            image.as_raw(),
            image.size[0] as u32,
            image.size[1] as u32,
            image::ColorType::Rgba8,
        )
        .expect("failed to save screenshot");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_paths = [
        "C:\\Windows\\Fonts\\LeelawUI.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
        "C:\\Windows\\Fonts\\seguiemj.ttf",
    ];
    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            let name = std::path::Path::new(path)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            fonts.font_data.insert(
                name.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data)),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push(name.clone());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push(name);
        }
    }
    ctx.set_fonts(fonts);
}

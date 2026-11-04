// menu/mod.rs — menu bar layout (Warm Ink Design System)
mod comic;
mod edit;
mod file;
mod layer;
mod misc;
mod select;
mod view;

use super::theme::Theme;
use crate::app::App;
use eframe::egui;

pub(crate) fn show(app: &mut App, ctx: &egui::Context) {
    let theme = if app.is_dark_theme {
        Theme::dark()
    } else {
        Theme::light()
    };

    egui::TopBottomPanel::top("menu_bar")
        .frame(
            egui::Frame::NONE
                .fill(theme.bg_panel_header)
                .inner_margin(egui::Margin::symmetric(8, 3))
                .stroke(egui::Stroke::new(1.0, theme.border)),
        )
        .show(ctx, |ui| menu_row(app, ui, &theme));
}

fn menu_row(app: &mut App, ui: &mut egui::Ui, _theme: &Theme) {
    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 0.0);

        file::show(ui, app);
        edit::show(ui, app);
        misc::animation(ui);
        layer::show(ui, app);
        select::show(ui, app);
        view::show(ui, app);
        comic::show(ui, app);
        misc::filter(ui, app);
        misc::window(ui, app);
        misc::help(ui);
        misc::status_text(ui);
    });
}

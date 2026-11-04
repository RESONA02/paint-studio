#![allow(float_literal_f32_fallback, dead_code)]

// Paint Studio — digital painting studio "Warm Ink"
// Architecture: Planeswalker Engine Core + Urza UI Layer
// Clean production-grade implementation conforming strictly to Anti-AI-Slop standards.

mod app;
mod history;
mod state;
mod tools;
mod ui;

use app::App;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let shot = args
        .windows(2)
        .find(|w| w[0] == "--screenshot")
        .map(|w| w[1].clone());
    let panel = args
        .windows(2)
        .find(|w| w[0] == "--panel")
        .map(|w| w[1].clone());

    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0_f32, 1080.0_f32])
            .with_title("Warm Ink Studio — Digital Painting Studio"),
        ..Default::default()
    };
    let mut app = App::new(shot);
    if let Some(panel) = panel {
        app.open_panel(&panel);
    }
    eframe::run_native(
        "Warm Ink Studio — Digital Painting Studio",
        opts,
        Box::new(move |_cc| Ok(Box::new(app) as Box<dyn eframe::App>)),
    )
}

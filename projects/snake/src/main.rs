#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::{
    egui::{self, Layout},
    epaint::{Color32, Pos2, Rect, RectShape, Rounding, Vec2},
};
mod core;

use core::Game;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(540.0, 600.0)),

        ..Default::default()
    };

    // Our application state:

    let mut game = Game::new();
    let mut name = "Arthur".to_owned();
    let mut level: u64 = 1;
    const CELL_SIZE: f32 = 20.0;

    //
    eframe::run_simple_native("Snake Pro", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut level, 1..=10).text("Level"));
                    if ui.button("Click each year").clicked() {
                        level += 1;
                    }
                    ui.label(format!("Hello '{name}', Level {level}"));
                });
                game.set_level(level);
                game.render(ui);
            });
        });
    })
}

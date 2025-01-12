#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui::{self, FontDefinitions, FontId};
mod settings;
use settings::Settings;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Searcher",
        options,
        Box::new(|_cc| {

            _cc.egui_ctx.set_pixels_per_point(1.2);

            Ok(Box::<App>::new(App::default()))
        }),
    )
}

#[derive(Default)]
struct App {
    settings: Settings,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.settings.update(ui);
        });
    }
}

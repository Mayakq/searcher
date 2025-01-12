#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };
    let app = App {};
    eframe::run_native(
        "Searcher",
        options,
        Box::new(|_cc| Ok(Box::<App>::new(app))),
    )
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

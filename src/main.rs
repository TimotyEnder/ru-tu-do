mod turing_machine;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 520.0])
            .with_title("Ru-Tu-Do"),
        ..Default::default()
    };

    eframe::run_native(
        "Ru-Tu-Do",
        options,
        Box::new(|_cc| Ok(Box::new(RuToDoUI::default()))),
    )
}

// ── App state ────────────────────────────────────────────────────────────────

#[derive(Default)]
struct RuToDoUI {}

// ── eframe::App impl ─────────────────────────────────────────────────────────

impl eframe::App for RuToDoUI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Toggle theme
        ui.ctx().set_visuals(egui::Visuals::dark());
        egui::Panel::top("top_panel")
            .resizable(false)
            .min_size(32.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Ru-Tu-Do");
                });
            });
        egui::Panel::left("tape_panel")
            .resizable(false)
            .min_size(32.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Ru-Tu-Do");
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Ru-Tu-Do");
            });
        });
    }
}

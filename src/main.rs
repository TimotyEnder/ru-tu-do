mod turing_machine;

use eframe::egui;
use egui::{ahash::random_state::set_random_source, response};

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

struct RuToDoUI {
    from_transition_field: String,
    to_transition_field: String,
}
impl Default for RuToDoUI {
    fn default() -> Self {
        return Self {
            from_transition_field: String::from("Nowhere"),
            to_transition_field: String::from("Nowhere"),
        };
    }
}

// ── eframe::App impl ─────────────────────────────────────────────────────────

impl eframe::App for RuToDoUI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Toggle theme
        ui.ctx().set_visuals(egui::Visuals::dark());
        egui::Panel::top("top_panel")
            .resizable(false)
            .min_size(32.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add Vertex").clicked() {
                        // handle click
                    }
                    ui.label("from:");
                    let response_from =
                        ui.add(egui::TextEdit::singleline(&mut self.from_transition_field));
                    ui.label("to:");
                    let response_to =
                        ui.add(egui::TextEdit::singleline(&mut self.to_transition_field));
                });
                // Need to return a Response here too
                ui.label("") // Dummy response, or use something meaningful
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

mod turing_machine;

use core::num;

use eframe::egui;
use egui::{Vec2, ahash::random_state::set_random_source, response};

use crate::turing_machine::TuringMachine;

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
    write_transition_field: String,
    accept_transition_field: String,
    machine: TuringMachine,
    transition_move_opt: String,
}
impl Default for RuToDoUI {
    fn default() -> Self {
        return Self {
            from_transition_field: String::new(),
            to_transition_field: String::new(),
            write_transition_field: String::new(),
            accept_transition_field: String::new(),
            machine: TuringMachine::new_default(),
            transition_move_opt: String::from("Left"),
        };
    }
}

// ── eframe::App impl ─────────────────────────────────────────────────────────

impl eframe::App for RuToDoUI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Toggle theme
        ui.ctx().set_visuals(egui::Visuals::dark());
        egui::Panel::top("top_panel")
            .frame(egui::Frame::NONE.inner_margin(10.0))
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    let button =
                        egui::Button::new("Add Vector").min_size(egui::Vec2::new(50.0, 50.0));

                    if ui.add(button).clicked() {
                        // Handle click
                    }
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("from:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.from_transition_field)
                                        .hint_text("e.g., q0")
                                        .desired_width(50.0), // Shows when field is empty
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("to:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.to_transition_field)
                                        .hint_text("e.g., q1")
                                        .desired_width(50.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("accept:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.accept_transition_field)
                                        .hint_text("eg. A")
                                        .desired_width(50.0),
                                );
                            });
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("write:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.write_transition_field)
                                        .hint_text("eg. A")
                                        .desired_width(50.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("move:");
                                egui::ComboBox::from_label("")
                                    .selected_text(&self.transition_move_opt) // Pass as &str
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.transition_move_opt,
                                            "Left".to_string(),
                                            "Left",
                                        );
                                        ui.selectable_value(
                                            &mut self.transition_move_opt,
                                            "Right".to_string(),
                                            "Right",
                                        );
                                    });
                            });
                            if ui.button("Add Transition").clicked() {
                                if let (Ok(from), Ok(to)) = (
                                    self.from_transition_field.parse::<usize>(),
                                    self.to_transition_field.parse::<usize>(),
                                ) {
                                    self.machine.add_transition(
                                        from,
                                        to,
                                        &self.write_transition_field,
                                        &self.accept_transition_field,
                                        &self.transition_move_opt,
                                    );
                                }
                            }
                        });
                    });
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

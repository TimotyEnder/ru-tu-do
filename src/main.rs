mod turing_machine;

use core::num;

use eframe::egui;
use egui::{Vec2, ahash::random_state::set_random_source, response};

use crate::turing_machine::{TuringMachine, TuringTape};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([800.0, 600.0])
            .with_inner_size([800.0, 600.0])
            .with_resizable(true) // This makes window non-resizable
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
    transition_move_opt: String,
    //
    machine: TuringMachine,
    //
    error_popup_txt: String,
    error_popus_shown: bool,
    //
    popup_title: String,
    popup_text: String,
    popup_shown: bool,
    //
    string_to_process: String,
    tape: TuringTape,
    current_state_index: usize,
}
impl Default for RuToDoUI {
    fn default() -> Self {
        return Self {
            machine: TuringMachine::new_default(),

            from_transition_field: String::new(),
            to_transition_field: String::new(),
            write_transition_field: String::new(),
            accept_transition_field: String::new(),
            transition_move_opt: String::from("Left"),
            string_to_process: String::new(),

            error_popup_txt: String::new(),
            error_popus_shown: false,

            popup_shown: false,
            popup_text: String::new(),
            popup_title: String::new(),
            tape: TuringTape::from_string_input(""),
            current_state_index: 0,
        };
    }
}

// ── eframe::App impl and UI helper functions ─────────────────────────────────────────────────────────
impl RuToDoUI {
    fn error_popup(&mut self, ctx: &egui::Context) {
        if self.error_popus_shown {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .fixed_size([200.0, 100.0])
                .show(ctx, |ui| {
                    ui.label(&*self.error_popup_txt);
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        if ui.button("OK").clicked() {
                            self.error_popus_shown = false;
                        }
                    });
                });
        }
    }
    fn popup(&mut self, ctx: &egui::Context) {
        if self.popup_shown {
            egui::Window::new(&self.popup_title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
                .fixed_size([100.0, 50.0])
                .show(ctx, |ui| {
                    ui.label(&*self.popup_text);
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        if ui.button("OK").clicked() {
                            self.popup_shown = false;
                        }
                    });
                });
        }
    }
    fn show_error_popup(&mut self, message: &str) {
        self.error_popup_txt = String::from(message);
        self.error_popus_shown = true;
    }
    fn show_popup(&mut self, message: &str, title: &str) {
        self.popup_text = String::from(message);
        self.popup_title = String::from(title);
        self.popup_shown = true;
    }
}
impl eframe::App for RuToDoUI {
    // ── Top Frame ─────────────────────────────────────────────────────────
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());
        // Create a custom style with larger fonts
        let screen_width = ui.ctx().viewport_rect().width();
        let screen_height = ui.ctx().viewport_rect().height();

        // Calculate base font size based on screen dimensions
        let base_font_size = (screen_width.min(screen_height) * 0.01).clamp(10.0, 32.0);

        let mut style = (*ui.ctx().global_style()).clone();
        style
            .text_styles
            .get_mut(&egui::TextStyle::Body)
            .unwrap()
            .size = base_font_size;
        style
            .text_styles
            .get_mut(&egui::TextStyle::Button)
            .unwrap()
            .size = base_font_size;
        style
            .text_styles
            .get_mut(&egui::TextStyle::Heading)
            .unwrap()
            .size = base_font_size * 1.5;
        ui.ctx().set_global_style(style);
        egui::Panel::top("top_panel")
            .frame(egui::Frame::NONE.inner_margin(10.0))
            .resizable(false)
            .show_inside(ui, |ui| {
                self.error_popup(ui.ctx());
                self.popup(ui.ctx());
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.set_width(ui.max_rect().width() * 0.10);
                        let vertex_create_button = egui::Button::new("Add Vertex").min_size(
                            egui::Vec2::new(ui.available_width(), ui.max_rect().height() * 3.5),
                        );

                        if ui.add(vertex_create_button).clicked() {
                            self.machine.add_vertex_button_handler();
                            self.show_popup("Vertex Created", "Success");
                        }
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.set_width(ui.max_rect().width() * 0.25);
                            ui.horizontal(|ui| {
                                ui.label("from:");

                                ui.add(
                                    egui::TextEdit::singleline(&mut self.from_transition_field)
                                        .hint_text("e.g., q0")
                                        .desired_width(20.0), // Shows when field is empty
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
                                ui.label("read:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.accept_transition_field)
                                        .hint_text("eg. A")
                                        .desired_width(70.0),
                                );
                            });
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.set_width(ui.max_rect().width() * 0.25);
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
                                    .selected_text(&self.transition_move_opt)
                                    .width(30.0)
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
                            let add_transition_button = egui::Button::new("Add Transition")
                                .min_size(egui::Vec2::new(
                                    ui.available_width(),
                                    ui.available_height(),
                                ));
                            if ui.add(add_transition_button).clicked() {
                                if let (Ok(from), Ok(to)) = (
                                    self.from_transition_field.parse::<usize>(),
                                    self.to_transition_field.parse::<usize>(),
                                ) && !self.write_transition_field.is_empty()
                                    && !self.accept_transition_field.is_empty()
                                    && self
                                        .machine
                                        .add_transition(
                                            from,
                                            to,
                                            &self.write_transition_field,
                                            &self.accept_transition_field,
                                            &self.transition_move_opt,
                                        )
                                        .is_ok()
                                {
                                    self.show_popup("Transition created", "Success");
                                } else {
                                    self.show_error_popup("Error creating transition!");
                                    self.to_transition_field.clear();
                                    self.from_transition_field.clear();
                                }
                            }
                        });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("String to process");
                        ui.set_width(ui.max_rect().width() * 0.25);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.string_to_process)
                                .hint_text("eg. ABCD")
                                .desired_width(ui.available_width()),
                        );
                        let string_process_button = egui::Button::new("Process")
                            .min_size(egui::Vec2::new(ui.available_width(), 25.0));

                        if ui.add(string_process_button).clicked() {
                            if !self.string_to_process.is_empty() {
                                self.tape = TuringTape::from_string_input(&self.string_to_process);
                                self.show_popup("string processed", "Success");
                            }
                        }
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        let step_button = egui::Button::new("Step");
                        if ui
                            .add_sized(
                                [ui.available_width(), ui.available_height() / 2.0],
                                step_button,
                            )
                            .clicked()
                        {
                            self.machine
                                .step(&mut self.tape, &mut self.current_state_index);
                        }
                        ui.end_row();
                        let reset_button = egui::Button::new("Reset Tape");
                        if ui
                            .add_sized([ui.available_width(), ui.available_height()], reset_button)
                            .clicked()
                        {
                            self.tape = TuringTape::from_string_input("");
                            self.show_popup("Tape Reset", "Success");
                        }
                    });
                });
                ui.label("")
            });
        // ── Tape Side Panel ─────────────────────────────────────────────────────────
        egui::Panel::left("tape_panel")
            .resizable(false)
            .min_size(ui.max_rect().width() * 0.2) // Set a sensible minimum width
            .show_inside(ui, |ui| {
                let total_height = ui.available_height();
                let num_cells = self.tape.tape.len();
                let cell_height = total_height / num_cells as f32; // Minimum 30px height

                ui.vertical_centered(|ui| {
                    egui::Grid::new("tape_grid")
                        .num_columns(1)
                        .striped(true)
                        .show(ui, |ui| {
                            for (it, tape_cell) in self.tape.tape.iter().enumerate() {
                                // Make each cell fill the full width
                                let available_width = ui.available_width();

                                // Create a button-like cell that fills the space
                                let response = ui.add_sized(
                                    [available_width, cell_height],
                                    egui::Button::new(format!("{} {}", tape_cell, {
                                        if it == self.tape.current_cell_index() {
                                            "<-"
                                        } else {
                                            ""
                                        }
                                    }))
                                    .frame(true)
                                    .fill(
                                        if it == self.tape.current_cell_index() {
                                            egui::Color32::from_rgb(124, 135, 238) // Blue for current cell
                                        } else if it % 2 == 0 {
                                            egui::Color32::from_rgb(60, 60, 60) // Dark gray for even rows
                                        } else {
                                            egui::Color32::from_rgb(40, 40, 40) // Lighter gray for odd rows
                                        },
                                    ),
                                );

                                ui.end_row();
                            }
                        });
                });
            });
        // ── Graph Panel ─────────────────────────────────────────────────────────
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Ru-Tu-Do");
            });
        });
    }
}

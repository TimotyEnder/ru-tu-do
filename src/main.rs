use crate::gui::RuToDoUI;

mod graph_elements;
mod gui;
mod turing_machine;
// or for no automatic layout:
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

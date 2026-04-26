use crate::turing_machine::TuringMachine;
use eframe::egui;
mod turing_machine;
struct App {
    counter: i32,
}
impl Default for App {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

fn main() {}

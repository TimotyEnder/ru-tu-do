use std::collections::{HashSet, VecDeque};
use std::error::Error;
static blank_cell_default_char: char = 'B';
pub struct TuringMachine {
    vertices: Vec<TuringVertex>,
}
impl TuringMachine {
    pub fn new(vertex_count: usize) -> Self {
        return TuringMachine {
            vertices: Vec::<TuringVertex>::with_capacity(vertex_count),
        };
    }
    pub fn add_transition(
        &mut self,
        from: usize,
        to: usize,
        to_write: &str,
        accepted_string: &str,
        move_direction: &str,
    ) -> Result<(), &'static str> {
        let move_dir_parsed: MovementDirection = match move_direction.to_lowercase().as_str() {
            "left" | "l" => MovementDirection::Left,
            "right" | "r" => MovementDirection::Right,
            _ => return Err("Invalid move direction, accepts only 'Left' or 'Right'"),
        };
        if !(0..self.vertices.len()).contains(&from) || !(0..self.vertices.len()).contains(&to) {
            return Err("Trying to add edges to vertices outside of the machine's scope.");
        } else if !self.vertices[from].add_transition(
            to,
            to_write,
            accepted_string,
            move_dir_parsed,
        ) {
            return Err(
                "Trying to add a transition with the same accepting input as another on this vertex. Turning machines are deterministic!",
            );
        }
        return Ok(());
    }
    pub fn process_string_input() {}
}
struct TuringVertex {
    pub transitions: Vec<TuringTransition>,
    pub set_of_accepted_strings_from_vertex: HashSet<String>,
}
impl<'a> TuringVertex {
    fn add_transition(
        &mut self,
        to: usize,
        to_write: &str,
        accepted_string: &str,
        move_direction: MovementDirection,
    ) -> bool {
        if self
            .set_of_accepted_strings_from_vertex
            .contains(accepted_string)
        {
            return false;
        } else {
            self.transitions.push({
                TuringTransition {
                    accepted_string: String::from(accepted_string),
                    to_write: String::from(to_write),
                    next_state_index: Some(to),
                    move_direction: move_direction,
                }
            });
            return true;
        }
    }
}

struct TuringTransition {
    accepted_string: String,
    to_write: String,
    next_state_index: Option<usize>,
    move_direction: MovementDirection,
}
enum MovementDirection {
    Left,
    Right,
}
struct TuringTape {
    tape: VecDeque<char>,
    reading_head_position: usize,
}
impl TuringTape {
    fn from_string_input(input: &str) -> Self {
        let mut tape: VecDeque<char> = input.chars().collect();
        for i in 0..3 {
            //artbitrary as the blank cells get added dynamically
            tape.push_front(blank_cell_default_char);
            tape.push_back(blank_cell_default_char);
        }
        return TuringTape {
            tape: tape,
            reading_head_position: 3,
        };
    }
    fn current_cell_input(&self) -> String {
        self.tape[self.reading_head_position].clone().to_string()
    }
}

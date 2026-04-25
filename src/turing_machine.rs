use std::collections::{HashSet, VecDeque};
use std::error::Error;
static BLANK_CELL_DEFAULT_CHAR: &str = "B";
pub struct TuringMachine {
    vertices: Vec<TuringVertex>,
}
impl TuringMachine {
    pub fn new(vertex_count: usize, list_of_accepting: &[usize]) -> Self {
        let mut to_ret: TuringMachine = TuringMachine {
            vertices: Vec::<TuringVertex>::new(),
        };
        for i in 0..vertex_count {
            to_ret.add_vertex(TuringVertex {
                transitions: Vec::<TuringTransition>::new(),
                set_of_accepted_strings_from_vertex: HashSet::<String>::new(),
                accepting: list_of_accepting.contains(&i),
            });
        }
        return to_ret;
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
    fn add_vertex(&mut self, vertex: TuringVertex) {
        self.vertices.push(vertex);
    }
    pub fn process_string_input() {}
}
struct TuringVertex {
    pub transitions: Vec<TuringTransition>,
    pub set_of_accepted_strings_from_vertex: HashSet<String>,
    pub accepting: bool,
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
    pub fn write_and_next_move(&self, tape: &mut TuringTape) -> Option<MovementDirection> {
        let tape_input = tape.current_cell_input();
        for transition in &self.transitions {
            if transition.accepted_string == tape_input {
                tape.write(&transition.to_write);
                return Some(transition.move_direction.clone());
            }
        }
        return None;
    }
}

struct TuringTransition {
    accepted_string: String,
    to_write: String,
    next_state_index: Option<usize>,
    move_direction: MovementDirection,
}
#[derive(PartialEq, Clone)]
enum MovementDirection {
    Left,
    Right,
}
struct TuringTape {
    tape: VecDeque<String>,
    reading_head_position: usize,
}
impl TuringTape {
    fn from_string_input(input: &str) -> Self {
        let mut tape: VecDeque<String> = input.chars().map(|c: char| c.to_string()).collect();
        for _ in 0..3 {
            //artbitrary as the blank cells get added dynamically
            tape.push_front(BLANK_CELL_DEFAULT_CHAR.to_string());
            tape.push_back(BLANK_CELL_DEFAULT_CHAR.to_string());
        }
        return TuringTape {
            tape: tape,
            reading_head_position: 3,
        };
    }
    fn current_cell_input(&self) -> String {
        self.tape[self.reading_head_position].clone().to_string()
    }
    fn write(&mut self, to_write: &str) {
        self.tape[self.reading_head_position] = to_write.to_string();
    }
    fn move_tape(&mut self, move_direction: MovementDirection) {
        if self.reading_head_position == 0 && move_direction == MovementDirection::Left {
            self.tape.push_front(BLANK_CELL_DEFAULT_CHAR.to_string());
        } else if self.reading_head_position + 1 >= self.tape.len()
            && move_direction == MovementDirection::Right
        {
            self.tape.push_back(BLANK_CELL_DEFAULT_CHAR.to_string());
            self.reading_head_position += 1;
        }
    }
}

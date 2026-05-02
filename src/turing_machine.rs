use std::{
    collections::{HashSet, VecDeque},
    usize::MAX,
};
static BLANK_CELL_DEFAULT_CHAR: &str = "#";
pub struct TuringMachine {
    pub vertices: Vec<TuringVertex>,
    start_state: usize,
    node_name_counter: usize,
}
impl TuringMachine {
    pub fn set_start_state(&mut self, state: usize) -> bool {
        if (0..=self.vertices.len()).contains(&state) {
            self.start_state = state;
            return true;
        }
        return false;
    }
    fn index_name_to_real_index(&self, index: &usize) -> usize {
        let mut real_state_index = MAX;
        let mut it = 0;
        for vertex in &self.vertices {
            if vertex.vertex_name == *index {
                real_state_index = it;
            }
            it += 1;
        }
        return real_state_index;
    }
    pub fn delete_state_with_index(&mut self, index: &usize) -> bool {
        //shift all vetices above the index down by one
        //delete all transition to that state on the way.
        let real_state_index = self.index_name_to_real_index(index);
        if !(0..=self.vertices.len() - 1).contains(&real_state_index) {
            return false;
        }
        let mut it: usize = 0;
        for vertex in &mut self.vertices {
            if it != real_state_index {
                for transition in &mut vertex.transitions {
                    if transition.next_state_index == Some(real_state_index) {
                        transition.next_state_index = None;
                    } else if let Some(value) = transition.next_state_index {
                        if value > real_state_index {
                            transition.next_state_index = Some(value - 1);
                        }
                    }
                }
            }
            it += 1;
        }
        self.vertices.remove(real_state_index);
        return true;
    }
    pub fn strip_state_with_index(&mut self, index: &usize) -> bool {
        let real_index = self.index_name_to_real_index(index);
        if !(0..=self.vertices.len() - 1).contains(&real_index) {
            return false;
        }
        self.vertices[real_index].transitions.clear();
        return true;
    }
    pub fn get_start_state(&self) -> usize {
        return self.start_state;
    }
    pub fn toggle_state_acception(&mut self, state_index: usize) -> Result<bool, &'static str> {
        if (0..=(self.vertices.len() - 1)).contains(&state_index) {
            if self.vertices[state_index].accepting {
                self.vertices[state_index].accepting = false;
                return Ok(false);
            } else {
                self.vertices[state_index].accepting = true;
                return Ok(true);
            }
        } else {
            return Err("State index out of bounds");
        }
    }
    pub fn new(vertex_count: usize, list_of_accepting: &[usize], start_state: &usize) -> Self {
        let mut to_ret: TuringMachine = TuringMachine {
            vertices: Vec::<TuringVertex>::new(),
            start_state: *start_state,
            node_name_counter: 0,
        };
        for i in 0..vertex_count {
            to_ret.add_vertex(TuringVertex {
                transitions: Vec::<TuringTransition>::new(),
                set_of_accepted_strings_from_vertex: HashSet::<String>::new(),
                accepting: list_of_accepting.contains(&i),
                vertex_name: to_ret.node_name_counter,
            });
            to_ret.node_name_counter += 1;
        }
        return to_ret;
    }
    pub fn new_default() -> Self {
        let mut to_ret = TuringMachine {
            vertices: Vec::<TuringVertex>::new(),
            start_state: 0,
            node_name_counter: 1,
        };
        to_ret.add_vertex(TuringVertex {
            transitions: Vec::<TuringTransition>::new(),
            set_of_accepted_strings_from_vertex: HashSet::<String>::new(),
            accepting: false,
            vertex_name: 0,
        });
        return to_ret;
    }
    pub fn add_transition(
        &mut self,
        from: &usize,
        to: &usize,
        to_write: &str,
        accepted_string: &str,
        move_direction: &str,
    ) -> Result<(), &'static str> {
        let move_dir_parsed: MovementDirection = match move_direction.to_lowercase().as_str() {
            "left" | "l" => MovementDirection::Left,
            "right" | "r" => MovementDirection::Right,
            _ => return Err("Invalid move direction, accepts only 'Left' or 'Right'"),
        };
        let from_real_index = self.index_name_to_real_index(from);
        let to_real_index = self.index_name_to_real_index(to);
        if !(0..self.vertices.len()).contains(&from_real_index)
            || !(0..self.vertices.len()).contains(&to_real_index)
        {
            return Err("Trying to add edges to vertices outside of the machine's scope.");
        } else if !self.vertices[from_real_index].add_transition(
            to_real_index,
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
    pub fn add_vertex_button_handler(&mut self) {
        let vertex = TuringVertex {
            transitions: Vec::<TuringTransition>::new(),
            set_of_accepted_strings_from_vertex: HashSet::<String>::new(),
            accepting: false,
            vertex_name: self.node_name_counter,
        };
        self.node_name_counter += 1;
        self.vertices.push(vertex);
    }
    pub fn process_string_input(&self, input: &str) -> (VecDeque<String>, bool, usize) {
        let mut tape = TuringTape::from_string_input(input);
        let mut current_state = self.start_state;
        print!("Initial state:\n");
        self.print_machine_state(&tape, &current_state);
        while !self.vertices[current_state].accepting
            && let Some(direction) =
                self.vertices[current_state].write_and_next_move(&mut tape, &mut current_state)
        {
            tape.move_tape(direction);
            self.print_machine_state(&tape, &current_state);
        }
        return (
            tape.tape,
            self.vertices[current_state].accepting,
            tape.reading_head_position,
        );
    }
    pub fn step(&self, tape: &mut TuringTape, current_state: &mut usize) -> bool {
        if let Some(direction) =
            self.vertices[*current_state].write_and_next_move(tape, current_state)
        {
            tape.move_tape(direction);
            return true;
        } else {
            return false;
        };
    }
    fn print_machine_state(&self, tape: &TuringTape, current_state: &usize) {
        print!("Current State ->[Q{}]\n", current_state);
        for tape_elem_index in 0..(tape.tape.len()) {
            if tape_elem_index == tape.reading_head_position {
                print!("[{}]", tape.tape[tape_elem_index]);
            } else {
                print!("{}", tape.tape[tape_elem_index]);
            }
        }
        print!("\n");
    }
}

pub struct TuringVertex {
    pub transitions: Vec<TuringTransition>,
    pub set_of_accepted_strings_from_vertex: HashSet<String>,
    pub accepting: bool,
    pub vertex_name: usize,
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
    pub fn write_and_next_move(
        &self,
        tape: &mut TuringTape,
        current_state: &mut usize,
    ) -> Option<MovementDirection> {
        let tape_input = tape.current_cell_input();
        for transition in &self.transitions {
            if transition.accepted_string == tape_input {
                tape.write(&transition.to_write);
                *current_state = match transition.next_state_index {
                    Some(index) => index,
                    None => *current_state,
                };
                self.print_transition_taken(transition);
                return Some(transition.move_direction.clone());
            }
        }
        return None;
    }
    fn print_transition_taken(&self, transition: &TuringTransition) {
        print!(
            "\n->Q{} write:{} move:{}\n\n",
            {
                match transition.next_state_index {
                    Some(index) => index.to_string(),
                    None => "No index".to_owned(),
                }
            },
            transition.to_write,
            {
                match transition.move_direction {
                    MovementDirection::Left => "Left",
                    MovementDirection::Right => "Right",
                }
            }
        )
    }
}

pub struct TuringTransition {
    pub accepted_string: String,
    pub to_write: String,
    pub next_state_index: Option<usize>,
    pub move_direction: MovementDirection,
}
impl TuringTransition {
    pub fn to_edge_label(&self) -> String {
        return format!(
            "{}/{}{}",
            self.accepted_string,
            self.to_write,
            match self.move_direction {
                MovementDirection::Left => "<-",
                MovementDirection::Right => "->",
            }
        );
    }
}
#[derive(PartialEq, Clone)]
pub enum MovementDirection {
    Left,
    Right,
}
pub struct TuringTape {
    pub tape: VecDeque<String>,
    reading_head_position: usize,
}
impl TuringTape {
    pub fn from_string_input(input: &str) -> Self {
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
    pub fn current_cell_index(&self) -> usize {
        return self.reading_head_position;
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
        } else {
            match move_direction {
                MovementDirection::Left => self.reading_head_position -= 1,
                MovementDirection::Right => self.reading_head_position += 1,
            }
        }
    }
}

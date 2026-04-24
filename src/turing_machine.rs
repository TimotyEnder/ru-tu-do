use std::collections::HashSet;
use std::error::Error;
pub struct TuringMachine {
    vertices: Vec<TuringVertex>,
}
impl TuringMachine {
    pub fn new(vertex_count: usize) -> Self {
        return TuringMachine {
            vertices: Vec::<TuringVertex>::with_capacity(vertex_count),
        };
    }
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        if !(0..self.vertices.len()).contains(&from) || !(0..self.vertices.len()).contains(&to) {
            return Err("Trying to add edges to vertices outside of the machine's scope.");
        } else {
        }
        return Ok(());
    }
}
struct TuringVertex {
    pub transition_edges: Vec<TuringEdge>,
    pub set_of_accepted_strings_from_vertex: HashSet<TuringEdge>,
}
impl TuringVertex {}
struct TuringEdge {
    accepted_string: String,
    to_write: String,
    destination: MovementDirection,
}
enum MovementDirection {
    Left,
    Right,
}

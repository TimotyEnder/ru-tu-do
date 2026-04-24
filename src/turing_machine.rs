pub struct TuringMachine {
    vertices: Vec<TuringVertex>,
}
impl TuringMachine {
    pub fn new(vertex_count: usize) -> Self {
        return TuringMachine {
            vertices: Vec::<TuringVertex>::new(vertex_count),
        };
    }
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<Self, Error> {
        if !(0..self.vertices.len()).contains(&from) || !(0..self.vertices.len()).contains(&to) {
            return Err("Trying to add edges to vertices outside of the machine's scope.");
        }
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

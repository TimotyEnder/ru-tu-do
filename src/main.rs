use crate::turing_machine::TuringMachine;

mod turing_machine;

fn main() {
    let mut turing_mach = TuringMachine::new(3, &[2], &(0));
    turing_mach.add_transition(0, 0, "C", "A", "r").unwrap();
    turing_mach.add_transition(0, 0, "A", "C", "r").unwrap();
    turing_mach.add_transition(0, 1, "B", "B", "l").unwrap();
    turing_mach.add_transition(1, 1, "A", "A", "l").unwrap();
    turing_mach.add_transition(1, 1, "C", "C", "l").unwrap();
    turing_mach.add_transition(1, 1, "B", "B", "r").unwrap();
    let (tape, accepted, final_head_position) = turing_mach.process_string_input("AACCCAA");
    println!("{:?}", tape);
    print!(
        "final head position is on the {}th element",
        final_head_position
    );
    print!("accepted? {}", accepted);
}

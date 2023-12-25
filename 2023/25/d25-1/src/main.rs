fn main() {
    let input = include_str!("../../example.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    todo!()
}

struct Node(String);

struct Wire {
    node_a: Node,
    node_b: Node,
}

struct Network(Vec<Wire>);

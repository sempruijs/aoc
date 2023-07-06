use crate::intcode::*;

mod intcode;

fn main() {
    let puzzle_input = include_str!("../../input.txt");
    let numbers: Vec<usize> = puzzle_input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut code = numbers_to_code(numbers);

    for i in 0..code.amount_of_instructions() {
        let instruction = list_to_instruction(code.get_chunk(i));
        code.apply_instruction(&instruction);
    }
    // dbg!(code.0);
    // code.run_instructions(instructions);
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let lists = puzzle_input_lists(s);
    let answer = lists_to_answer(&lists.0, &lists.1);
    answer
}

fn puzzle_input_lists(s: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers: Vec<u32> = s
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let v1 = numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| *x)
        .collect::<Vec<u32>>();

    let v2 = numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, x)| *x)
        .collect::<Vec<u32>>();

    (v1, v2)
}

fn amount_in_vector(v: &Vec<u32>, x: &u32) -> usize {
    v.iter().filter(|a| a == &x).count()
}

fn lists_to_answer(v1: &Vec<u32>, v2: &Vec<u32>) -> usize {
    v1.iter()
        .map(|x| amount_in_vector(v2, x) * (*x as usize))
        .sum()
}

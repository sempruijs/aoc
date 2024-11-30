fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> usize {
    s.len()
}

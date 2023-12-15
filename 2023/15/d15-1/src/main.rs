fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_result(input);
    println!("{}", answer);
}

fn puzzle_input_to_result(s: &str) -> u32 {
    s.split_once("\n").unwrap().0.split(",").map(hash).sum()
}

fn hash(s: &str) -> u32 {
    let ascii_values = s.as_bytes();
    let mut current: u32 = 0;
    for n in ascii_values {
        current += *n as u32;
        current = current * 17;
        current = current % 256;
    }
    current
}

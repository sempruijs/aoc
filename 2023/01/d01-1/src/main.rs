fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> u32 {
    s.lines().map(line_to_calibration_value).sum()
}

fn line_to_calibration_value(s: &str) -> u32 {
    let numbers = line_to_numbers(s);
    numbers_to_calibration_value(numbers)
}

fn line_to_numbers(s: &str) -> Vec<u32> {
    s.chars()
        .into_iter()
        .filter(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn numbers_to_calibration_value(xs: Vec<u32>) -> u32 {
    let first = xs.first().unwrap();
    let last = xs.last().unwrap();
    (first * 10) + last
}

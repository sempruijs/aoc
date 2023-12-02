const WORD_NUMBERS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> u32 {
    s.lines().map(line_to_calibration_value).sum()
}

fn reverse_str(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn line_to_calibration_value(s: &str) -> u32 {
    let first = first_digit(s, false);

    // reverse is on so it will search for the last word
    let last = first_digit(s, true);

    (first * 10) + last
}

fn first_digit(s: &str, reverse: bool) -> u32 {
    let numbers = match reverse {
        false => vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0",
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ],
        true => vec![
            "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin", "0",
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ],
    };
    let s: String = match reverse {
        true => reverse_str(s),
        false => String::from(s),
    };

    let mut result_index = s.len();
    let mut result: u32 = 0;

    for (index, n) in numbers.iter().enumerate() {
        if let Some(i) = s.find(n) {
            if i < result_index {
                result_index = i;
                result = (index % 10) as u32;
            }
        }
    }

    result
}

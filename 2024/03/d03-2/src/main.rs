fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("{}", answer);
}

#[derive(Clone, Debug)]
enum Instruction {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn input_to_answer(s: &str) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, _)| try_parse_instruction(s, i))
        .fold((true, 0), |(active, sum), current| match current {
            Some(instruction) => match instruction {
                Instruction::Multiply(x, y) => match active {
                    true => (active, (sum + (x * y))),
                    false => (active, sum),
                },
                Instruction::Do => (true, sum),
                Instruction::Dont => (false, sum),
            },
            None => (active, sum),
        })
        .1
}

fn try_parse_instruction(s: &str, i: usize) -> Option<Instruction> {
    if s.len() - i > 8 {
        if &s[i..(i + 7)] == "don't()" {
            return Some(Instruction::Dont);
        }
        if &s[i..(i + 4)] == "do()" {
            return Some(Instruction::Do);
        }
        let mul = &s[i..(i + 4)];
        if mul == "mul(" {
            let between_brackets = match &s[(i + 4)..].split_once(")") {
                Some(p) => p.0,
                None => {
                    return None;
                }
            };
            let (left, right) = match between_brackets.split_once(",") {
                Some(p) => (p.0, p.1),
                None => {
                    return None;
                }
            };
            let left_n = match parse_number(left) {
                Some(n) => n,
                None => return None,
            };
            let right_n = match parse_number(right) {
                Some(n) => n,
                None => return None,
            };
            return Some(Instruction::Multiply(left_n, right_n));
        }
    }
    None
}

fn parse_number(s: &str) -> Option<i32> {
    match s.chars().filter(|c| c.is_whitespace()).count() > 0 {
        true => None,
        false => match s.parse::<i32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        },
    }
}

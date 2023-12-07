use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_result(input);
    println!("{}", answer);
}

fn puzzle_input_to_result(s: &str) -> u32 {
    s.lines().map(|s| Card::from_line(s).points()).sum()
}

struct Card {
    win_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

impl Card {
    fn from_line(s: &str) -> Self {
        let (win_str, your_str) = s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let win_numbers: HashSet<u32> = HashSet::from_iter(
            win_str
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap()),
        );

        let your_numbers: HashSet<u32> = HashSet::from_iter(
            your_str
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap()),
        );

        Card {
            win_numbers,
            your_numbers,
        }
    }

    fn points(&self) -> u32 {
        let amount_of_winning_numbers: u32 = self
            .win_numbers
            .intersection(&self.your_numbers)
            .collect::<Vec<&u32>>()
            .len() as u32;

        return match amount_of_winning_numbers == 0 {
            true => 0,
            false => (2 as u32).pow(amount_of_winning_numbers - 1),
        };
    }
}

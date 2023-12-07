use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_result(input);
    println!("{}", answer);
}

fn puzzle_input_to_result(s: &str) -> u32 {
    // s.lines().map(|s| Card::from_line(s).points()).sum()
    let cards: Vec<Card> = s.lines().map(|s| Card::from_line(s)).collect();
    cards_to_copy_amount(cards.as_slice(), Vec::new(), 1)
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

        amount_of_winning_numbers
    }
}

fn distribute_over(x: u32, copies: Vec<u32>) -> Vec<u32> {
    let mut result = copies;

    for i in 0..x {
        let i = i as usize;
        if let _ = result[i] {
            result[i] += 1;
        } else {
            result.push(2);
        }
    }
    result
}

// [Cards] -> [u32] -> u32 -> u32
fn cards_to_copy_amount(cards: &[Card], copies: Vec<u32>, n: u32) -> u32 {
    // are cards
    let (c, cs) = (&cards[0], &cards[1..]);

    // are copies
    let (x, xs) = (&copies[0], &copies[1..]);

    let points = c.points();
    let copies = distribute_over(points, xs.to_vec());

    if copies.iter().sum::<u32>() == 0 {
        // base case
        return n;
    } else {
        cards_to_copy_amount(cs, copies, n + x)
    }
}

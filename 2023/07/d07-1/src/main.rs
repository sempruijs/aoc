use std::collections::HashMap;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = puzzle_input_to_result(input);
    println!("{answer}");
}

fn puzzle_input_to_result(s: &str) -> u32 {
    let mut hands = Hands::from_str(s).0;
    hands.sort_by_key(|h| h.value);
    hands.reverse();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum()
}

#[derive(Debug)]
struct Hand {
    value: u32,
    bid: u32,
}

struct Hands(Vec<Hand>);

// this function will
fn cards_to_value(s: &str) -> u32 {
    let type_value = cards_to_type(s);
    let mut hex = String::new();
    hex.push(type_value);

    for c in s.chars() {
        let c = char_to_hexadecimal(&c);
        hex.push(c);
    }

    u32::from_str_radix(&hex, 16).unwrap()
}

impl Hand {
    fn from_line(s: &str) -> Self {
        let (cards, bid) = s.split_once(" ").unwrap();
        let value = cards_to_value(cards);
        let bid: u32 = bid.parse().unwrap();
        Self { value, bid }
    }
}

impl Hands {
    fn from_str(s: &str) -> Self {
        Hands(s.lines().map(|s| Hand::from_line(s)).collect::<Vec<Hand>>())
    }
}

fn char_to_hexadecimal(c: &char) -> char {
    match c {
        &'A' => 'e',
        &'K' => 'd',
        &'Q' => 'c',
        &'J' => 'b',
        &'T' => 'a',
        &'9' => '9',
        &'8' => '8',
        &'7' => '7',
        &'6' => '6',
        &'5' => '5',
        &'4' => '4',
        &'3' => '3',
        &'2' => '2',
        _ => panic!("could not find convert"),
    }
}

fn cards_to_type(s: &str) -> char {
    let mut m: HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        *m.entry(c).or_default() += 1;
    }

    let mut frequencies: Vec<&u32> = m.values().collect();
    frequencies.sort();

    let first = frequencies[0];
    let second = frequencies[1];

    match first {
        &5 => 'f',
        &4 => 'e',
        &3 => {
            if second == &2 {
                'd'
            } else {
                'c'
            }
        }
        &2 => {
            if second == &2 {
                'b'
            } else {
                'a'
            }
        }
        _ => '9',
    }
}

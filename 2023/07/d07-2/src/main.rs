use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_result(input);
    println!("{answer}");
}

fn puzzle_input_to_result(s: &str) -> u32 {
    let mut hands = Hands::from_str(s).0;
    hands.sort_by_key(|h| h.value);
    dbg!(&hands);
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
    let hex = cards_to_hex_value(s);
    hex_to_value(&hex)
}

fn hex_to_value(s: &str) -> u32 {
    u32::from_str_radix(&s, 16).unwrap()
}

fn cards_to_hex_value(s: &str) -> String {
    let type_value = cards_to_type(s);
    let mut hex = String::new();
    hex.push(type_value);

    for c in s.chars() {
        let c = char_to_hexadecimal(&c);
        hex.push(c);
    }

    hex
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
        &'J' => '0',
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

    let j_frequencie = *m.get(&'J').unwrap_or(&0);
    m.remove_entry(&'J');

    let mut frequencies: Vec<&u32> = m.values().collect();
    frequencies.sort();
    frequencies.reverse();

    let first = match j_frequencie == 5 {
        true => 5,
        false => frequencies[0] + j_frequencie,
    };
    if first == 5 {
        return 'f';
    }
    let second = frequencies[1];

    match first {
        4 => 'e',
        3 => {
            if second == &2 {
                'd'
            } else {
                'c'
            }
        }
        2 => {
            if second == &2 {
                'b'
            } else {
                'a'
            }
        }
        _ => '9',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards_to_hex_value() {
        let cards_1 = "32T3K";
        let expected_1 = "a32a3d";
        let result_1 = cards_to_hex_value(cards_1);

        let cards_2 = "T55J5";
        let expected_2 = "ea5505";
        let result_2 = cards_to_hex_value(cards_2);

        let cards_3 = "KK677";
        let expected_3 = "bdd677";
        let result_3 = cards_to_hex_value(cards_3);

        let cards_4 = "KTJJT";
        let expected_4 = "eda00a";
        let result_4 = cards_to_hex_value(cards_4);

        let cards_5 = "QQQJA";
        let expected_5 = "eccc0e";
        let result_5 = cards_to_hex_value(cards_5);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
    }

    #[test]
    fn test_hex_to_value() {
        let hex_1 = "ccccbe";
        let result_1 = hex_to_value(&hex_1);
        let expected_1 = 13421758;

        assert_eq!(result_1, expected_1);
    }
}

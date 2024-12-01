fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

struct Pair {
    x: u32,
    y: u32,
}

impl Pair {
    fn difference(&self) -> u32 {
        let (small, big) = match &self.x > &self.y {
            true => (&self.y, &self.x),
            false => (&self.x, &self.y),
        };
        (big - small).clone()
    }
}

fn input_to_answer(s: &str) -> u32 {
    let lists = puzzle_input_lists(s);
    let v1 = order_list(lists.0);
    let v2 = order_list(lists.1);
    let pairs = lists_to_pairs(v1, v2);
    let answer = pairs.iter().map(|p| p.difference()).sum::<u32>();
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

fn order_list(v: Vec<u32>) -> Vec<u32> {
    let mut list = v;
    list.sort();
    list
}

fn lists_to_pairs(v1: Vec<u32>, v2: Vec<u32>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..(v1.len()) {
        let x = v1[i];
        let y = v2[i];
        let p = Pair { x, y };
        pairs.push(p);
    }
    pairs
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);

    println!("{}", answer);
}

#[derive(PartialEq, PartialOrd)]
struct BagSet {
    r: u32,
    g: u32,
    b: u32,
}

struct Game {
    id: u32,
    bag_sets: Vec<BagSet>,
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    s.lines()
        .map(|s| Game::from_line(s))
        .filter(|game| game.max_criteria(12, 13, 14))
        .map(|game| game.id)
        .sum::<u32>()
}

impl Game {
    pub fn from_line(s: &str) -> Self {
        let id = line_to_game_id(s);

        let bag_sets_strings: Vec<Vec<&str>> = s
            .split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .map(|s| s.split(", ").collect::<Vec<&str>>())
            .collect();

        let bag_sets: Vec<BagSet> = bag_sets_strings
            .into_iter()
            .map(|bag_set_string| -> BagSet {
                bag_set_string
                    .iter()
                    .fold(BagSet { r: 0, b: 0, g: 0 }, |mut acc, s| {
                        let (n, color) = s.split_once(" ").unwrap();
                        let n: u32 = n.parse().unwrap();
                        match color {
                            "red" => acc.r += n,
                            "green" => acc.g += n,
                            "blue" => acc.b += n,
                            _ => panic!("cannot find color: {}", color),
                        };
                        acc
                    })
            })
            .collect::<Vec<BagSet>>();

        Game { id, bag_sets }
    }

    pub fn max_criteria(&self, r: u32, g: u32, b: u32) -> bool {
        let max_bag: BagSet =
            self.bag_sets
                .iter()
                .fold(BagSet { r: 0, g: 0, b: 0 }, |mut acc, x| {
                    if x.r > acc.r {
                        acc.r = x.r;
                    }
                    if x.g > acc.g {
                        acc.g = x.g;
                    }
                    if x.b > acc.b {
                        acc.b = x.b;
                    }
                    acc
                });

        max_bag.r <= r && max_bag.g <= g && max_bag.b <= b
    }
}

fn line_to_game_id(s: &str) -> u32 {
    s.split_once(":")
        .unwrap()
        .0
        .split_once(" ")
        .unwrap()
        .1
        .parse::<u32>()
        .unwrap()
}

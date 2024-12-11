use std::num::ParseIntError;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

#[derive(Debug)]
struct World(Vec<u128>);

fn input_to_answer(s: &str) -> usize {
    let mut w = World::try_from(s).unwrap();
    (0..25).for_each(|_| w.blink());
    w.0.len()
}

impl TryFrom<&str> for World {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines()
                .next()
                .unwrap()
                .split_whitespace()
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect(),
        ))
    }
}

impl World {
    fn blink(&mut self) {
        let mut shifted = 0;
        for (i, n) in self.0.clone().iter().enumerate() {
            if n == &0 {
                self.0[i + shifted] = 1;
            } else {
                if let Some((x, y)) = split(n) {
                    self.0[i + shifted] = x;
                    shifted += 1;
                    self.0.insert(i + shifted, y);
                } else {
                    self.0[i + shifted] = n * 2024;
                }
            }
        }
    }
}

fn split(n: &u128) -> Option<(u128, u128)> {
    let s = n.to_string();
    match s.len() % 2 == 0 && s.len() >= 2 {
        true => {
            let left: &u128 = &s[0..(s.len() / 2)].parse().unwrap();
            let right: &u128 = &s[(s.len() / 2)..].parse().unwrap();
            Some((*left, *right))
        }
        false => None,
    }
}

use std::{fmt::Display, iter::zip};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let worlds = Worlds::from_str(s);
    worlds.sum_values()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Ash,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Rock => '#',
            Cell::Ash => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct World(Vec<Vec<Cell>>);

#[derive(Debug)]
struct Worlds(Vec<World>);

impl Worlds {
    fn from_str(s: &str) -> Self {
        Self(
            s.split("\n\n")
                .map(|s| World::from_str(s))
                .collect::<Vec<World>>(),
        )
    }

    fn sum_values(&self) -> u32 {
        self.0.iter().map(|w| w.value()).sum()
    }
}

impl World {
    fn from_str(s: &str) -> Self {
        Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Cell::Ash,
                            '#' => Cell::Rock,
                            _ => panic!("could not "),
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>(),
        )
    }

    fn horizontal_mirror(&self) -> Option<u32> {
        for line_index in 1..self.0.len() {
            let mirror_pairs: Vec<(usize, usize)> =
                zip((0..line_index).rev(), line_index..self.0.len()).collect();
            let mut use_mistake = false;
            if mirror_pairs
                .clone()
                .into_iter()
                .filter(|(p_i, n_i)| {
                    match_mirror_criteria(&self.0[*p_i], &self.0[*n_i], &mut use_mistake)
                })
                .count()
                == mirror_pairs.len()
                && use_mistake
            {
                return Some((line_index) as u32);
            }
        }
        None
    }

    fn vertical_mirror(&self) -> Option<u32> {
        self.transpose().horizontal_mirror()
    }

    fn value(&self) -> u32 {
        let horizontal = self.horizontal_mirror().unwrap_or(0);
        let vertical = self.vertical_mirror().unwrap_or(0);
        horizontal * 100 + vertical
    }

    fn transpose(&self) -> Self {
        let transposed = transpose(self.0.clone());
        Self(transposed)
    }
}

fn match_mirror_criteria(v1: &Vec<Cell>, v2: &Vec<Cell>, use_mistake: &mut bool) -> bool {
    // self.0[*p_i] == self.0[*n_i]
    match use_mistake {
        true => v1 == v2,
        false => match v1 == v2 {
            true => true,
            false => {
                let matching = zip(v1, v2).into_iter().filter(|&(a, b)| a == b).count();
                if matching + 1 == v1.len() {
                    *use_mistake = true;
                    return true;
                }
                false
            }
        },
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

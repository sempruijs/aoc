use std::mem;

use memoize::memoize;
use rayon::prelude::*;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let w = World::try_from(s).unwrap();
    w.answer()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Towel(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Towels(Vec<Towel>);

impl From<&str> for Towels {
    fn from(s: &str) -> Self {
        Self(
            s.split(", ")
                .map(|s| Towel(s.to_string()))
                .collect::<Vec<Towel>>(),
        )
    }
}

struct World {
    towels: Towels,
    combinations: Combinations,
}

impl World {
    fn answer(self) -> usize {
        self.combinations
            .0
            .into_iter()
            .enumerate()
            .inspect(|(i, c)| println!("{i}, {}", c.0))
            .map(|(_, c)| {
                let towels = self.towels.clone().get_useful(&c);
                println!("{}", c.0);
                valid_amount(c, towels)
            })
            .sum()
    }
}

impl Towels {
    fn get_useful(self, combination: &Combination) -> Self {
        Self(
            self.0
                .into_iter()
                .filter(|t| combination.0.contains(&t.0))
                .collect(),
        )
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (left, right) = s.split_once("\n\n").unwrap();
        let towels = Towels::from(left);
        let combinations = Combinations::from(right);
        Ok(Self {
            towels,
            combinations,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Combination(String);

struct Combinations(Vec<Combination>);

impl From<&str> for Combinations {
    fn from(s: &str) -> Self {
        Self(s.lines().map(|s| Combination(s.to_string())).collect())
    }
}

#[memoize]
fn valid_amount(combination: Combination, towels: Towels) -> usize {
    // base case
    if combination.0.is_empty() {
        return 1;
    }
    if towels.0.iter().all(|t| !combination.0.ends_with(&t.0))
        || towels.0.iter().all(|t| !combination.0.starts_with(&t.0))
    {
        return 0;
    }

    towels
        .0
        .iter()
        .map(|towel| {
            if combination.0.starts_with(&towel.0) {
                let combination = Combination(combination.0[towel.0.len()..].to_string());
                let towels = &towels.clone().get_useful(&combination);
                return valid_amount(combination, towels.clone());
            }
            return 0;
        })
        .sum()
}

impl Combination {}

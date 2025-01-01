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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Towel(String);

#[derive(Debug, Clone, PartialEq, Eq)]
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
            .filter(|(_, c)| {
                let towels = self.towels.clone().get_useful(c);
                println!("{}", c.0);
                c.valid(&towels)
            })
            .count()
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

struct Combination(String);

struct Combinations(Vec<Combination>);

impl From<&str> for Combinations {
    fn from(s: &str) -> Self {
        Self(s.lines().map(|s| Combination(s.to_string())).collect())
    }
}

impl Combination {
    fn valid(&self, towels: &Towels) -> bool {
        // base case
        if self.0.is_empty() {
            return true;
        }
        if towels.0.iter().all(|t| !self.0.ends_with(&t.0))
            || towels.0.iter().all(|t| !self.0.starts_with(&t.0))
        {
            return false;
        }

        towels.0.iter().any(|towel| {
            if self.0.starts_with(&towel.0) {
                let combination = Combination(self.0[towel.0.len()..].to_string());
                let towels = &towels.clone().get_useful(&combination);
                return combination.valid(towels);
            }
            return false;
        })
    }
}

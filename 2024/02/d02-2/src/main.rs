use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let reports = input_to_reports(&s);
    reports
        .iter()
        .filter(|r| r.valid() || r.valid_with_one_mistake())
        .count()
}

#[derive(Debug, Eq, PartialEq)]
struct Report(Vec<i32>);

fn line_to_report(s: &str) -> Report {
    let v: Vec<i32> = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    Report(v)
}

fn input_to_reports(s: &str) -> Vec<Report> {
    s.lines().map(line_to_report).collect()
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Decreasing,
    Increasing,
}

struct Pair(i32, i32);

impl Pair {
    fn dir(&self) -> Direction {
        match self.0 > self.1 {
            true => Direction::Decreasing,
            false => Direction::Increasing,
        }
    }

    fn valid_difference(&self) -> bool {
        let difference = match self.0 > self.1 {
            true => self.0 - self.1,
            false => self.1 - self.0,
        };
        difference > 0 && difference < 4
    }
}

impl Report {
    fn to_pairs(&self) -> Vec<Pair> {
        self.0
            .clone()
            .into_iter()
            .tuple_windows()
            .map(|(p, c)| Pair(p, c))
            .collect()
    }

    fn valid(&self) -> bool {
        let pairs = &self.to_pairs();
        let correct_difference = pairs.iter().fold(true, |result, p| match result {
            true => p.valid_difference(),
            false => false,
        });
        let correct_direction = pairs.iter().all(|p| p.dir() == Direction::Decreasing)
            || pairs.iter().all(|p| p.dir() == Direction::Increasing);
        correct_difference && correct_direction
    }

    fn valid_with_one_mistake(&self) -> bool {
        let r = &self.clone();
        self.0
            .clone()
            .iter()
            .enumerate()
            .fold(false, |result, (i, _)| {
                let new_r = r.remove_at_index(i);
                match result {
                    true => true,
                    false => new_r.valid(),
                }
            })
    }

    fn remove_at_index(&self, i: usize) -> Self {
        let mut v = self.0.clone();
        v.remove(i);
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid() {
        let input = Report(vec![7, 6, 4, 2, 1]);
        assert!(input.valid());

        let input = Report(vec![1, 2, 7, 8, 9]);
        assert!(!input.valid());
    }
}

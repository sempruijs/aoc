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

impl Report {
    fn valid(&self) -> bool {
        let decrease = self.0.iter().is_sorted_by(|x, y| {
            let diff = **x - **y;
            diff < 4 && diff > 0
        });
        let increase = self.0.iter().is_sorted_by(|x, y| {
            let diff = **y - **x;
            diff < 4 && diff > 0
        });
        increase || decrease
    }

    fn valid_with_one_mistake(&self) -> bool {
        let r = self;
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

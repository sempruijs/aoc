use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> i32 {
    let (rules, updates) = s.split_once("\n\n").unwrap();
    let rules = Rules::from(rules);
    updates
        .lines()
        .map(Update::from)
        .filter(|u| !u.valid_with_rules(&rules))
        .map(|u| u.order(&rules).middle())
        .sum()
}

struct Rule {
    left: i32,
    right: i32,
}
struct Rules(Vec<Rule>);

#[derive(Debug, PartialEq, Eq)]
struct Update(Vec<i32>);

impl From<&str> for Update {
    fn from(s: &str) -> Self {
        Self(
            s.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        Self {
            left: s.split_once("|").unwrap().0.parse::<i32>().unwrap(),
            right: s.split_once("|").unwrap().1.parse::<i32>().unwrap(),
        }
    }
}

impl From<&str> for Rules {
    fn from(s: &str) -> Self {
        Self(s.lines().map(Rule::from).collect())
    }
}

impl Update {
    fn valid_with_rules(&self, rs: &Rules) -> bool {
        rs.0.iter()
            .map(|r| self.valid_with_rule(r))
            .filter(|b| !b)
            .count()
            == 0
    }

    fn valid_with_rule(&self, r: &Rule) -> bool {
        self.0
            .iter()
            .filter(|x| **x == r.left || **x == r.right)
            .is_sorted_by(|a, b| (a == &&r.left && b == &&r.right) || a == b)
    }

    fn middle(&self) -> i32 {
        let mid = &self.0.len() / 2;
        self.0[mid]
    }

    fn order(&self, rs: &Rules) -> Self {
        let mut v = self.0.clone();
        let mut allow = true;
        self.0
            .iter()
            .enumerate()
            .tuple_windows()
            .for_each(|((i, a), (j, b))| {
                if allow {
                    if let Some((a, b)) = rs.switch_if_incorrect((*a, *b)) {
                        v[i] = a;
                        v[j] = b;
                        allow = false;
                    }
                }
            });
        match Self(v.clone()).valid_with_rules(rs) {
            true => Self(v),
            false => Self(v).order(rs),
        }
    }
}

impl Rules {
    fn switch_if_incorrect(&self, p: (i32, i32)) -> Option<(i32, i32)> {
        let update = Update(vec![p.0, p.1]);
        match update.valid_with_rules(self) {
            true => None,
            false => Some((p.1, p.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_answer() {
        let input = include_str!("../../example.txt");
        let result = input_to_answer(input);
        let expected = 123;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_order() {
        let puzzle_input = include_str!("../../example.txt");
        let rules = Rules::from(puzzle_input.split_once("\n\n").unwrap().0);

        let input = Update(vec![75, 97, 47, 61, 53]);
        let expected = Update(vec![97, 75, 47, 61, 53]);
        let result = input.order(&rules);
        let middel = result.middle();
        assert_eq!(middel, 47);
        assert_eq!(expected, result);

        let input = Update(vec![61, 13, 29]);
        let expected = Update(vec![61, 29, 13]);
        let result = input.order(&rules);
        let middel = result.middle();
        assert_eq!(middel, 29);
        assert_eq!(expected, result);

        let input = Update(vec![97, 13, 75, 29, 47]);
        let expected = Update(vec![97, 75, 47, 29, 13]);
        let result = input.order(&rules);
        let middel = result.middle();
        assert_eq!(middel, 47);
        assert_eq!(expected, result);

        let input = Update(vec![1, 2, 3, 4, 5]);
        let expected = Update(vec![1, 2, 3, 4, 5]);
        let result = input.order(&rules);
        let middel = result.middle();
        assert_eq!(middel, 3);
        assert_eq!(expected, result);
    }
}

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
        .filter(|u| u.valid_with_rules(&rules))
        .map(|u| u.middle())
        .sum()
}

struct Rule {
    left: i32,
    right: i32,
}
struct Rules(Vec<Rule>);

struct Update(Vec<i32>);

impl From<&str> for Update {
    fn from(s: &str) -> Self {
        Self(s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once("|").unwrap();
        Self {
            left: left.parse::<i32>().unwrap(),
            right: right.parse::<i32>().unwrap(),
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
}

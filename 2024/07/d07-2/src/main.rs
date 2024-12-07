use radix_fmt::*;
use rayon::prelude::*;
use std::num::ParseIntError;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> i64 {
    let lines: Vec<&str> = s.lines().collect();
    let operations: Vec<Equation> = lines
        .into_par_iter()
        .map(|l| Equation::try_from(l).unwrap())
        .filter(|e| e.valid())
        .collect();
    operations
        .iter()
        .fold(0, |sum, current| sum + current.expected)
}

struct Equation {
    expected: i64,
    numbers: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Plus(i64),
    Multiply(i64),
    Concat(i64),
}

#[derive(Debug, PartialEq, Eq)]
struct Operations(Vec<Operation>);

impl TryFrom<&str> for Equation {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (expected, ns) = s.split_once(":").unwrap();
        let expected = expected.parse::<i64>()?;
        let numbers: Result<Vec<i64>, _> =
            ns.split_whitespace().map(|n| n.parse::<i64>()).collect();
        Ok(Self {
            expected,
            numbers: numbers?,
        })
    }
}

impl Operation {
    fn merge_with(&self, o: &Operation) -> Self {
        match self {
            Operation::Plus(a) => match o {
                Operation::Plus(b) => Self::Plus(a + b),
                Operation::Multiply(b) => Self::Multiply(a + b),
                Operation::Concat(b) => Self::Concat(a + b),
            },
            Operation::Multiply(a) => match o {
                Operation::Plus(b) => Self::Plus(a * b),
                Operation::Multiply(b) => Self::Multiply(a * b),
                Operation::Concat(b) => Self::Concat(a * b),
            },
            Self::Concat(a) => match o {
                Self::Plus(b) => {
                    let n = a.to_string() + &b.to_string();
                    Self::Plus(n.parse().unwrap())
                }
                Self::Multiply(b) => {
                    let n = a.to_string() + &b.to_string();
                    Self::Multiply(n.parse().unwrap())
                }
                Self::Concat(b) => {
                    let n = a.to_string() + &b.to_string();
                    Self::Concat(n.parse().unwrap())
                }
            },
        }
    }

    fn to_number(&self) -> i64 {
        match self {
            Self::Plus(x) => *x,
            Self::Multiply(x) => *x,
            Self::Concat(x) => *x,
        }
    }
}

impl Operations {
    fn calculate(&self) -> i64 {
        self.0
            .iter()
            .fold(Operation::Multiply(1), |sum, current| {
                sum.merge_with(current)
            })
            .to_number()
    }
}

impl Equation {
    fn all_operations(&self) -> Vec<Operations> {
        let len = self.numbers.len();
        let result_len = 3_i64.pow(len.try_into().unwrap());
        (0..result_len)
            .map(|n| {
                let bytes = format!("{}", radix_3(n));
                let zeros = String::from("0")
                    .chars()
                    .cycle()
                    .take(len - bytes.len())
                    .collect::<String>();
                let bytes = zeros + &bytes;
                Operations(
                    bytes
                        .chars()
                        .enumerate()
                        .map(|(i, c)| match c {
                            '0' => Operation::Plus(self.numbers[i]),
                            '1' => Operation::Multiply(self.numbers[i]),
                            '2' => Operation::Concat(self.numbers[i]),
                            _ => panic!("bla "),
                        })
                        .collect::<Vec<Operation>>(),
                )
            })
            .filter(|operations| match operations.0.last().unwrap() {
                Operation::Plus(_) => true,
                Operation::Multiply(_) => false,
                Operation::Concat(_) => false,
            })
            .collect()
    }

    fn valid(&self) -> bool {
        self.all_operations()
            .into_iter()
            .filter(|o| o.calculate() == self.expected)
            .count()
            > 0
    }
}

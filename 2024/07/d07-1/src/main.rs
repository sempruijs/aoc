use std::num::ParseIntError;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> i64 {
    s.lines()
        .map(|l| Equation::try_from(l).unwrap())
        .filter(|e| e.valid())
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
    // fn merge_with(&self, o: &Operation) -> Self {
    //     match self {
    //         Operation::Plus(a) => match o {
    //             Operation::Plus(b) => Self::Plus(a + b),
    //             Operation::Multiply(_) => panic!("Error: tried to merge Plus with Multiply which is not allowed. Only Multiply and Plus i s allowed"),
    //         },
    //         Operation::Multiply(a) => match o {
    //             Operation::Plus(b) => Self::Plus(a * b),
    //             Operation::Multiply(b) => Self::Multiply(a * b),
    //         },
    //     }
    // }

     fn merge_with(&self, o: &Operation) -> Self {
        match self {
            Operation::Plus(a) => match o {
                Operation::Plus(b) => Self::Plus(a + b),
                Operation::Multiply(b) => Self::Multiply(a + b),
            },
            Operation::Multiply(a) => match o {
                Operation::Plus(b) => Self::Plus(a * b),
                Operation::Multiply(b) => Self::Multiply(a * b),
            },
        }
    }


    fn to_number(&self) -> i64 {
        match self {
            Operation::Plus(x) => *x,
            Operation::Multiply(x) => *x,
        }
    }
}

impl Operations {
    // fn calculate_mul(&self) -> Self {
    //     let mut v = self.0.clone();
    //     if v.len() == 1 {
    //         return Self(match v.iter().next().unwrap() {
    //             Operation::Plus(x) => vec![Operation::Plus(*x)],
    //             Operation::Multiply(x) => vec![Operation::Plus(*x)],
    //         });
    //     }
    //     for (i, o) in self.0.iter().enumerate() {
    //         if let Operation::Multiply(_) = o {
    //             let current = o;
    //             let next = v.get(i + 1).unwrap();
    //             let new = current.merge_with(next);
    //             v[i] = new;
    //             v.remove(i + 1);
    //             return match v.iter().any(|o| match o {
    //                 Operation::Multiply(_) => true,
    //                 _ => false,
    //             }) && v.len() > 1
    //             {
    //                 true => Self(v).calculate_mul(),
    //                 false => Self(v),
    //             };
    //         }
    //     }
    //     Self(v)
    // }

    // fn calculate(&self) -> i64 {
    //     self.calculate_mul()
    //         .0
    //         .iter()
    //         .fold(0, |sum, current| sum + current.to_number())
    // }
    fn calculate(&self) -> i64 {
        self.0.iter().fold(Operation::Multiply(1), |sum, current| sum.merge_with(current)).to_number()
    }
}

impl Equation {
    fn all_operations(&self) -> Vec<Operations> {
        let len = self.numbers.len();
        let result_len = 2_i64.pow(len.try_into().unwrap());
        (0..result_len)
            .map(|n| {
                let bytes = format!("{:b}", n);
                let zeros = String::from("0").chars().cycle().take(len - bytes.len()).collect::<String>();
                let bytes = zeros + &bytes;
                Operations(
                    bytes
                        .chars()
                        .enumerate()
                        .map(|(i, c)| match c {
                            '0' => Operation::Plus(self.numbers[i]),
                            '1' => Operation::Multiply(self.numbers[i]),
                            _ => panic!("Error finding combinations"),
                        })
                        .collect::<Vec<Operation>>(),
                )
            })
            .filter(|operations| match operations.0.last().unwrap() {
                Operation::Plus(_) => true,
                Operation::Multiply(_) => false,
            })
            .collect()
    }

    fn valid(&self) -> bool {
        self.all_operations()
            .into_iter()
            .filter(|o| o.calculate() == self.expected)
            .count() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn check_merge() {
        let input1 = Operation::Plus(5);
        let input2 = Operation::Plus(5);
        let expected = 10;
        let result = input1.merge_with(&input2).to_number();
        assert_eq!(expected, result);

        let input1 = Operation::Plus(5);
        let input2 = Operation::Plus(10);
        let expected = 15;
        let result = input1.merge_with(&input2).to_number();
        assert_eq!(expected, result);

        let input1 = Operation::Multiply(5);
        let input2 = Operation::Plus(10);
        let expected = 50;
        let result = input1.merge_with(&input2).to_number();
        assert_eq!(expected, result);

        let input1 = Operation::Multiply(5);
        let input2 = Operation::Multiply(10);
        let expected = Operation::Multiply(50);
        let result = input1.merge_with(&input2);
        assert_eq!(expected, result);
    }

    #[test]
    fn check_calculate() {
        let input = Operations(vec![
            Operation::Plus(5),
            Operation::Plus(5),
            Operation::Plus(5),
        ]);
        let expected = 15;
        let result = input.calculate();
        assert_eq!(result, expected);

        let input = Operations(vec![
            Operation::Multiply(5),
            Operation::Plus(5),
            Operation::Plus(5),
        ]);
        let expected = 30;
        let result = input.calculate();
        assert_eq!(result, expected);

        // let input = Operations(vec![
        //     Operation::Plus(5),
        //     Operation::Multiply(5),
        //     Operation::Plus(3),
        // ]);
        // let expected = 20;
        // let result = input.calculate();
        // assert_eq!(result, expected);

        let input = Operations(vec![
            Operation::Plus(11),
            Operation::Multiply(6),
            Operation::Plus(16),
            Operation::Plus(20),
        ]);
        let expected = 292;
        let result = input.calculate();
        assert_eq!(result, expected);
    }

    #[test]
    fn check_valid() {
        let input = Equation {
            expected: 190,
            numbers: vec![19, 10],
        };
        assert!(input.valid());

        // let input = Equation {
        //     expected: 292,
        //     numbers: vec![11, 6, 16, 20],
        // };
        // assert!(input.valid());

        let input = Equation {
            expected: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(input.valid());
    }

    #[test]
    #[ignore]
    fn test_input_to_answer() {
        let input = "292: 11 6 16 20";
        let expected = 292;
        let result = input_to_answer(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn check_all_operations() {
        // let input = Equation {
        //     expected: 190,
        //     numbers: vec![19, 100],
        // };
        // let expected = vec![
        //     Operations(vec![
        //         Operation::Plus(19),
        //         Operation::Plus(100),
        //     ]),
        //     Operations(vec![
        //         Operation::Multiply(19),
        //         Operation::Plus(100),
        //     ])
        // ];
        // let result = input.all_operations();
        // assert_eq!(result, expected);
        let input = Equation {
            expected: 292,
            numbers: vec![11, 6, 16, 20],
        };
        dbg!(input.all_operations());
        assert!(false)
    }
}

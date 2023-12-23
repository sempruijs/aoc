fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> i32 {
    s.lines()
        .map(|s| Sequence::from(s).next_value())
        .sum::<i32>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sequence(Vec<i32>);

impl From<&str> for Sequence {
    fn from(line: &str) -> Self {
        Sequence(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }
}

impl Sequence {
    fn next_value(&self) -> i32 {
        let mut sequences: Vec<Sequence> = vec![self.clone()];
        loop {
            let last_sequence = sequences.last().unwrap();
            if last_sequence.is_zero() {
                return sequences
                    .iter()
                    .map(|s| s.0.last().unwrap())
                    .rev()
                    .fold(0, |acc, x| acc + x);
                // return 0;
            }
            let new_sequence = last_sequence.div();
            sequences.push(new_sequence);
        }
    }

    fn div(&self) -> Self {
        let numbers = &self.0;
        let mut result: Vec<i32> = Vec::new();

        for (i, n) in numbers.iter().enumerate() {
            if i > 0 {
                let previous = numbers[i - 1];
                result.push(*n - previous);
            }
        }
        Self(result)
    }

    fn is_zero(&self) -> bool {
        self.0
            .iter()
            .filter(|n| n != &&0)
            .collect::<Vec<&i32>>()
            .is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::Sequence;

    #[test]
    fn test_is_zero() {
        let input_1 = Sequence(vec![0, 0, 1, 0]);
        let input_2 = Sequence(vec![0, 0, 0, 0]);
        assert!(!input_1.is_zero());
        assert!(input_2.is_zero());
    }

    #[test]
    fn test_div() {
        let input_1 = Sequence(vec![0, 3, 6, 9]);
        let expected_1 = Sequence(vec![3, 3, 3]);
        let result_1 = input_1.div();
        assert_eq!(result_1, expected_1);
    }
}

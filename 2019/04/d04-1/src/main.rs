fn main() {
    let input = "372304-847060";
    let answer = puzzle_input_to_result(input);
    println!("{}", answer);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Digits(Vec<u32>);

impl From<u32> for Digits {
    fn from(n: u32) -> Self {
        let digits: Vec<u32> = n
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        Self(digits)
    }
}

fn input_to_range(s: &str) -> Vec<u32> {
    let (start, end) = s.split_once("-").unwrap();
    let start: u32 = start.parse().unwrap();
    let end: u32 = end.parse().unwrap();
    (start..end).collect()
}

fn puzzle_input_to_result(s: &str) -> usize {
    input_to_range(s).into_iter().filter(valid_number).count()
}

fn valid_number(n: &u32) -> bool {
    let digits = Digits::from(*n);
    digits.has_nabour() && digits.does_increase()
}

impl Digits {
    fn does_increase(&self) -> bool {
        let mut acc = 0;
        for n in &self.0 {
            if &acc > n {
                return false;
            }
            acc = *n;
        }
        true
    }

    fn has_nabour(&self) -> bool {
        let mut acc = 42;
        for n in &self.0 {
            if &acc == n {
                return true;
            }
            acc = *n;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_range() {
        let input_1 = "1-3";
        let expected_1 = vec![1, 2];
        let result_1 = input_to_range(input_1);
        assert_eq!(result_1, expected_1);
    }

    #[test]
    fn test_to_digits() {
        let input_1 = 123;
        let expected_1 = Digits(vec![1, 2, 3]);
        let result_1 = Digits::from(input_1);
        assert_eq!(result_1, expected_1);
    }
}

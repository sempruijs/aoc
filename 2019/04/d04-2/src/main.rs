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
    digits.has_even_nabour() && digits.does_increase()
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

    fn has_even_nabour(&self) -> bool {
        let chunks = chunk_by_elements(self.0.clone());

        for chunk in chunks {
            if chunk.len() == 2 {
                return true;
            }
        }
        false
    }
}

fn chunk_by_elements<T: PartialEq>(data: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current_chunk = Vec::new();

    for item in data {
        if let Some(last_item) = current_chunk.last() {
            if item != *last_item {
                result.push(current_chunk);
                current_chunk = Vec::new();
            }
        }
        current_chunk.push(item);
    }

    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
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

    #[test]
    fn test_chunk_by_element() {
        let input_1 = vec![1, 1, 1, 3, 2];
        let expected_1 = vec![vec![1, 1, 1], vec![3], vec![2]];
        let result_1 = chunk_by_elements(input_1);
        assert_eq!(expected_1, result_1);
    }

    #[test]
    fn test_even_nabour() {
        let input_1 = 112233;
        let input_2 = 123444;
        let input_3 = 111122;

        let result_1 = Digits::from(input_1).has_even_nabour();
        let result_2 = Digits::from(input_2).has_even_nabour();
        let result_3 = Digits::from(input_3).has_even_nabour();

        assert!(result_1);
        assert!(!result_2);
        assert!(result_3);
    }
}

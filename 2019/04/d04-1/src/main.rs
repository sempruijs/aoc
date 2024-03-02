fn main() {
    let input = "372304-847060";
    let answer = puzzle_input_to_result(input);
    println!("{}", answer);
}

fn input_to_range(s: &str) -> Vec<u32> {
    let (start, end) = s.split_once("-").unwrap();
    let start: u32 = start.parse().unwrap();
    let end: u32 = end.parse().unwrap();
    (start..end).collect()
}

fn puzzle_input_to_result(s: &str) -> u32 {
    input_to_range(s).into_iter().filter(valid_number).sum()
}

fn valid_number(n: &u32) -> bool {
    true
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
}

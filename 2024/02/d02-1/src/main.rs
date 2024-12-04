fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(&input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let reports = input_to_reports(&s);
    reports.iter().filter(|r| r.valid()).count()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = Report(vec![7, 6, 4, 2, 1]);
        let expected = true;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 2, 7, 8, 9]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![9, 7, 6, 2, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 3, 2, 4, 5]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![8, 6, 4, 4, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 3, 6, 7, 9]);
        let expected = true;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 3, 6, 6, 7]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 1, 1, 1, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 0, 0, 0, 0]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 2, 3, 4, 5]);
        let expected = true;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![5, 5, 2, 1, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![5, 4, 3, 4, 5]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 2, 3, 2, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 2, 3, 7, 8]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 2]);
        let expected = true;
        let result = input.valid();
        assert_eq!(expected, result);

        let input = Report(vec![1, 1]);
        let expected = false;
        let result = input.valid();
        assert_eq!(expected, result);
    }

    #[test]
    fn check_input_to_answer() {
        let input = include_str!("../../example.txt");
        let result = input_to_answer(&input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_to_report() {
        let input = "29 27 23 20 17";
        let result = line_to_report(input);
        let expected = Report(vec![29, 27, 23, 20, 17]);
        assert_eq!(result, expected);

        let input = "29 27";
        let result = line_to_report(input);
        let expected = Report(vec![29, 27]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_program() {
        let input = include_str!("../../input.txt");
        let answer = input_to_answer(input);
        assert_eq!(answer, 224);
    }
}

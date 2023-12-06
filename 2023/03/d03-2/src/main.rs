use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn from(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// a bind is a character that make numbers valid for counting
struct Bind(Point);

struct Bindables(Vec<Bind>);

#[derive(Debug)]
struct Number {
    value: u32,
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Numbers(Vec<Number>);

impl Bindables {
    pub fn from_str(s: &str) -> Self {
        let char_matrix: Vec<Vec<char>> = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let mut result: Vec<Bind> = Vec::new();

        for (y, line) in char_matrix.into_iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c != &'.' && !c.is_digit(10) {
                    result.push(Bind(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    }));
                }
            }
        }
        Bindables(result)
    }
}

impl Numbers {
    // one HELL of a state machine should be tested carefully.
    pub fn from_str(s: &str) -> Self {
        let mut result: Vec<Number> = Vec::new();

        let mut previous_char = '.';
        let mut num_string = String::new();

        let mut start_point = Point::origin();
        let mut end_point = Point::origin();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let previous_is_digit = previous_char.is_digit(10);
                let current_is_digit = c.is_digit(10);

                if !previous_is_digit && current_is_digit {
                    // numbers start here
                    num_string = String::from("");
                    num_string.push(c);
                    start_point = Point::from(x as i32, y as i32);
                } else if previous_is_digit && current_is_digit {
                    // reading a number
                    num_string.push(c);
                } else if previous_is_digit && !current_is_digit {
                    // here the number ends
                    end_point = Point::from((x - 1) as i32, y as i32);
                    let n = Number {
                        value: num_string.parse().unwrap(),
                        start: start_point,
                        end: end_point,
                    };
                    result.push(n);
                    num_string = String::from("");
                }

                if x == line.len() - 1 && current_is_digit {
                    // last char here, so the number ends
                    end_point = Point::from(x as i32, y as i32);
                    let n = Number {
                        value: num_string.parse().unwrap(),
                        start: start_point,
                        end: end_point,
                    };

                    result.push(n);
                    num_string = String::from("");
                }
                previous_char = c;
            }
            previous_char = '.';
        }
        dbg!(&result);
        Numbers(result)
    }
}

impl Number {
    pub fn can_bind(&self, bindables: &Bindables) -> bool {
        let bindable_points: Vec<Point> = bindables.0.iter().map(|b| b.0).collect();
        for p in self.bindable_positions() {
            if bindable_points.contains(&p) {
                return true;
            };
        }
        false
    }

    fn bindable_positions(&self) -> Vec<Point> {
        let x_positions = (self.start.x - 1)..=(self.end.x + 1);
        let top: Vec<Point> = x_positions
            .clone()
            .map(|x| Point::from(x, self.start.y - 1))
            .collect();

        let bottom: Vec<Point> = x_positions
            .clone()
            .map(|x| Point::from(x, self.start.y + 1))
            .collect();

        let y = self.start.y;
        let (left, right) = (
            Point::from(self.start.x - 1, y),
            Point::from(self.end.x + 1, y),
        );

        let mut result: Vec<Point> = vec![left, right];
        result.extend(&top);
        result.extend(&bottom);

        result
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let numbers = Numbers::from_str(s);
    let bindables = Bindables::from_str(s);
    let valid_numbers: Vec<u32> = numbers
        .0
        .into_iter()
        .filter(|n| n.can_bind(&bindables))
        .map(|n| n.value)
        .collect();

    valid_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_from_str() {
        let str_1 = "123...";
        let str_2 = "...123";
        let str_3 = "1.2.3.";
        let str_4 = ".1.2.3";

        let nums_1_sum: u32 = Numbers::from_str(str_1).0.iter().map(|n| n.value).sum();
        let nums_2_sum: u32 = Numbers::from_str(str_2).0.iter().map(|n| n.value).sum();
        let nums_3_sum: u32 = Numbers::from_str(str_3).0.iter().map(|n| n.value).sum();
        let nums_4_sum: u32 = Numbers::from_str(str_4).0.iter().map(|n| n.value).sum();

        assert_eq!(nums_1_sum, 123);
        assert_eq!(nums_2_sum, 123);
        assert_eq!(nums_3_sum, 6);
        assert_eq!(nums_4_sum, 6);
    }
}

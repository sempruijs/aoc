use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Debug)]
struct Gear(Point);

#[derive(Debug)]
struct Gears(Vec<Gear>);

#[derive(Debug)]
struct Number {
    value: u32,
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Numbers(Vec<Number>);

impl Gears {
    pub fn from_str(s: &str) -> Self {
        let char_matrix: Vec<Vec<char>> = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let mut result: Vec<Gear> = Vec::new();

        for (y, line) in char_matrix.into_iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c == &'*' {
                    result.push(Gear(Point {
                        x: x as i32,
                        y: y as i32,
                    }));
                }
            }
        }
        Gears(result)
    }
}

impl Gear {
    pub fn ratio(&self, numbers: &Numbers) -> Option<u32> {
        let mut binding_values: Vec<u32> = Vec::new();
        let gear_bind_points = self.bind_locations();

        for number in &numbers.0 {
            let has_intersection = number
                .locations()
                .intersection(&gear_bind_points)
                .collect::<Vec<&Point>>()
                .len()
                > 0;

            if has_intersection {
                binding_values.push(number.value);
            }
        }
        if binding_values.len() == 2 {
            let ratio = binding_values[0] * binding_values[1];
            return Some(ratio);
        }
        None
    }

    fn bind_locations(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for x in self.0.x - 1..=self.0.x + 1 {
            for y in self.0.y - 1..=self.0.y + 1 {
                if x != self.0.x || self.0.y != y {
                    let p = Point::from(x, y);
                    result.insert(p);
                }
            }
        }

        result
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
        Numbers(result)
    }
}

impl Number {
    pub fn locations(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        let y = self.start.y;
        (self.start.x..=self.end.x).for_each(|x| {
            let p = Point::from(x, y);
            result.insert(p);
        });

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
    Gears::from_str(s)
        .0
        .iter()
        .filter_map(|g| Some(g.ratio(&numbers)).unwrap())
        .sum()
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

    #[test]
    fn test_number_locations() {
        let n = Number {
            value: 42,
            start: Point { x: 3, y: 3 },
            end: Point { x: 5, y: 3 },
        };

        let mut locations = HashSet::new();
        locations.insert(Point::from(3, 3));
        locations.insert(Point::from(4, 3));
        locations.insert(Point::from(5, 3));

        assert_eq!(n.locations(), locations);
    }

    #[test]
    fn test_gear_bind_locations() {
        let gear_bind_locations = Gear(Point { x: 3, y: 3 }).bind_locations();
        let mut expected_output: HashSet<Point> = HashSet::new();
        expected_output.insert(Point::from(2, 2));
        expected_output.insert(Point::from(2, 3));
        expected_output.insert(Point::from(2, 4));

        expected_output.insert(Point::from(3, 2));
        expected_output.insert(Point::from(3, 4));

        expected_output.insert(Point::from(4, 2));
        expected_output.insert(Point::from(4, 3));
        expected_output.insert(Point::from(4, 4));

        // dbg!(&expected_output);
        // dbg!(&gear_bind_locations);

        assert_eq!(gear_bind_locations, expected_output);
    }
}

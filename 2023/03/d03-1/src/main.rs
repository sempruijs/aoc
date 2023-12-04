use std::{collections::HashMap, fmt::Display};

struct Point {
    x: u32,
    y: u32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn origin() -> Self {
        Point {
            x: 0,
            y: 0,
        }
    }

    pub fn from(x: &u32, y: &u32) -> Self {
        Point {
            x: *x,
            y: *y,
        }
    }
}

// a bind is a character that make numbers valid for counting
struct Bind(Point);

struct Bindables(Vec<Bind>);

struct Number {
    value: u32,
    start: Point,
    end: Point,
}

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

        for (y, line) in s.lines() {
            for (x, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    num_string.push(c);

                    // if previous is NOT a digit
                    if !previous_char.is_digit(10) {
                        // here the number will begin
                        start_point = Point::from(&x,&y);
                    } else if x == line.len() {
                        //push point
                        end_point = Point::from(&x, &y);
                        num_string = String::from("");

                        let n = Number {
                            value: num_string.parse::<u32>().unwrap(),
                            start: start_point,
                            end: end_point,
                        }
                        result.push(n);
                    }
                } else {
                    // when it is not a digit

                    if previous_char.is_digit(10) {
                        // here the number ends, so it should be pushed
                        // to the list
                        end_point = Point::from(&(x - 1), &y);
                        let n = Number {
                            value: num_string.parse::<u32>().unwrap(),
                            start: start_point,
                            end: end_point,
                        }
                        // mmmm jammie
                        result.push(n);
                    } 
                    previous_char = c;
                }
                // because a new line is comming
                previous_char = '.';
            }
        }
        Numbers(result)
    }
}

impl Number {
    pub fn can_bind(&self, matrix: &Bindables) -> bool {
        true
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

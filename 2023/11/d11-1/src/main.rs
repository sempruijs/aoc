use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let galaxy = Galaxy::from_str(s).add_lightyear_distance();
    galaxy.sum_distances()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Galaxy(Vec<Point>);

impl Galaxy {
    fn from_str(s: &str) -> Self {
        let mut result = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    result.push(p);
                }
            }
        }
        Self(result)
    }

    fn add_lightyear_row(&self, transpose: bool) -> Self {
        let numbers: HashSet<i32> = match transpose {
            false => HashSet::from_iter(self.0.clone().iter().map(|p| p.y)),
            true => HashSet::from_iter(self.0.clone().iter().map(|p| p.x)),
        };

        let (min_n, max_n) = (numbers.iter().min().unwrap(), numbers.iter().max().unwrap());
        let total_numbers: HashSet<i32> = HashSet::from_iter(*min_n..=*max_n);
        let mut empty_row_locations = total_numbers
            .difference(&numbers)
            .map(|n| *n)
            .collect::<Vec<i32>>();
        empty_row_locations.sort();

        let mut numbers = self.0.clone();
        for n in empty_row_locations {
            for number in numbers.iter_mut() {
                match transpose {
                    true => {
                        if number.x < n {
                            number.x -= 1;
                        }
                    }
                    false => {
                        if number.y < n {
                            number.y -= 1;
                        }
                    }
                }
            }
        }

        Self(numbers)
    }

    fn add_lightyear_distance(&self) -> Self {
        self.add_lightyear_row(false).add_lightyear_row(true)
    }

    fn sum_distances(&self) -> u32 {
        let pairs = self.to_pairs();
        pairs.iter().map(|p| distance(&p.0, &p.1)).sum::<u32>()
    }

    fn to_pairs(&self) -> Vec<(Point, Point)> {
        let points = &self.0;
        let mut result = Vec::new();
        for (i, first) in points.iter().enumerate() {
            for second in &points[i..] {
                let pair: (Point, Point) = (*first, *second);
                result.push(pair);
            }
        }
        result
    }
}

fn distance(p1: &Point, p2: &Point) -> u32 {
    let y_div = match p1.y >= p2.y {
        true => p1.y - p2.y,
        false => p2.y - p1.y,
    };
    let x_div: i32 = match p1.x >= p2.x {
        true => p1.x - p2.x,
        false => p2.x - p1.x,
    };

    (y_div + x_div).abs() as u32
}

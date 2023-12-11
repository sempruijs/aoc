use std::iter::Flatten;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let size = input_to_size(s);
    let galaxy = Galaxy::from_str(s);
    galaxy.sum_distances()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Galaxy(Vec<Point>);

fn input_to_size(s: &str) -> (u32, u32) {
    let y = s.lines().count();
    let x = s.lines().next().unwrap().chars().count();

    (x as u32, y as u32)
}

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

    fn add_lightyear_distance_row(&self, size: (u32, u32), transpose: bool) -> Self {
        let grid = match transpose {
            false => self.as_grid(size),
            true => transpose_vec(self.as_grid(size)),
        };

        let mut result = grid.clone();
        for (i, row) in grid.iter().enumerate() {
            if row.is_empty() {
                for (y, row) in grid.iter().enumerate() {
                    for (x, c) in row.iter().enumerate() {
                        result[y][x] = Point { x: c.x, y: c.y - 1 };
                    }
                }
            }
        }
        Self(result.into_iter().flatten().collect::<Vec<Point>>())
    }

    fn add_lightyear_distance(&self, size: (u32, u32)) -> Self {
        let transformed_row = self.add_lightyear_distance_row(size, false);
        transformed_row.add_lightyear_distance_row(size, true)
    }

    fn sum_distances(&self) -> u32 {
        let pairs = self.to_pairs();
        pairs.iter().map(|p| distance(&p.0, &p.1)).sum::<u32>()
    }

    fn as_grid(&self, size: (u32, u32)) -> Vec<Vec<Point>> {
        let mut result = (0..(size.0))
            .map(|_| Vec::new())
            .collect::<Vec<Vec<Point>>>();
        for p in &self.0 {
            result[p.y as usize].push(*p);
        }
        result
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

fn transpose_vec<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

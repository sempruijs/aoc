use std::collections::HashSet;

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



    fn add_lightyear_row(&self, transpose: bool) -> Self {
        let numbers: HashSet<i32> = match transpose {
            false => HashSet::from_iter(self.0.clone().iter().map(|p| p.y)),
            true => HashSet::from_iter(self.0.clone().iter().map(|p| p.x)),
        };
        
        let (min_n, max_n) = (numbers.iter().min().unwrap(), numbers.iter().max().unwrap());
        let total_numbers: HashSet<i32> = HashSet::from_iter(*min_n..=*max_n);
        let mut empty_row_locations = total_numbers.intersection(&numbers).map(|n| *n).collect::<Vec<i32>>();
        empty_row_locations.sort();

        let mut numbers = self.0.clone();
        for n in empty_row_locations {
            for number in numbers.iter_mut() {
                if number.y < n {
                    number.y -= 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_vec() {
        let v1 = vec![
            vec![1,1,1],
            vec![2,2,2],
            vec![3,3,3],
        ];

        let expected_1 = vec![
            vec![1,2,3],
            vec![1,2,3],
            vec![1,2,3],
        ];
        let r1 = transpose_vec(v1);
        assert_eq!(r1, expected_1);
    }
}

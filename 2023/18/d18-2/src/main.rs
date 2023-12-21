use shoelace::*;

struct Points(Vec<Point>);

impl Points {
    fn line_surface_area(&self) -> u64 {
        let points = &self.0;
        let mut result = 0;
        for (i, p) in points.iter().enumerate() {
            if i > 0 {
                let previous_point = points[i - 1];
                let distance = distance(&previous_point, &p);
                result += distance;
            }
        }
        result
    }
}

fn distance(p1: &Point, p2: &Point) -> u64 {
    if p1.x == p2.x {
        return difference(p1.y, p2.y);
    } else if p1.y == p2.y {
        return difference(p1.x, p2.x);
    }
    panic!("cannot calculate distance because p1 and p2 create a diagnal line");
}

fn difference(a: i64, b: i64) -> u64 {
    let (small, big) = match a >= b {
        true => (b, a),
        false => (a, b),
    };
    (big - small).try_into().unwrap()
}

impl From<&str> for Points {
    fn from(s: &str) -> Self {
        let mut points: Vec<Point> = vec![Point::from((0, 0))];

        for line in s.lines() {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let hex_amount = words[2]
                .split_once("#")
                .unwrap()
                .1
                .split_once(")")
                .unwrap()
                .0;
            let dir = hex_amount.chars().last().unwrap();
            let hex_distance = &hex_amount[..5];
            let distance = i64::from_str_radix(hex_distance, 16).unwrap();
            let last_point = points.last().expect("No last point found");

            let p = match dir {
                '3' => Point {
                    x: last_point.x,
                    y: last_point.y + distance,
                },
                '1' => Point {
                    x: last_point.x,
                    y: last_point.y - distance,
                },
                '2' => Point {
                    x: last_point.x - distance,
                    y: last_point.y,
                },
                '0' => Point {
                    x: last_point.x + distance,
                    y: last_point.y,
                },
                _ => panic!("Could not parse dir"),
            };
            points.push(p);
        }
        Points(points)
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> f64 {
    let points = Points::from(s);
    let area = Area::from(points.0).0;
    let points = Points::from(s);
    let line_distance = points.line_surface_area();
    area + (line_distance / 2 + 1) as f64
}

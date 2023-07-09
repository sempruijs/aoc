use std::{fmt::Display, include_str};

#[derive(Debug, PartialEq)]
struct Point(i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(self.0, self.1)")
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point(self.0, self.1)
    }
}

#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn dir(&self) -> Dir {
        let (p1, p2) = (&self.0, &self.1);

        if p1.0 == p2.0 {
            Dir::Vertical
        } else if p1.1 == p2.1 {
            Dir::Horizontal
        } else {
            panic!(
                "Could not decide direction based on p1: {} p2: {}",
                &p1, &p2
            );
        }
    }
}

#[derive(Debug)]
struct Shape(Vec<Line>);

#[derive(Debug, PartialEq)]
enum Step {
    Y(i32),
    X(i32),
}

#[derive(Debug, PartialEq)]
enum Dir {
    Horizontal,
    Vertical,
}

fn step_to_line(step: Step, from_point: &Point) -> Line {
    let (x, y) = (from_point.0, from_point.1);

    match step {
        Step::Y(steps) => Line(Point(x, y), Point(x, y + steps)),
        Step::X(steps) => Line(Point(x, y), Point(x + steps, y)),
    }
}

// fn steps_to_line(steps: Vec<Step>) -> Line {
//     let mut result: Vec<Point> = vec![Point(0, 0)];

//     for step in steps {
//         let from_point = result.last().unwrap();
//         result.extend(step_to_points(step, from_point));
//     }

//     Line(result)
// }

fn point_to_distance(p: &Point) -> u32 {
    let (x, y) = (p.0, p.1);
    x.unsigned_abs() + y.unsigned_abs()
}

fn str_to_steps(s: &str) -> Vec<Step> {
    let str_steps: Vec<&str> = s.split(",").collect::<Vec<&str>>();
    let steps: Vec<Step> = str_steps.iter().map(|s| str_to_step(s)).collect();

    steps
}

// str should be something like D26, L4
fn str_to_step(s: &str) -> Step {
    let (dir, steps) = s.split_at(1);
    let steps: i32 = steps.parse().unwrap();
    match dir {
        "U" => Step::Y(steps),
        "R" => Step::X(steps),
        "D" => Step::Y(-steps),
        "L" => Step::X(-steps),
        _ => panic!("could not convert dir with intput: {}", dir),
    }
}

fn lines_to_intersection(l1: &Line, l2: &Line) -> Option<Point> {
    if l1.dir() == l2.dir() {
        //their is a possibility that this skips an important intersection
        // because two lines in the same direction can overlap
        return None;
    }
    //this will bind the horizontal line to h_line and vertical to v_line
    let (h_line, v_line) = if l1.dir() == Dir::Horizontal {
        (l1, l2)
    } else {
        (l2, l1)
    };

    let x = v_line.0 .0;
    let y = h_line.0 .1;

    Some(Point(x, y))
}

fn steps_to_shape(steps: Vec<Step>) -> Shape {
    let mut result: Vec<Line> = Vec::new();
    let mut from_point = Point(0, 0);

    for step in steps {
        let line = step_to_line(step, &from_point);
        from_point = line.1.clone();

        result.push(line);
    }

    Shape(result)
}

fn str_to_shortest_distance(s: &str) -> u32 {
    let s = s.lines().collect::<Vec<&str>>();

    let steps_1 = str_to_steps(s[0]);
    let steps_2 = str_to_steps(s[1]);

    let shape_1 = steps_to_shape(steps_1);
    let shape_2 = steps_to_shape(steps_2);
    dbg!(&shape_1);
    dbg!(&shape_2);

    let mut intersections: Vec<Point> = Vec::new();

    for l1 in shape_1.0 {
        for l2 in &shape_2.0 {
            match lines_to_intersection(&l1, l2) {
                Some(p) => intersections.push(p),
                None => {}
            }
        }
    }

    let mut distances: Vec<u32> = intersections.iter().map(point_to_distance).collect();

    distances.sort();
    distances[1]
}

fn main() {
    let puzzle_lines = include_str!("../../input.txt");

    let result = str_to_shortest_distance(puzzle_lines);
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_to_distance() {
        let point = Point(-1, 2);
        assert_eq!(point_to_distance(&point), 3);
    }

    #[test]
    fn test_str_to_step() {
        let str = "D26";
        let step = str_to_step(str);
        assert_eq!(step, Step::Y(-26));
    }

    #[test]
    fn test_step_to_line() {
        let origin = Point(0, 0);
        let step = Step::X(5);

        let line = step_to_line(step, &origin);
        assert_eq!(line, Line(origin, Point(5, 0)));
    }

    #[test]
    fn test_steps_to_lines() {
        let steps = vec![Step::X(5), Step::Y(-3)];
        let shape = steps_to_shape(steps);

        assert_eq!(
            vec![
                Line(Point(0, 0), Point(5, 0)),
                Line(Point(5, 0), Point(5, -3))
            ],
            shape.0
        );
    }

    #[test]
    fn test_lines_to_intersection() {
        let l1 = Line(Point(0, 0), Point(5, 0));
        let l2 = Line(Point(2, 3), Point(2, -3));

        let intersection = lines_to_intersection(&l1, &l2);

        assert_eq!(intersection, Some(Point(2, 0)));
    }

    #[test]
    fn test_str_to_distance() {
        let input_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let distance_1 = str_to_shortest_distance(input_1);
        assert_eq!(distance_1, 159);
    }
}

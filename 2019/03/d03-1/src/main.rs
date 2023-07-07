use std::include_str;

#[derive(Debug, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, PartialEq)]
struct Line(Vec<Point>);

#[derive(Debug, PartialEq)]
struct Intersections(Vec<Point>);

#[derive(Debug, PartialEq)]
enum Step {
    Y(i32),
    X(i32),
}

fn step_to_points(step: Step, from_point: &Point) -> Vec<Point> {
    let (x, y) = (from_point.0, from_point.1);

    let mut points: Vec<Point> = Vec::new();

    match step {
        Step::Y(steps) => {
            let dir: i32 = if steps > 0 { 1 } else { -1 };

            // the + dir for the off by 1 error, but it had to be in the right direction, so I could use dir for it
            for i in 0..positive(steps) {
                points.push(Point(x, y + dir + dir * i));
            }
        }
        Step::X(steps) => {
            let dir: i32 = if steps > 0 { 1 } else { -1 };

            for i in 0..positive(steps) {
                // the + dir for the off by 1 error, but it had to be in the right direction, so I could use dir for it
                points.push(Point(x + dir + dir * i, y));
            }
        }
    }

    points
}

fn steps_to_line(steps: Vec<Step>) -> Line {
    let mut result: Vec<Point> = vec![Point(0, 0)];

    for step in steps {
        let from_point = result.last().unwrap();
        result.extend(step_to_points(step, from_point));
    }

    Line(result)
}

fn point_to_distance(p: &Point) -> u32 {
    let (x, y) = (p.0, p.1);
    (positive(x) + positive(y)).try_into().unwrap()
}

fn positive(x: i32) -> i32 {
    if x < 0 {
        x * -1
    } else {
        x
    }
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

fn lines_to_intersections(l1: Line, l2: Line) -> Intersections {
    let mut result: Vec<Point> = Vec::new();

    for a in &l1.0 {
        for b in &l2.0 {
            if a == b {
                result.push(Point(a.0, a.1));
            }
        }
    }

    Intersections(result)
}

fn str_to_line(s: &str) -> Line {
    let steps = str_to_steps(s);
    let line = steps_to_line(steps);

    line
}

fn main() {
    let lines = include_str!("../../input.txt")
        .lines()
        .collect::<Vec<&str>>();
    let l1 = str_to_line(lines[0]);
    let l2 = str_to_line(lines[1]);

    let intersections: Intersections = lines_to_intersections(l1, l2);
    let distance: u32 = point_to_distance(&intersections.0[1]);
    println!("{}", distance);
    // println!("{}", puzzel_input);
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn test_positive() {
        assert_eq!(positive(-5), 5);
        assert_eq!(positive(5), 5);
    }

    #[test]
    fn test_point_to_distance() {
        let point = Point(-1, 2);
        assert_eq!(point_to_distance(point), 3);
    }

    #[test]
    fn test_lines_to_intersections() {
        let steps1 = vec![Step::X(10)];
        let steps2 = vec![Step::Y(5), Step::X(5), Step::Y(-10)];

        let line1 = steps_to_line(steps1);
        let line2 = steps_to_line(steps2);

        let intersections = lines_to_intersections(line1, line2);
        assert_eq!(intersections.0, vec![Point(0, 0), Point(5, 0)]);
    }

    #[test]
    fn test_str_to_step() {
        let str = "D26";
        let step = str_to_step(str);
        assert_eq!(step, Step::Y(-26));
    }
}

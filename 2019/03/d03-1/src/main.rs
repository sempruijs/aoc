mod parser;

pub enum Step {
    X(i32),
    Y(i32),
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Point {
    fn from(x: &i32, y: &i32) -> Point {
        Point { x: *x, y: *y }
    }

    pub fn origin() -> Self {
        Point::from(&0, &0)
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Step {
    pub fn to_line(&self, from_point: Point) -> Line {
        let new_point = match self {
            Step::X(steps) => Point::from(&(from_point.x + steps), &from_point.y),
            Step::Y(steps) => Point::from(&from_point.x, &(from_point.y + steps)),
        };

        Line::from(&from_point, &new_point)
    }
}

impl Line {
    fn from(p1: &Point, p2: &Point) -> Self {
        let dir_line = Line {
            p1: p1.clone(),
            p2: p2.clone(),
        };

        let (small_p, big_p) = match &p1.x > &p2.x || &p1.y > &p2.y {
            true => (p2, p1),
            false => (p1, p2),
        };

        Line {
            p1: small_p.clone(),
            p2: big_p.clone(),
        }
    }

    fn dir(&self) -> Dir {
        if self.p1.x == self.p2.x {
            return Dir::Vertical;
        }
        Dir::Horizontal
    }
}

fn steps_to_lines(steps: Vec<Step>) -> Vec<Line> {
    let mut from_point = Point::origin();
    let mut lines: Vec<Line> = Vec::new();

    for step in steps {
        let line = step.to_line(from_point);
        from_point = line.p2.clone();
        lines.push(line);
    }

    lines
}

fn main() {
    let puzzel_input = include_str!("../../input.txt");
    println!("{}", puzzel_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir() {
        // vertical line
        let l1 = Line {
            p1: Point::from(&-3, &-3),
            p2: Point::from(&-3, &5),
        };

        assert_eq!(l1.dir(), Dir::Vertical);
    }

    #[test]
    fn test_line_from() {
        let p1 = Point::from(&0, &0);
        let p2 = Point::from(&0, &-5);
        let result = Line::from(&p1.clone(), &p2.clone());
        assert_eq!(result, Line { p1: p2, p2: p1 });
    }

    #[test]
    fn test_step_to_line() {
        let step = Step::X(5);
        let from_point = Point::from(&0, &0);
        let result = step.to_line(from_point.clone());
        let expected_result = Line::from(&from_point, &Point::from(&5, &0));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_steps_to_lines() {
        let steps = vec![Step::X(5), Step::Y(-3)];
        let result = steps_to_lines(steps);
        let expected_points = vec![Point::origin(), Point::from(&5, &0), Point::from(&5, &-3)];
        let expected_result: Vec<Line> = vec![
            Line::from(&expected_points[0], &expected_points[1]),
            Line::from(&expected_points[1], &expected_points[2]),
        ];

        assert_eq!(result, expected_result);
    }
}

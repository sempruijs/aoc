mod parser;

pub enum Step {
    X(i32),
    Y(i32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Copy)]
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

    pub fn axis(&self, dir: &Dir) -> i32 {
        match dir {
            Dir::Horizontal => self.x,
            Dir::Vertical => self.y,
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

impl Clone for Line {
    fn clone(&self) -> Self {
        Line::from(&self.p1, &self.p2)
    }
}

impl Line {
    fn from(p1: &Point, p2: &Point) -> Self {
        if p1.x != p2.x && p1.y != p2.y {
            panic!("Failed to create line baesd on the folowing inputs:\n\n p1: ({}, {})\n p2: ({},{})\n\n This wil create an diagonal line, which is not allowed.", p1.x, p1.y, p2.x, p2.y);
        }

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

    fn to_points(&self) -> Vec<Point> {
        let dir = self.dir();
        let range = (self.p1.axis(&dir), self.p2.axis(&dir));

        let mut result: Vec<Point> = Vec::new();

        for i in (range.0)..(range.1 + 1) {
            let p = match dir {
                Dir::Horizontal => Point::from(&i, &self.p1.y),
                Dir::Vertical => Point::from(&self.p1.x, &i),
            };

            result.push(p);
        }

        result
    }

    // I choose option instead of returning an empty vec because I found that more readable
    fn get_intersections_with(&self, l: &Line) -> Option<Vec<Point>> {
        if self.has_intersection_with(l) {
            if self.dir() == l.dir() {
                let overlapping_line = self.get_overlapping(&l).unwrap();
                let points = overlapping_line.to_points();

                // Some Vec<p>
                return Some(points);
            }

            let (h_line, v_line) = match self.dir() {
                Dir::Horizontal => (*self, *l),
                Dir::Vertical => (*l, *self),
            };

            let x = v_line.p1.x;
            let y = h_line.p1.y;
            let p = Point::from(&x, &y);

            return Some(vec![p]);
        }
        None
    }

    fn has_intersection_with(&self, l: &Line) -> bool {
        if self.dir() == l.dir() {
            let dir = self.dir();
            let (low_l, high_l) = match self.p2.axis(&dir) < l.p2.axis(&dir) {
                true => (self.clone(), *l),
                false => (*l, *self),
            };

            return low_l.p2.axis(&dir) >= high_l.p1.axis(&dir);
        }

        let (h_line, v_line) = match self.dir() {
            Dir::Horizontal => (*self, *l),
            Dir::Vertical => (*l, *self),
        };

        let between_x = h_line.p1.y >= v_line.p1.y && h_line.p1.y <= v_line.p2.y;
        let between_y = v_line.p1.x >= h_line.p1.x && v_line.p1.x <= h_line.p2.x;

        between_x && between_y
    }

    // gets the overlapping line of two lines that have the same direction
    fn get_overlapping(&self, l: &Line) -> Option<Line> {
        if self.has_intersection_with(l) {
            if self.dir() != l.dir() {
                panic!("Lines do not have the same direction");
            }

            let dir = self.dir();

            let (low_l, high_l) = match self.p2.axis(&dir) < l.p2.axis(&dir) {
                true => (self, l),
                false => (l, self),
            };

            // if a line completely overlaps the other, it should return the smallest line
            if high_l.p1.axis(&dir) <= low_l.p1.axis(&dir) {
                return Some(*low_l);
            } else if low_l.p2.axis(&dir) >= high_l.p2.axis(&dir) {
                return Some(*high_l);
            }

            let result = match dir {
                Dir::Horizontal => Line::from(
                    &Point::from(&high_l.p1.x, &self.p1.y),
                    &Point::from(&low_l.p2.x, &self.p1.y),
                ),
                Dir::Vertical => Line::from(
                    &Point::from(&self.p1.x, &low_l.p2.y),
                    &Point::from(&self.p1.x, &high_l.p1.y),
                ),
            };

            return Some(result);
        }

        None
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
    #[should_panic]
    fn test_diagnoal_from_line() {
        let p1 = Point::origin();
        let p2 = Point::from(&2, &2);
        let line = Line::from(&p1, &p2);
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

    #[test]
    pub fn test_point_dir() {
        let p = Point::from(&5, &3);
        let r1 = p.axis(&Dir::Horizontal);
        let r2 = p.axis(&Dir::Vertical);

        assert_eq!(r1, 5);
        assert_eq!(r2, 3);
    }

    #[test]
    fn test_has_intersection() {
        // is an vertical line |
        let l1 = Line::from(&Point::from(&0, &3), &Point::from(&0, &6));

        // is an horizontal line --
        let l2 = Line::from(&Point::from(&-5, &0), &Point::from(&5, &0));

        // should be false, no intersection
        let r1 = l1.has_intersection_with(&l2);
        assert!(!r1);

        // is an horizontal line --
        let l3 = Line::from(&Point::from(&-5, &0), &Point::from(&5, &0));

        // is an horizontal line --
        let l4 = Line::from(&Point::from(&3, &0), &Point::from(&10, &0));

        // should be true, has intersection
        let r2 = l3.has_intersection_with(&l4);
        assert!(r2);

        // is an vertical line |
        let l5 = Line::from(&Point::from(&0, &-5), &Point::from(&0, &5));

        // is an vertical line |
        let l6 = Line::from(&Point::from(&0, &-10), &Point::from(&0, &-5));

        // should be true, has intersection
        let r3 = l5.has_intersection_with(&l6);
        assert!(r3);

        // these are the same line compared to eachother,
        // so it should be true
        let r4 = l5.has_intersection_with(&l5);
        assert!(r4);

        // should be false
        let r5 = l4.has_intersection_with(&l6);
        assert!(!r5);

        let l7 = Line::from(&Point::from(&-5, &0), &Point::from(&5, &0));
        let l8 = Line::from(&Point::from(&0, &-5), &Point::from(&0, &5));
        let r6 = l7.has_intersection_with(&l8);
        assert!(r6);
    }

    #[test]
    fn test_line_to_points() {
        let l = Line::from(&Point::from(&-5, &0), &Point::from(&-3, &0));
        let result = l.to_points();
        let expected_result = vec![
            Point::from(&-5, &0),
            Point::from(&-4, &0),
            Point::from(&-3, &0),
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_overlapping() {
        let l1 = Line::from(&Point::from(&0, &-5), &Point::from(&0, &5));
        let l2 = Line::from(&Point::from(&0, &4), &Point::from(&0, &7));
        let r1 = l1.get_overlapping(&l2);
        let expected_r1 = Some(Line::from(&Point::from(&0, &4), &Point::from(&0, &5)));

        assert_eq!(r1, expected_r1);

        let l3 = Line::from(&Point::from(&0, &-5), &Point::from(&0, &3));
        let r2 = l1.get_overlapping(&l3);
        let expected_r2 = Some(l3);

        assert_eq!(r2, expected_r2);
    }

    #[test]
    fn test_get_intersections_with() {
        let l1 = Line::from(&Point::from(&-5, &0), &Point::from(&5, &0));
        let l2 = Line::from(&Point::from(&0, &-5), &Point::from(&0, &5));
        let r1 = l1.get_intersections_with(&l2);
        let expected_r1 = Some(vec![Point::from(&0, &0)]);

        assert_eq!(r1, expected_r1);
    }
}

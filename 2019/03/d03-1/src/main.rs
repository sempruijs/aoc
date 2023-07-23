enum Steps {
    X(i32),
    Y(i32),
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    Horizontal,
    Vertical,
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Point {
    fn from(x: &i32, y: &i32) -> Point {
        Point { x: *x, y: *y }
    }
}

impl Line {
    // fn from(p1: &Point, p2: &Point) -> Self {
    // }

    fn dir(&self) -> Dir {
        if self.p1.x == self.p2.x {
            return Dir::Vertical;
        }
        Dir::Horizontal
    }
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
}

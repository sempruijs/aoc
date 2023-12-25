use std::collections::HashMap;
use std::convert::From;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let matrix = Matrix::from(s);
    let start_dir = Dir::West;
    matrix.distance(&start_dir)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Dir::North => "^",
            Dir::East => ">",
            Dir::South => "v",
            Dir::West => "<",
        };
        write!(f, "{}", s)
    }
}

impl Dir {
    fn opesite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Pipe {
    fn output(&self, dir: &Dir) -> Option<Dir> {
        if &self.hole_a.opesite() == dir {
            return Some(self.hole_b);
        } else if &self.hole_b.opesite() == dir {
            return Some(self.hole_a);
        }
        None
    }
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn move_to(&self, d: &Dir) -> Self {
        let (x, y) = (self.x, self.y);

        match d {
            Dir::North => Point { x, y: y - 1 },
            Dir::East => Point { x: x + 1, y },
            Dir::South => Point { x, y: y + 1 },
            Dir::West => Point { x: x - 1, y },
        }
    }
}

#[derive(Debug, Hash, Clone)]
struct Pipe {
    hole_a: Dir,
    hole_b: Dir,
}

#[derive(Debug)]
struct Matrix {
    entries: HashMap<Point, Pipe>,
    start: Point,
}

impl From<&str> for Matrix {
    fn from(s: &str) -> Self {
        let mut m: HashMap<Point, Pipe> = HashMap::new();
        let mut start = Point::origin();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point {
                    x: x as u32,
                    y: y as u32,
                };

                match Pipe::try_from(c) {
                    Ok(pipe) => {
                        m.insert(p, pipe);
                    }
                    Err(s) => {
                        if s == "S" {
                            start = p;
                        }
                    }
                };
            }
        }
        Self {
            entries: m.clone(),
            start,
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Pipe {
                hole_a: Dir::North,
                hole_b: Dir::South,
            }),
            '-' => Ok(Pipe {
                hole_a: Dir::East,
                hole_b: Dir::West,
            }),
            'L' => Ok(Pipe {
                hole_a: Dir::North,
                hole_b: Dir::East,
            }),
            'J' => Ok(Pipe {
                hole_a: Dir::North,
                hole_b: Dir::West,
            }),
            '7' => Ok(Pipe {
                hole_a: Dir::West,
                hole_b: Dir::South,
            }),
            'F' => Ok(Pipe {
                hole_a: Dir::East,
                hole_b: Dir::South,
            }),
            '.' => Err("."),
            'S' => Err("S"),
            _ => panic!("Could not parse character to pipe."),
        }
    }
}

impl Matrix {
    fn distance(&self, start_dir: &Dir) -> u32 {
        let mut pipe_count = 1;
        let mut next_point: Point = self.start.move_to(start_dir);
        let mut dir = start_dir.clone();

        while next_point != self.start {
            // println!("{}", pipe_count);
            let pipe = self.entries.get(&next_point).unwrap();
            let out_dir = pipe.output(&dir).unwrap_or_else(|| {
                dbg!(&pipe);
                dbg!(&dir);
                println!("{}", pipe_count);
                println!("{}", pipe_count);
                panic!("does not work");
            });
            dir = out_dir;
            next_point = next_point.move_to(&dir);
            pipe_count += 1;
        }

        pipe_count / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_output() {
        let pipe_1 = Pipe::try_from('L').unwrap();
        let result_1 = pipe_1.output(&Dir::West).unwrap();
        let expected_1 = Dir::North;

        assert_eq!(expected_1, result_1);

        let pipe_2 = Pipe::try_from('L').unwrap();
        let result_2 = pipe_2.output(&Dir::South).unwrap();
        let expected_2 = Dir::East;

        assert_eq!(expected_2, result_2);

        let pipe_3 = Pipe::try_from('-').unwrap();
        let result_3 = pipe_3.output(&Dir::East).unwrap();
        let expected_3 = Dir::East;

        assert_eq!(expected_3, result_3);
    }

    #[test]

    fn test_point_move_to_dir() {
        let p = Point { x: 5, y: 5 };
        let north = p.move_to(&Dir::North);
        let east = p.move_to(&Dir::East);
        let south = p.move_to(&Dir::South);
        let west = p.move_to(&Dir::West);

        let expect_north = Point { x: 5, y: 4 };
        let expect_east = Point { x: 6, y: 5 };
        let expect_south = Point { x: 5, y: 6 };
        let expect_west = Point { x: 4, y: 5 };

        assert_eq!(north, expect_north);
        assert_eq!(east, expect_east);
        assert_eq!(south, expect_south);
        assert_eq!(west, expect_west);
    }
}

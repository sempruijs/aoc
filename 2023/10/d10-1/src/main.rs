use std::collections::HashMap;
use std::convert::From;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let matrix = Matrix::from(s);
    matrix.distance()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn opesite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Pipe {
    fn get_io(&self, dir: &Dir) -> Option<(Dir, Dir)> {
        if self.hole_a == dir.opesite() {
            return Some((*dir, self.hole_b));
        } else if self.hole_b == dir.opesite() {
            return Some((*dir, self.hole_b));
        }
        None
    }
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn get(&self, d: &Dir) -> Self {
        let (x, y) = (self.x, self.y);

        match d {
            Dir::North => Point { x, y: y + 1 },
            Dir::East => Point { x: x + 1, y },
            Dir::South => Point { x, y: y - 1 },
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
            's' => Err("S"),
            _ => panic!("Could not parse character to pipe."),
        }
    }
}

impl Matrix {
    fn distance(&self) -> u32 {
        todo!()
    }

    fn start_as_pipe(&self) -> Pipe {
        let start = &self.start;
        let surounding_points = vec![Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .map(|d| start.get(d))
            .collect::<Vec<Point>>();

        let surrounding_pipes = surounding_points
            .iter()
            .filter_map(|p| self.entries.get(p))
            .collect::<Vec<&Pipe>>();
    }
}

#![allow(warnings)]
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

const size: u16 = 70;
const blocks: usize = 1024;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> u32 {
    let w = World::try_from(s).unwrap();
    println!("{w}");
    w.answer()
}

struct World {
    g: UnGraph<Point, u32>,
    start: Point,
    end: Point,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 0..=size {
            for x in 0..=size {
                let p = Point { x, y };
                if self.g.neighbors(p.into()).count() > 0 {
                    result.push('.');
                } else {
                    result.push('#');
                }
            }
            result.push_str("\n\r");
        }
        write!(f, "{result}")
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

struct Points(Vec<Point>);

impl From<Point> for NodeIndex {
    fn from(p: Point) -> Self {
        (((p.x as u32) << 16) | (p.y as u32)).into()
    }
}

impl From<NodeIndex> for Point {
    fn from(value: NodeIndex) -> Self {
        let value: u32 = value.index().try_into().unwrap();
        let x = (value >> 16) as u16;
        let y = (value & 0xFFFF_FFFF) as u16;
        Self { x, y }
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let points = Points::try_from(s).unwrap().get_amount(blocks);
        let mut edges: Vec<(Point, Point)> = Vec::new();
        for y in 0..=size {
            for x in 0..=size {
                let directions = vec![Dir::North, Dir::East, Dir::South, Dir::West];
                let p1 = Point {
                    x: x as u16,
                    y: y as u16,
                };
                for dir in directions {
                    if let Some(p2) = p1.transform(&dir) {
                        if !points.0.contains(&p1) && !points.0.contains(&p2) {
                            edges.push((p1, p2));
                        }
                    }
                }
            }
        }
        let g = UnGraph::<Point, u32>::from_edges(&edges);
        let start = Point { x: 0, y: 0 };
        let end = Point {
            x: size as u16,
            y: size as u16,
        };
        Ok(Self { g, start, end })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Point {
    fn transform(&self, dir: &Dir) -> Option<Self> {
        if (self.x == 0 && dir == &Dir::West) || (self.y == 0 && dir == &Dir::North) {
            return None;
        }
        let p = match dir {
            Dir::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Dir::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        };
        Some(p)
    }
}

impl Points {
    fn get_amount(self, x: usize) -> Self {
        let mut result: Vec<Point> = self.0;
        result.split_off(x);
        Self(result)
    }
}

impl TryFrom<&str> for Points {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines()
                .map(|l| {
                    let (left, right) = l.split_once(",").unwrap();
                    Point {
                        x: left.parse::<u16>().unwrap(),
                        y: right.parse::<u16>().unwrap(),
                    }
                })
                .collect::<Vec<Point>>(),
        ))
    }
}

impl World {
    fn answer(&self) -> u32 {
        let dijkstra = dijkstra(&self.g, self.start.into(), None, |_| 1);
        *dijkstra.get(&self.end.into()).unwrap()
    }
}

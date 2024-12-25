use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::csr::IndexType;
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <-> {}", self.p1, self.p2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let directions = self
            .out
            .clone()
            .into_iter()
            .fold(String::new(), |mut result, dir| {
                let s = format!("{}", dir);
                result.push_str(&s);
                result
            });
        write!(f, "p: {}\n out: {}", self.p, directions)
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::Up => "^",
            Dir::Right => ">",
            Dir::Down => "v",
            Dir::Left => "<",
        };
        write!(f, "{c}")
    }
}

struct World {
    g: UnGraph<u32, u32>,
    start: Point,
    end: Point,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn transform(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn index(&self) -> u32 {
        self.x * 100 + self.y
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn cost(&self) -> u32 {
        self.p1.x.abs_diff(self.p2.x) + self.p1.y.abs_diff(self.p2.y) + 1000
    }

    fn edge(&self) -> (u32, u32, u32) {
        (self.p1.index(), self.p2.index(), self.cost())
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Corner {
    p: Point,
    out: Vec<Dir>,
}

#[derive(Debug, Clone)]
struct Corners(HashSet<Corner>);

impl TryFrom<&str> for Corners {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ps: HashSet<Point> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'.' || c == &'S' || c == &'E')
                    .map(|(x, _)| Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();
        let mut result: HashSet<Corner> = HashSet::new();
        for p in &ps {
            let left_point = ps.get(&p.transform(&Dir::Left));
            let right_point = ps.get(&p.transform(&Dir::Right));
            let down_point = ps.get(&p.transform(&Dir::Down));
            let up_point = ps.get(&p.transform(&Dir::Up));
            if !(left_point.is_some()
                && right_point.is_some()
                && up_point.is_none()
                && down_point.is_none())
                && !(left_point.is_none()
                    && right_point.is_none()
                    && up_point.is_some()
                    && down_point.is_some())
            {
                // add all directions;
                let directions: Vec<Dir> = vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left]
                    .into_iter()
                    .filter(|d| ps.get(&p.transform(d)).is_some())
                    .collect();
                let corner = Corner {
                    p: p.clone(),
                    out: directions,
                };
                result.insert(corner);
            }
        }
        Ok(Self(result))
    }
}

struct Lines(HashSet<Line>);

impl From<Corners> for Lines {
    fn from(corners: Corners) -> Self {
        let mut lines = HashSet::new();
        for corner in &corners.0 {
            let mut corners: Vec<Corner> = corners
                .clone()
                .filter_on_x(corner.p.x)
                .0
                .into_iter()
                .collect();
            radsort::sort_by_key(&mut corners, |c: &Corner| c.p.y);
            let corners: Vec<Corner> = corners.into_iter().filter(|c| c.p.y > corner.p.y).collect();
            'inner: for corner2 in corners {
                if corner2.out.contains(&Dir::Up) {
                    let line = Line {
                        p1: corner.p.clone(),
                        p2: corner2.p.clone(),
                    };
                    lines.insert(line);
                } else {
                    break 'inner;
                }
            }
        }
        for corner in &corners.0 {
            let mut corners: Vec<Corner> = corners
                .clone()
                .filter_on_y(corner.p.y)
                .0
                .into_iter()
                .collect();
            radsort::sort_by_key(&mut corners, |c: &Corner| c.p.x);
            let corners: Vec<Corner> = corners.into_iter().filter(|c| c.p.x > corner.p.x).collect();
            'inner: for corner2 in corners {
                if corner2.out.contains(&Dir::Left) {
                    let line = Line {
                        p1: corner.p.clone(),
                        p2: corner2.p.clone(),
                    };
                    lines.insert(line);
                } else {
                    break 'inner;
                }
            }
        }
        Self(lines)
    }
}

impl Corners {
    fn filter_on_x(&self, x: u32) -> Self {
        Self(
            self.0
                .clone()
                .into_iter()
                .filter(|corner| corner.p.x == x)
                .collect(),
        )
    }

    fn filter_on_y(&self, y: u32) -> Self {
        Self(
            self.0
                .clone()
                .into_iter()
                .filter(|corner| corner.p.y == y)
                .collect(),
        )
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let corners = Corners::try_from(s).unwrap();
        let edges: &[(u32, u32, u32)] = &Lines::from(corners)
            .0
            .into_iter()
            .map(|l| l.edge())
            .collect::<Vec<(u32, u32, u32)>>();
        let g = UnGraph::<u32, u32>::from_edges(edges);
        let start = Point {
            x: 1,
            y: s.lines().count() as u32 - 2,
        };
        let end = Point {
            x: s.lines().next().unwrap().chars().count() as u32 - 2,
            y: 1,
        };
        Ok(Self {
            g: g,
            start: start,
            end: end,
        })
    }
}

impl World {
    fn answer(&self) -> u32 {
        let node_map = dijkstra(&self.g, self.start.index().into(), None, |e| *e.weight());
        *node_map
            .get(&NodeIndex::new(self.end.index().try_into().unwrap()))
            .unwrap()
    }
}

fn input_to_answer(s: &str) -> u32 {
    World::try_from(s).unwrap().answer()
}

// #![allow(warnings)]

use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {}", answer);
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <-> {}", self.p1, self.p2)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result =
            self.0
                .clone()
                .into_iter()
                .map(Point::from)
                .fold(String::new(), |mut result, p| {
                    result.push_str(&format!(" {p}"));
                    result
                });
        write!(f, "{}", result)
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
    g: UnGraph<Point, u32>,
    start: Point,
    end: Point,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl From<Point> for NodeIndex {
    fn from(p: Point) -> Self {
        (((p.x as u32) << 16) | (p.y as u32)).into()
    }
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,
}

impl From<NodeIndex> for Point {
    fn from(value: NodeIndex) -> Self {
        let value: u32 = value.index().try_into().unwrap();
        let x = (value >> 16) as u16;
        let y = (value & 0xFFFF_FFFF) as u16;
        Self { x, y }
    }
}

impl Line {
    fn cost(&self) -> u32 {
        (self.p1.x.abs_diff(self.p2.x) + self.p1.y.abs_diff(self.p2.y) + 1000) as u32
    }

    fn edge(&self) -> (Point, Point, u32) {
        (self.p1.clone(), self.p2.clone(), self.cost())
    }

    fn points(&self) -> Vec<Point> {
        // determen direction
        match self.p1.x == self.p2.x {
            true => {
                // vertical
                let (small, big) = match self.p1.y < self.p2.y {
                    true => (self.p1, self.p2),
                    false => (self.p2, self.p1),
                };
                return (small.y..=big.y)
                    .map(|y| Point { x: small.x, y: y })
                    .collect();
            }
            false => {
                // horizontal
                let (small, big) = match self.p1.x < self.p2.x {
                    true => (self.p1, self.p2),
                    false => (self.p2, self.p1),
                };
                return (small.x..=big.x)
                    .map(|x| Point { x: x, y: small.y })
                    .collect();
            }
        }
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
    fn filter_on_x(&self, x: u16) -> Self {
        Self(
            self.0
                .clone()
                .into_iter()
                .filter(|corner| corner.p.x == x)
                .collect(),
        )
    }

    fn filter_on_y(&self, y: u16) -> Self {
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
        let edges: &[(Point, Point, u32)] = &Lines::from(corners)
            .0
            .into_iter()
            .map(|l| l.edge())
            .collect::<Vec<(Point, Point, u32)>>();
        let g = UnGraph::<Point, u32>::from_edges(edges);
        let start = Point {
            x: 1,
            y: s.lines().count() as u16 - 2,
        };
        let end = Point {
            x: s.lines().next().unwrap().chars().count() as u16 - 2,
            y: 1,
        };
        Ok(Self {
            g: g,
            start: start,
            end: end,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path(Vec<NodeIndex>);

impl Path {
    fn push(self, i: NodeIndex) -> Self {
        let mut result = self;
        result.0.push(i);
        result
    }

    fn contains(&self, i: &NodeIndex) -> bool {
        self.0.contains(i)
    }

    fn cost(&self, w: &World) -> u32 {
        let slice = &self.0[..];
        slice
            .windows(2)
            .into_iter()
            .map(|window| {
                w.g.edge_weight(w.g.find_edge(window[0], window[1]).unwrap())
                    .unwrap()
            })
            .sum()
    }

    fn points(&self) -> Vec<Point> {
        let slice = &self.0[..];
        slice
            .windows(2)
            .flat_map(|window| {
                Line {
                    p1: window[0].into(),
                    p2: window[1].into(),
                }
                .points()
            })
            .collect()
    }

    fn last(&self) -> NodeIndex {
        self.0.iter().last().unwrap().clone()
    }
}

impl World {
    fn shortest_paths(
        &self,
        path: Path,
        max_len: u32,
        dijkstra: &HashMap<NodeIndex, u32>,
    ) -> Vec<Path> {
        let cost = path.cost(self);
        if cost > max_len || dijkstra.get(&path.last()).unwrap() < &cost {
            return Vec::new();
        }
        if cost == max_len && path.last() == NodeIndex::from(self.end.clone()) {
            return vec![path];
        }
        self.g
            .neighbors(path.last())
            .flat_map(|neighbor| match path.contains(&neighbor) {
                true => vec![],
                false => {
                    let new_path = path.clone().push(neighbor);
                    self.shortest_paths(new_path, max_len, dijkstra)
                }
            })
            .collect()
    }

    fn answer(&self) -> u32 {
        let dijkstra = dijkstra(&self.g, self.start.into(), None, |e| *e.weight());
        let len = dijkstra.get(&self.end.into()).unwrap();
        println!("{len}");
        let start_path = Path(vec![self.start.into()]);
        self.shortest_paths(start_path, *len, &dijkstra)
            .into_iter()
            .flat_map(|p| p.points())
            .collect::<HashSet<Point>>()
            .len() as u32
    }
}

fn input_to_answer(s: &str) -> u32 {
    World::try_from(s).unwrap().answer()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_points() {
        let input = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 0, y: 3 },
        };
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ];
        let result = input.points();
        assert_eq!(result, expected);

        let input = Line {
            p1: Point { x: 0, y: 3 },
            p2: Point { x: 0, y: 0 },
        };
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ];
        let result = input.points();
        assert_eq!(result, expected);

        let input = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 3, y: 0 },
        };
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ];
        let result = input.points();
        assert_eq!(result, expected);

        let input = Line {
            p1: Point { x: 3, y: 0 },
            p2: Point { x: 0, y: 0 },
        };
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ];
        let result = input.points();
        assert_eq!(result, expected);
    }
}

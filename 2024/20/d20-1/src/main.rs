#![allow(warnings)]
use petgraph::algo::dijkstra;
use petgraph::algo::has_path_connecting;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,
}

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

impl Line {
    fn cost(&self) -> u32 {
        (self.p1.x.abs_diff(self.p2.x) + self.p1.y.abs_diff(self.p2.y) + 1000) as u32
    }

    fn edge(&self) -> (Point, Point, u32) {
        (self.p1.clone(), self.p2.clone(), self.cost())
    }
}

struct Lines(HashSet<Line>);

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

fn input_to_answer(s: &str) -> usize {
    let w = World::try_from(s).unwrap();
    let normal_time = w.shortest_path();
    w.to_cheat_worlds()
        .into_iter()
        .filter(|w| w.time_saved(normal_time) >= 100)
        .count()
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> Vec<Dir> {
        vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left]
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

impl World {
    fn to_cheat_worlds(&self) -> Vec<World> {
        todo!()
    }

    fn get_all_cheats_edges(&self) -> (Point, Point, u32) {
        todo!()
    }

    fn shortest_path(&self) -> usize {
        todo!()
    }

    fn time_saved(&self, normal_time: usize) -> usize {
        todo!()
    }
}

#![allow(warnings)]
use petgraph::algo::dijkstra;
use petgraph::algo::has_path_connecting;
use petgraph::graph::Node;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hm = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let p = Point {
                            x: x as u16,
                            y: y as u16,
                        };
                        (p, c)
                    })
                    .collect::<Vec<(Point, char)>>()
            })
            .collect::<HashMap<Point, char>>();
        let mut start = Point::default();
        let mut end = Point::default();
        let mut edges: Vec<(NodeIndex, NodeIndex, u32)> = Vec::new();

        for (p, c) in &hm {
            match c {
                'S' => {
                    start = *p;
                }
                'E' => {
                    end = *p;
                }
                '.' => {
                    Dir::all().into_iter().for_each(|d| {
                        if let Some(p2) = p.transform(&d) {
                            if hm.get(&p) == Some(&'.') {
                                edges.push(((*p).into(), p2.into(), 1));
                            }
                        }
                    });
                }
                '#' => continue,
                c => panic!("Found unknown character: {c}"),
            }
        }
        let g = UnGraph::<Point, u32>::from_edges(edges);
        Ok(Self { end, start, g })
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
    fn transform(&self, dir: &Dir) -> Option<Self> {
        if (dir == &Dir::Up && self.y == 0) || (dir == &Dir::Left && self.x == 0) {
            return None;
        }

        let result = match dir {
            Dir::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        };
        Some(result)
    }
}

impl World {
    fn to_cheat_worlds(&self) -> Vec<World> {
        self.get_all_cheats_edges()
            .into_iter()
            .map(|e| {
                let mut g = self.g.clone();
                g.add_edge(e.0.into(), e.1.into(), 2);
                World {
                    g,
                    start: self.start,
                    end: self.end,
                }
            })
            .collect()
    }

    fn get_all_cheats_edges(&self) -> Vec<(Point, Point)> {
        self.g
            .node_indices()
            .flat_map(|node_index| {
                let p = Point::from(node_index);
                Dir::all()
                    .into_iter()
                    .filter_map(|d| {
                        if let Some(p2) = p.transform(&d) {
                            if let Some(p2) = p2.transform(&d) {
                                return Some((p, p2));
                            }
                        }
                        None
                    })
                    .collect::<Vec<(Point, Point)>>()
            })
            .collect()
    }

    fn shortest_path(&self) -> usize {
        let dijkstra = dijkstra(&self.g, self.start.into(), None, |_| 1);
        *dijkstra.get(&self.end.into()).unwrap()
    }

    fn time_saved(&self, normal_time: usize) -> usize {
        normal_time - self.shortest_path()
    }
}

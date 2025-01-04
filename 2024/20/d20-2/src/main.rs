#![allow(warnings)]
use petgraph::algo::dijkstra;
use petgraph::algo::has_path_connecting;
use petgraph::algo::matching;
use petgraph::dot::{Config, Dot};
use petgraph::graph::Node;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Points(Vec<Point>);

impl From<&str> for Points {
    fn from(s: &str) -> Self {
        Self(
            s.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter(|(_, c)| c != &'#')
                        .map(|(x, _)| Point {
                            x: x as u16,
                            y: y as u16,
                        })
                        .collect::<Vec<Point>>()
                })
                .collect(),
        )
    }
}
fn main() {
    let input = include_str!("../../input.txt");
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

        let start = hm
            .iter()
            .filter(|(_, v)| v == &&'S')
            .next()
            .unwrap()
            .0
            .clone();

        let end = hm
            .iter()
            .filter(|(_, v)| v == &&'E')
            .next()
            .unwrap()
            .0
            .clone();

        let points: HashSet<Point> = hm
            .iter()
            .filter(|(_, v)| v != &&'#')
            .map(|(k, _)| k)
            .cloned()
            .collect();

        let edges: Vec<(NodeIndex, NodeIndex, u32)> = points
            .clone()
            .into_iter()
            .flat_map(|p| {
                Dir::all()
                    .iter()
                    .filter_map(|d| {
                        if let Some(p2) = p.transform(d) {
                            return match points.get(&p2) {
                                Some(p2) => Some((p.into(), (*p2).into(), 1)),
                                None => None,
                            };
                        }
                        None
                    })
                    .collect::<Vec<(NodeIndex, NodeIndex, u32)>>()
            })
            .collect();

        let g = UnGraph::<Point, u32>::from_edges(edges);
        Ok(Self { end, start, g })
    }
}

fn input_to_answer(s: &str) -> usize {
    let w = World::try_from(s).unwrap();
    let points = Points::from(s);
    w.fast_shortcut_amount(100, points)
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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

impl Point {
    fn distance(&self, p2: &Point) -> u16 {
        self.x.abs_diff(p2.x) + self.y.abs_diff(p2.y)
    }
}

impl World {
    fn fast_shortcut_amount(&self, minimum_time_saved: i32, points: Points) -> usize {
        let dijkstra = dijkstra(&self.g, self.start.into(), None, |_| 1);
        self.get_all_cheats_edges(points)
            .into_iter()
            .filter(|(p1, p2)| {
                if let Some(distance_a) = dijkstra.get(&(*p1).into()) {
                    if let Some(distance_b) = dijkstra.get(&(*p2).into()) {
                        let dif = distance_b - distance_a;
                        return dif - 2 >= minimum_time_saved;
                    }
                }
                false
            })
            .inspect(|(p1, p2)| println!("{p1} -> {p2}"))
            .count()
    }

    fn get_all_cheats_edges(&self, points: Points) -> Vec<(Point, Point)> {
        points
            .0
            .clone()
            .into_iter()
            .flat_map(|p1| {
                points
                    .0
                    .clone()
                    .into_iter()
                    .filter_map(|p2| match p1.distance(&p2) == 2 {
                        true => Some((p1, p2)),
                        false => None,
                    })
                    .collect::<Vec<(Point, Point)>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_node_index() {
        let p = Point { x: 0, y: 0 };
        let expected: NodeIndex = 0.into();
        let result = NodeIndex::from(p);
        assert_eq!(result, expected);

        let p = Point { x: 1, y: 0 };
        let expected: NodeIndex = 65536.into();
        let result = NodeIndex::from(p);
        assert_eq!(result, expected);

        let input: NodeIndex = 65536.into();
        let expected = Point { x: 1, y: 0 };
        let result = Point::from(input);
        assert_eq!(result, expected);
    }
}

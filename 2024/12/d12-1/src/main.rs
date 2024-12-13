use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

// this is all horible code. Please forgive me.
fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let world = World::try_from(s).unwrap();
    let shapes = Shapes::from(world);
    shapes.fence_prices()
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct Shape {
    points: HashSet<Point>,
    c: char,
}

#[derive(Debug)]
struct Shapes(Vec<Shape>);

impl TryFrom<&str> for Shapes {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            let p = Point {
                                x: x as i32,
                                y: y as i32,
                            };
                            let mut hs = HashSet::new();
                            hs.insert(p);
                            Shape { points: hs, c: c }
                        })
                        .collect::<Vec<Shape>>()
                })
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct World(HashMap<Point, char>);

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut hm = HashMap::new();
        s.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                hm.insert(point, c);
            })
        });
        Ok(Self(hm))
    }
}

impl From<World> for Shapes {
    fn from(w: World) -> Self {
        let mut w = w.clone();
        let mut shapes = Vec::new();
        while !w.0.is_empty() {
            if let Some(shape) = w.get_first_shape() {
                w = w.filter(&shape);
                shapes.push(shape);
            }
        }
        Self(shapes)
    }
}

impl World {
    fn filter(&self, shape: &Shape) -> Self {
        let mut hm = self.0.clone();
        shape.points.iter().for_each(|p| {
            hm.remove(p);
        });
        Self(hm)
    }

    fn get_first_shape(&self) -> Option<Shape> {
        let (p, c) = self.0.iter().next()?;
        let mut visited = HashSet::from([p.clone()]);
        self.make_shape(p, &mut visited, *c);
        Some(Shape {
            points: visited,
            c: *c,
        })
    }
}

impl World {
    fn make_shape(&self, p: &Point, mut visited: &mut HashSet<Point>, c: char) {
        let top = Point { x: p.x, y: p.y - 1 };
        let bottom = Point { x: p.x, y: p.y + 1 };
        let right = Point { x: p.x + 1, y: p.y };
        let left = Point { x: p.x - 1, y: p.y };
        vec![top, right, bottom, left].iter().for_each(|p| {
            if let Some(d) = self.0.get(&p) {
                if *d == c && !visited.contains(&p) {
                    visited.insert(p.clone());
                    self.make_shape(p, &mut visited, c);
                }
            }
        });
    }
}

impl Shapes {
    fn fence_prices(&self) -> usize {
        let ns: Vec<usize> = self.0.par_iter().map(Shape::fence_price).collect();
        ns.iter().sum()
    }
}

impl Shape {
    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> usize {
        self.points.iter().map(|p| 4 - self.neighbor_count(p)).sum()
    }

    fn neighbor_count(&self, p: &Point) -> usize {
        let top = Point { x: p.x, y: p.y - 1 };
        let bottom = Point { x: p.x, y: p.y + 1 };
        let right = Point { x: p.x + 1, y: p.y };
        let left = Point { x: p.x - 1, y: p.y };
        vec![top, right, bottom, left]
            .iter()
            .filter(|x| self.points.contains(x))
            .count()
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

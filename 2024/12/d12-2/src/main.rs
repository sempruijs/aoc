use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let world = World::try_from(s).unwrap();
    let shapes = Shapes::from(world.clone());
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

    // horrible function, please forgive me.
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
    fn is_diagonal(&self, p: &Point) -> bool {
        let top_left = Point {
            x: p.x - 1,
            y: p.y - 1,
        };
        let bottom_left = Point { x: p.x - 1, y: p.y };
        let top_right = Point { x: p.x, y: p.y - 1 };
        let bottom_right = Point { x: p.x, y: p.y };
        let br = self.points.contains(&bottom_right);
        let tr = self.points.contains(&top_right);
        let bl = self.points.contains(&bottom_left);
        let tl = self.points.contains(&top_left);
        (tl && br && !tr && !bl) || (!tl && !br && tr && bl)
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn sides(&self) -> usize {
        let corners = self
            .points
            .clone()
            .into_iter()
            .flat_map(|p| p.corners())
            .fold(HashMap::new(), |mut hm, p| {
                hm.entry(p).and_modify(|x| *x += 1).or_insert(1);
                hm
            });
        corners.iter().fold(0, |mut sum, (p, x)| {
            if x == &2 && self.is_diagonal(p) {
                sum += 2;
            }
            if x % 2 == 1 {
                sum += 1;
            }
            sum
        })
    }

    fn fence_price(&self) -> usize {
        let area = self.area();
        let sides = self.sides();
        let price = sides * area;
        println!(
            "shape {} has {} sides, an area of {}, total: {}",
            self.c, sides, area, price
        );
        price
    }
}

impl Point {
    fn corners(&self) -> Vec<Point> {
        let top_left = Self {
            x: self.x,
            y: self.y,
        };
        let bottom_left = Self {
            x: self.x,
            y: self.y + 1,
        };
        let top_right = Self {
            x: self.x + 1,
            y: self.y,
        };
        let bottom_right = Self {
            x: self.x + 1,
            y: self.y + 1,
        };
        vec![top_left, top_right, bottom_left, bottom_right]
    }
}

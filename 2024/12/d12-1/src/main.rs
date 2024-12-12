use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let shapes = Shapes::try_from(s).unwrap().merge();
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

impl Shapes {
    fn merge(mut self) -> Self {
        for (j, shape1) in self.0.iter().enumerate() {
            for (i, shape2) in self.0.iter().enumerate() {
                if shape1.c == shape2.c && shape1 != shape2 {
                    for point in shape1.points.clone() {
                        if shape2.neighbor_count(&point) > 0 {
                            let points: HashSet<Point> = shape1
                                .clone()
                                .points
                                .union(&shape2.points.clone())
                                .cloned()
                                .collect();
                            let shape = Shape {
                                points: points,
                                c: shape1.c,
                            };
                            self.0.remove(i);
                            self.0.remove(j);
                            self.0.push(shape);
                            return self.merge();
                        }
                    }
                }
            }
        }
        self
    }

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

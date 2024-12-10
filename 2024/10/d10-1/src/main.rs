use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let w = World::try_from(s).unwrap();
    w.routes_count()
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct World(HashMap<Point, u8>);

#[derive(Eq, Hash, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Point {
    fn apply(&self, dir: &Dir) -> Self {
        Self {
            x: match dir {
                Dir::East => self.x + 1,
                Dir::West => self.x - 1,
                _ => self.x,
            },
            y: match dir {
                Dir::North => self.y - 1,
                Dir::South => self.y + 1,
                _ => self.y,
            },
        }
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut hs = HashMap::new();
        s.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                let n = c.to_digit(10).unwrap() as u8;
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                hs.insert(p, n);
            })
        });
        Ok(Self(hs))
    }
}

impl World {
    fn routes_count(&self) -> usize {
        self.zeros()
            .iter()
            .map(|p| {
                let mut hs = HashSet::new();
                self.route_count(&mut hs, p);
                hs.len()
            })
            .sum()
    }

    fn route_count(&self, hs: &mut HashSet<Point>, p: &Point) {
        if let Some(x) = self.0.get(p) {
            let p_north = p.apply(&Dir::North);
            let p_east = p.apply(&Dir::East);
            let p_south = p.apply(&Dir::South);
            let p_west = p.apply(&Dir::West);
            let points = vec![p_north, p_east, p_south, p_west];
            points.iter().for_each(|p| {
                if let Some(y) = self.0.get(p) {
                    if *y == x + 1 {
                        if *y == 9 {
                            hs.insert(p.clone());
                        } else {
                            self.route_count(hs, p);
                        }
                    };
                }
            });
        }
    }

    fn zeros(&self) -> Vec<Point> {
        self.0.iter().fold(Vec::new(), |mut result, (k, v)| {
            if *v == 0 {
                result.push(k.clone());
            }
            result
        })
    }
}

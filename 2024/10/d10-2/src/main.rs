use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> u32 {
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
    fn routes_count(&self) -> u32 {
        self.zeros().iter().map(|p| self.route_count(p)).sum()
    }

    fn route_count(&self, p: &Point) -> u32 {
        if let Some(x) = self.0.get(p) {
            let p_north = p.apply(&Dir::North);
            let p_east = p.apply(&Dir::East);
            let p_south = p.apply(&Dir::South);
            let p_west = p.apply(&Dir::West);
            let points = vec![p_north, p_east, p_south, p_west];
            return points
                .iter()
                .map(|p| match self.0.get(p) {
                    Some(y) => match *y == x + 1 {
                        true => match *y == 9 {
                            true => 1,
                            false => self.route_count(p),
                        },
                        false => 0,
                    },
                    None => 0,
                })
                .sum::<u32>();
        } else {
            0
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
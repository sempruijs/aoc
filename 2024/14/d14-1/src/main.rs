use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {}", answer);
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p={},{} v={},{}", self.p.x, self.p.y, self.v.x, self.v.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    fn as_world_point(self, width: usize, height: usize) -> Self {
        Self {
            x: self.x.rem_euclid(width as i32),
            y: self.y.rem_euclid(height as i32),
        }
    }

    fn quadrant(&self, width: usize, height: usize) -> Option<usize> {
        let p = self.clone().as_world_point(width, height);
        let width: i32 = width.try_into().unwrap();
        let height: i32 = height.try_into().unwrap();

        // check for if a point is in a middel lane.
        if p.y == (height - 1) / 2 || p.x == (width - 1) / 2 {
            return None;
        }

        if p.x * 2 < width {
            if p.y * 2 < height {
                return Some(0);
            } else {
                return Some(2);
            }
        } else {
            if p.y * 2 < height {
                return Some(1);
            } else {
                return Some(3);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    p: Point,
    v: Point,
}

#[derive(Debug)]
struct World {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hm: HashMap<Point, usize> = HashMap::new();
        self.robots.clone().into_iter().for_each(|r| {
            let p = r.p.as_world_point(self.width, self.height);
            hm.entry(p).and_modify(|x| *x += 1).or_insert(1);
        });
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                }
                .as_world_point(self.width, self.height);
                if let Some(n) = hm.get(&p) {
                    result.push_str(&format!("{}", n));
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let height = 103;
        let width = 101;
        let robots: Vec<Robot> = s.lines().map(|l| Robot::try_from(l).unwrap()).collect();
        Ok(Self {
            height,
            width,
            robots,
        })
    }
}

impl TryFrom<&str> for Robot {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (left, right) = s.split_once(" ").unwrap();
        let (p_x, p_y) = left.split_once("=").unwrap().1.split_once(",").unwrap();
        let (v_x, v_y) = right.split_once("=").unwrap().1.split_once(",").unwrap();
        let p = Point {
            x: p_x.parse().unwrap(),
            y: p_y.parse().unwrap(),
        };
        let v = Point {
            x: v_x.parse().unwrap(),
            y: v_y.parse().unwrap(),
        };
        Ok(Self { p, v })
    }
}

impl Robot {
    fn next(self, width: usize, height: usize) -> Self {
        let p = Point {
            x: self.p.x + self.v.x,
            y: self.p.y + self.v.y,
        }
        .as_world_point(width, height);
        Self {
            p,
            v: self.v.clone(),
        }
    }
}

impl World {
    fn next(self) -> Self {
        let robots = self
            .robots
            .into_iter()
            .map(|r| r.next(self.width, self.height))
            .collect();
        Self {
            robots,
            width: self.width,
            height: self.height,
        }
    }

    fn answer(&self) -> usize {
        self.robots
            .iter()
            .fold(vec![0, 0, 0, 0], |mut v, r| {
                if let Some(i) = r.p.quadrant(self.width, self.height) {
                    v[i] += 1;
                }
                v
            })
            .iter()
            .fold(1, |result, n| if n > &0 { n * result } else { result })
    }
}

fn input_to_answer(s: &str) -> usize {
    let mut w = World::try_from(s).unwrap();
    for _ in 0..100 {
        w = w.next();
    }
    w.answer()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_as_world() {
        let height = 103;
        let width = 101;
        let input = Point { x: 4, y: 3 };
        let result = input.as_world_point(width, height);
        let expected = Point { x: 1, y: 3 };
        assert_eq!(result, expected);
    }
}

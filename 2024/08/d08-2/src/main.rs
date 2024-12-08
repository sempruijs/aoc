use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("{}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let w = World::from(s);
    let dimentions = dimentions(s);
    let freaquencies = frequencies(s);
    let points: HashSet<Point> = freaquencies
        .iter()
        .flat_map(|c| w.lines(c).block_points())
        .filter(|p| p.within(&dimentions))
        .collect();
    points.len()
}

struct World(HashSet<Attenna>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Attenna {
    p: Point,
    c: char,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for World {
    fn from(s: &str) -> Self {
        Self(
            s.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| {
                            let p = Point {
                                x: x as i32,
                                y: y as i32,
                            };
                            let c = c;
                            Attenna { p: p, c: c }
                        })
                        .filter(move |a| a.c != '.')
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line(Point, Point);

impl From<(&Point, &Point)> for Line {
    fn from((p1, p2): (&Point, &Point)) -> Self {
        match p1.y > p2.y {
            true => Self(p1.clone(), p2.clone()),
            false => Self(p2.clone(), p1.clone()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Lines(HashSet<Line>);

fn frequencies(s: &str) -> HashSet<char> {
    s.chars()
        .filter(|c| c != &'\n' || c != &'.')
        .map(|c| c)
        .collect()
}

fn dimentions(s: &str) -> (i32, i32) {
    let height: i32 = s.lines().count() as i32;
    let width: i32 = s.lines().next().unwrap().chars().count() as i32;
    (width, height)
}

impl Point {
    fn within(&self, dimentions: &(i32, i32)) -> bool {
        self.x >= 0 && self.x < dimentions.0 && self.y >= 0 && self.y < dimentions.1
    }
}

impl World {
    fn lines(&self, c: &char) -> Lines {
        let points: Vec<Point> = self
            .0
            .iter()
            .filter(|a| &a.c == c)
            .map(|a| a.p.clone())
            .collect();
        let mut hs = HashSet::new();
        for p1 in points.iter() {
            for p2 in points.iter() {
                if p1 != p2 {
                    let line = Line::from((p1, p2));
                    hs.insert(line);
                }
            }
        }
        Lines(hs)
    }
}

impl Line {
    fn block_points(&self) -> Vec<Point> {
        let diff_x = self.1.x - self.0.x;
        let diff_y = self.1.y - self.0.y;
        let p1 = Point {
            x: self.0.x - diff_x,
            y: self.0.y - diff_y,
        };
        let p2 = Point {
            x: self.1.x + diff_x,
            y: self.1.y + diff_y,
        };
        vec![p1, p2]
    }
}

impl Lines {
    fn block_points(&self) -> HashSet<Point> {
        let mut hs: HashSet<Point> = self.0.iter().flat_map(|l| l.block_points()).collect();
        let wrong_points: HashSet<Point> =
            self.0.clone().into_iter().fold(HashSet::new(), |mut v, l| {
                v.insert(l.0);
                v.insert(l.1);
                v
            });
        for p in wrong_points {
            hs.remove(&p);
        }
        hs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_within() {
        let dimentions = (5, 5);
        let input = Point { x: 3, y: 5 };
        assert!(!input.within(&dimentions));

        let dimentions = (5, 5);
        let input = Point { x: 3, y: 4 };
        assert!(input.within(&dimentions));
    }
}

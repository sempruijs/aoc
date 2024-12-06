use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");
    let world = World::try_from(input).unwrap();
    let player = Player::try_from(input).unwrap();
    let mut hs = HashSet::new();
    hs.insert(player.p.clone());
    let answer = player.visited_points(&hs, &world).len();
    println!("{}", answer);
}
#[derive(Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::Nothing),
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Nothing),
            _ => Err(()),
        }
    }
}
impl TryFrom<&str> for Player {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (p, _) = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|((_, _), c)| c == &'^')
            .next()
            .unwrap();
        Ok(Self {
            p: Point(p.0, p.1),
            dir: Dir::North,
        })
    }
}

enum Cell {
    Wall,
    Nothing,
}

struct Player {
    p: Point,
    dir: Dir,
}
#[derive(Eq, Hash, PartialEq, Clone)]
struct Point(usize, usize);
struct World(HashMap<Point, Cell>);

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut hm = HashMap::<Point, Cell>::new();
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| ((x, y), c))
                    .collect::<Vec<((usize, usize), char)>>()
            })
            .for_each(|((x, y), c)| {
                let p = Point(x, y);
                let cell = Cell::try_from(c).unwrap();
                hm.insert(p, cell);
            });
        Ok(Self(hm))
    }
}

impl Point {
    fn apply(&self, dir: &Dir) -> Option<Self> {
        match self.0 == 0 || self.1 == 0 {
            true => None,
            false => Some(match dir {
                Dir::North => Point(self.0, self.1 - 1),
                Dir::East => Point(self.0 + 1, self.1),
                Dir::South => Point(self.0, self.1 + 1),
                Dir::West => Point(self.0 - 1, self.1),
            }),
        }
    }
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::North => Self::East,
            Dir::East => Self::South,
            Dir::South => Self::West,
            Dir::West => Self::North,
        }
    }
}

impl Player {
    fn next(&self, w: &World) -> Option<Player> {
        let current = &self.p;
        let next = current.apply(&self.dir)?;
        let next_cell = w.0.get(&next)?;
        Some(match next_cell {
            Cell::Nothing => Player {
                p: next.clone(),
                dir: self.dir.clone(),
            },
            Cell::Wall => Player {
                p: current.clone(),
                dir: self.dir.turn_right(),
            }
            .next(w)?,
        })
    }

    fn visited_points(&self, hs: &HashSet<Point>, w: &World) -> HashSet<Point> {
        let mut hs = hs.clone();
        match self.next(&w) {
            Some(player) => {
                hs.insert(player.p.clone());
                player.visited_points(&hs, w)
            }
            None => hs,
        }
    }

    fn steps(&self, w: &World) -> usize {
        match self.next(w) {
            Some(p) => 1 + p.steps(w),
            None => 0,
        }
    }
}

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let (left, right) = s.split_once("\n\n").unwrap();
    let instructions = Instructions::try_from(right).unwrap();
    let world = World::try_from(left).unwrap();
    // world.interactive();
    let end_world = world.apply_instructions(&instructions);
    println!("{end_world}");
    end_world.answer()
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                match self.tiles.get(&p) {
                    None => {
                        if self.tiles.get(&Point { x: p.x - 1, y: p.y }) != Some(&Tile::Box) {
                            result.push('.');
                        }
                    }
                    Some(Tile::Box) => result.push_str("[]"),
                    Some(Tile::Wall) => result.push('#'),
                    Some(Tile::Player) => result.push('@'),
                }
            }
            result.push_str("\n\r");
        }
        write!(f, "{result}")
    }
}

impl Point {
    fn distance(&self) -> usize {
        (self.y * &100 + &self.x) as usize
    }

    fn transform(&self, i: &Instruction) -> Point {
        match i {
            Instruction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Instruction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Instruction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Instruction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl TryFrom<&str> for Instructions {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.chars()
                .filter(|c| c != &'\n')
                .map(|c| Instruction::from(&c))
                .collect::<Vec<Instruction>>(),
        ))
    }
}

impl From<&char> for Instruction {
    fn from(c: &char) -> Self {
        match c {
            &'^' => Instruction::North,
            &'v' => Instruction::South,
            &'<' => Instruction::West,
            &'>' => Instruction::East,
            _ => panic!("Unknown character to parse"),
        }
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut hm: HashMap<Point, Tile> = HashMap::new();
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let p1 = Point {
                    x: 2 * x as i32,
                    y: y as i32,
                };
                let p2 = Point {
                    x: 2 * x as i32 + 1,
                    y: y as i32,
                };
                match c {
                    '.' => continue,
                    '#' => {
                        hm.insert(p1, Tile::Wall);
                        hm.insert(p2, Tile::Wall);
                    }
                    'O' => {
                        hm.insert(p1, Tile::Box);
                    }
                    '@' => {
                        hm.insert(p1, Tile::Player);
                    }
                    c => panic!("Unknown char: {}", c),
                };
            }
        }
        Ok(Self {
            tiles: hm.clone(),
            width: width * 2,
            height: height,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Player,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Instruction {
    North,
    East,
    South,
    West,
}

struct Instructions(Vec<Instruction>);

struct Transaction {
    p: Point,
    tile: Option<Tile>,
}

#[derive(Debug, Clone)]
struct World {
    tiles: HashMap<Point, Tile>,
    width: usize,
    height: usize,
}

struct Transactions(Vec<Transaction>);

impl Instruction {
    fn rev(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
    fn to_transactions(&self, w: &World) -> Option<Transactions> {
        let mut transactions = vec![
            Transaction {
                p: w.player().transform(self),
                tile: Some(Tile::Player),
            },
            Transaction {
                p: w.player(),
                tile: None,
            },
        ];
        let mut next_points = HashSet::new();
        let next_point = w.player().transform(self);
        next_points.insert(next_point.clone());
        let vertical = self == &Instruction::North || self == &Instruction::South;
        loop {
            if !next_points.is_empty() {
                for next_point in next_points.clone() {
                    let current_tile: Option<Tile> =
                        w.tiles.get(&next_point.transform(&self.rev())).cloned();
                    match w.tiles.get(&next_point) {
                        None => {
                            if self == &Instruction::West
                                && w.tiles.get(&next_point.transform(self)) == Some(&Tile::Box)
                            {
                                let behind_box_point =
                                    &next_point.clone().transform(self).transform(self);
                                let t1 = Transaction {
                                    p: behind_box_point.clone(),
                                    tile: Some(Tile::Box),
                                };
                                let t2 = Transaction {
                                    p: next_point.transform(self).clone(),
                                    tile: None,
                                };
                                transactions.push(t1);
                                transactions.push(t2);
                                next_points.insert(next_point.transform(self).transform(self));
                                next_points.remove(&next_point);
                            }
                            let t = Transaction {
                                p: next_point.clone(),
                                tile: current_tile.clone(),
                            };
                            let left_point = next_point.transform(&Instruction::West);
                            if vertical && w.tiles.get(&left_point) == Some(&Tile::Box) {
                                next_points.insert(left_point);
                            }
                            transactions.push(t);
                            next_points.remove(&next_point);
                        }
                        Some(Tile::Wall) => return None,
                        Some(Tile::Player) => panic!("Found duplicate player"),
                        Some(Tile::Box) => {
                            if !vertical {
                                next_points.insert(next_point.transform(self).transform(self));
                                let t1 = Transaction {
                                    p: next_point.clone(),
                                    tile: current_tile.clone(),
                                };
                                let t2 = Transaction {
                                    p: next_point.transform(self),
                                    tile: w.tiles.get(&next_point).cloned(),
                                };
                                transactions.push(t1);
                                transactions.push(t2);
                                next_points.remove(&next_point);
                            } else {
                                let t = Transaction {
                                    p: next_point.clone(),
                                    tile: current_tile,
                                };
                                transactions.push(t);
                                next_points.insert(next_point.transform(self));
                            }
                            if vertical {
                                let right_point =
                                    next_point.transform(self).transform(&Instruction::East);
                                next_points.insert(right_point);
                            }
                            next_points.remove(&next_point);
                        }
                    };
                }
            } else {
                return Some(Transactions(transactions));
            }
        }
    }
}

impl World {
    fn interactive(&self) {
        let mut world = self.clone();
        let stdin = stdin();
        //setting up stdout and going into raw mode
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        //detecting keydown events
        for c in stdin.keys() {
            //clearing the screen and going to top left corner
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .unwrap();

            match c.unwrap() {
                Key::Right => {
                    world = world.apply_instruction(&Instruction::East);
                    println!("{}", world);
                }
                Key::Down => {
                    world = world.apply_instruction(&Instruction::South);
                    println!("{}", world);
                }
                Key::Up => {
                    world = world.apply_instruction(&Instruction::North);
                    println!("{}", world);
                }
                Key::Left => {
                    world = world.apply_instruction(&Instruction::West);
                    println!("{}", world);
                }
                Key::Ctrl('q') => break,
                _ => (),
            }

            stdout.flush().unwrap();
        }
    }
    fn player(&self) -> Point {
        self.tiles
            .iter()
            .filter(|(_, t)| t == &&Tile::Player)
            .next()
            .unwrap()
            .0
            .clone()
    }

    fn apply_instruction(self, instruction: &Instruction) -> Self {
        let result = match instruction.to_transactions(&self) {
            Some(transactions) => self.apply_transactions(&transactions),
            None => self.clone(),
        };
        result
        // match result.has_the_terrible_wall_bug() {
        //     true => self,
        //     false => result,
        // }
    }

    // fn has_the_terrible_wall_bug(&self) -> bool {
    //     self.tiles
    //         .clone()
    //         .into_iter()
    //         .filter(|(p, t)| {
    //             t == &Tile::Box
    //                 && self.tiles.get(&p.transform(&Instruction::West)) == Some(&Tile::Box)
    //         })
    //         .count()
    //         > 0
    // }

    fn apply_transactions(&self, transactions: &Transactions) -> Self {
        transactions
            .0
            .iter()
            .fold(self.clone(), |w, t| w.apply_transaction(t))
    }

    fn apply_transaction(&self, t: &Transaction) -> Self {
        let mut tiles = self.tiles.clone();
        if let Some(tile) = t.tile.clone() {
            tiles.insert(t.p.clone(), tile);
        } else {
            tiles.remove(&t.p);
        }
        Self {
            tiles: tiles,
            width: self.width,
            height: self.height,
        }
    }

    fn answer(self) -> usize {
        self.tiles
            .into_iter()
            .filter(|(_, t)| t == &Tile::Box)
            .map(|(p, _)| p.distance())
            .sum()
    }

    fn apply_instructions(self, instructions: &Instructions) -> Self {
        let result = instructions
            .0
            .iter()
            .fold(self.clone(), |result, instruction| {
                result.apply_instruction(instruction)
            });
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let input = Point { x: 2, y: 2 };
        let result = input.distance();
        let expected = 202;
        assert_eq!(result, expected);
    }
}

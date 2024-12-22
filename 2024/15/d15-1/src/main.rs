use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let (left, right) = s.split_once("\n\n").unwrap();
    let instructions = Instructions::try_from(right).unwrap();
    let world = World::try_from(left).unwrap();
    // println!("{world}");
    world.apply_instructions(&instructions).answer()
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
                let c = match self.tiles.get(&p) {
                    None => '.',
                    Some(Tile::Box) => 'O',
                    Some(Tile::Wall) => '#',
                    Some(Tile::Player) => '@',
                };
                result.push(c);
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
                let p = Point {
                    y: y.try_into().unwrap(),
                    x: x.try_into().unwrap(),
                };
                let tile = match c {
                    '.' => None,
                    '#' => Some(Tile::Wall),
                    'O' => Some(Tile::Box),
                    '@' => Some(Tile::Player),
                    c => panic!("Unknown char: {}", c),
                };
                if let Some(tile) = tile {
                    hm.insert(p, tile);
                }
            }
        }
        Ok(Self {
            tiles: hm.clone(),
            width,
            height,
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
        let mut current_tile = Tile::Player;
        let mut next_point = w.player().transform(self);
        loop {
            match w.tiles.get(&next_point) {
                None => {
                    let t = Transaction {
                        p: next_point,
                        tile: Some(current_tile),
                    };
                    transactions.push(t);
                    return Some(Transactions(transactions));
                }
                Some(Tile::Wall) => return None,
                Some(tile) => {
                    let t = Transaction {
                        p: next_point.clone(),
                        tile: Some(current_tile),
                    };
                    transactions.push(t);
                    next_point = next_point.transform(self);
                    current_tile = tile.clone();
                }
            };
        }
    }
}

impl World {
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
            None => self,
        };
        // println!("{result}");
        result
    }

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

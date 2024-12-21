use std::collections::HashMap;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let (left, right) = s.split_once("\n\n").unwrap();
    let instructions = Instructions::try_from(right).unwrap();
    World::try_from(left)
        .unwrap()
        .apply_instructions(&instructions)
        .answer()
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
        todo!()
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
}

struct Transactions(Vec<Transaction>);

impl Instruction {
    fn to_transactions(&self, w: &World) -> Option<Transactions> {
        let mut transactions = vec![Transaction {
            p: w.player(),
            tile: None,
        }];
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
                Some(Tile::Box) => {
                    let t = Transaction {
                        p: next_point.clone(),
                        tile: Some(current_tile),
                    };
                    transactions.push(t);
                    next_point = next_point.transform(self);
                    current_tile = Tile::Box;
                }
                Some(Tile::Player) => panic!("Found a duplicate player"),
            }
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
        if let Some(transactions) = instruction.to_transactions(&self) {
            self.apply_transactions(&transactions)
        } else {
            self
        }
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
        }
        Self { tiles: tiles }
    }

    fn answer(self) -> usize {
        self.tiles
            .into_iter()
            .filter(|(_, t)| t == &Tile::Box)
            .map(|(p, _)| p.distance())
            .sum()
    }

    fn apply_instructions(self, instructions: &Instructions) -> Self {
        instructions
            .0
            .iter()
            .fold(self.clone(), |result, instruction| {
                result.apply_instruction(instruction)
            })
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

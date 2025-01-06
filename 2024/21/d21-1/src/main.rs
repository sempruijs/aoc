#![allow(warnings)]
use std::{collections::HashMap, thread::Thread};

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let codes = Codes::try_from(s).unwrap();
    let world = World::default();
    codes.answer(world)
}

impl Default for World {
    fn default() -> Self {
        Self {
            base: NumPadRoom::default(),
            layers: vec![
                DPadRoom::default(),
                DPadRoom::default(),
                DPadRoom::default(),
            ],
        }
    }
}

impl Default for NumPadRoom {
    fn default() -> Self {
        let mut hm = HashMap::new();

        // 7 8 9
        hm.insert(NumPadKey::Seven, Point { x: 0, y: 0 });
        hm.insert(NumPadKey::Eight, Point { x: 1, y: 0 });
        hm.insert(NumPadKey::Nine, Point { x: 2, y: 0 });

        // 4 5 6
        hm.insert(NumPadKey::Four, Point { x: 0, y: 1 });
        hm.insert(NumPadKey::Five, Point { x: 1, y: 1 });
        hm.insert(NumPadKey::Six, Point { x: 2, y: 1 });

        // 1 2 3
        hm.insert(NumPadKey::One, Point { x: 0, y: 2 });
        hm.insert(NumPadKey::Two, Point { x: 1, y: 2 });
        hm.insert(NumPadKey::Three, Point { x: 2, y: 2 });

        // - 0 A
        hm.insert(NumPadKey::Zero, Point { x: 1, y: 3 });
        hm.insert(NumPadKey::Press, Point { x: 2, y: 3 });

        let arm = Arm(hm.get(&NumPadKey::Press).unwrap().clone());

        Self { key_pad: hm, arm }
    }
}

impl Default for DPadRoom {
    fn default() -> Self {
        let mut hm = HashMap::new();

        // - ^ A
        hm.insert(Instruction::North, Point { x: 1, y: 0 });
        hm.insert(Instruction::Press, Point { x: 2, y: 0 });

        // < v >
        hm.insert(Instruction::West, Point { x: 0, y: 1 });
        hm.insert(Instruction::South, Point { x: 1, y: 1 });
        hm.insert(Instruction::East, Point { x: 2, y: 1 });

        let arm = Arm(hm.get(&Instruction::Press).unwrap().clone());

        Self { key_pad: hm, arm }
    }
}

impl TryFrom<&str> for Codes {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines().map(|l| Code::try_from(l).unwrap()).collect(),
        ))
    }
}

impl TryFrom<&str> for Code {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.chars()
                .map(|c| match c {
                    '0' => NumPadKey::Zero,
                    '1' => NumPadKey::One,
                    '2' => NumPadKey::Two,
                    '3' => NumPadKey::Three,
                    '4' => NumPadKey::Four,
                    '5' => NumPadKey::Five,
                    '6' => NumPadKey::Six,
                    '7' => NumPadKey::Seven,
                    '8' => NumPadKey::Eight,
                    '9' => NumPadKey::Nine,
                    'A' => NumPadKey::Press,
                    c => panic!("Enknown character {c}"),
                })
                .collect(),
        ))
    }
}

struct Codes(Vec<Code>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    North,
    East,
    South,
    West,
    Press,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Instructions(Vec<Instruction>);

impl From<Point> for Instructions {
    fn from(value: Point) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NumPadKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Press,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Arm(Point);

struct Code(Vec<NumPadKey>);

impl From<&Code> for String {
    fn from(code: &Code) -> Self {
        code.0
            .iter()
            .fold(String::new(), |mut result, instruction| {
                let c = match instruction {
                    &NumPadKey::Zero => '0',
                    &NumPadKey::One => '1',
                    &NumPadKey::Two => '2',
                    &NumPadKey::Three => '3',
                    &NumPadKey::Four => '4',
                    &NumPadKey::Five => '5',
                    &NumPadKey::Six => '6',
                    &NumPadKey::Seven => '7',
                    &NumPadKey::Eight => '8',
                    &NumPadKey::Nine => '9',
                    &NumPadKey::Press => 'A',
                };
                result.push(c);
                result
            })
    }
}

trait GoTo<K> {
    fn go_to(self, key: &K) -> (Instructions, Self);
}

impl DPadRoom {
    fn apply_instructions(&self, instructions: Instructions) -> (Instructions, Self) {
        instructions.0.iter().fold(
            (Instructions::default(), self.clone()),
            |(mut instructions, layer), instruction| {
                let (new_instructions, layer) = layer.go_to(instruction);
                instructions.0.extend(new_instructions.0);
                (instructions, layer)
            },
        )
    }
}

impl GoTo<NumPadKey> for NumPadRoom {
    fn go_to(self, key: &NumPadKey) -> (Instructions, Self) {
        let current = self.arm.0;
        let destination = self
            .key_pad
            .get(key)
            .expect("Could not recieve position for num pad key: {key}");
        let instructions = Instructions::from(destination.min(&current));
        let room = Self {
            key_pad: self.key_pad.clone(),
            arm: Arm(destination.clone()),
        };
        (instructions, room)
    }
}

impl Point {
    fn min(&self, p: &Point) -> Self {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

impl GoTo<Instruction> for DPadRoom {
    fn go_to(self, key: &Instruction) -> (Instructions, Self) {
        let current = self.arm.0;
        let destination = self
            .key_pad
            .get(key)
            .expect("Could not recieve position for num pad key: {key}");
        let instructions = Instructions::from(destination.min(&current));
        let room = Self {
            key_pad: self.key_pad.clone(),
            arm: Arm(destination.clone()),
        };
        (instructions, room)
    }
}

struct NumPadRoom {
    key_pad: HashMap<NumPadKey, Point>,
    arm: Arm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DPadRoom {
    key_pad: HashMap<Instruction, Point>,
    arm: Arm,
}

struct World {
    base: NumPadRoom,
    layers: Vec<DPadRoom>,
}

impl Code {
    fn instructions(&self, world: World) -> (Instructions, World) {
        // get base instructions
        // these instructions are executed on inside the numpad room.
        let (base_instructions, base) = self.0.iter().fold(
            (Instructions::default(), world.base),
            |(mut instructions, base), key| {
                let (new_instructions, base) = base.go_to(key);
                instructions.0.extend(new_instructions.0);
                (instructions, base)
            },
        );
        let (instructions, layers) = world.layers.iter().fold(
            (base_instructions, Vec::new()),
            |(instructions, mut layers), layer| {
                let (instructions, layer) = layer.apply_instructions(instructions);
                layers.push(layer);
                (instructions, layers)
            },
        );
        let world = World { base, layers };
        (instructions, world)
    }

    fn score(&self, world: World) -> (usize, World) {
        let (instructions, world) = self.instructions(world);
        let score = instructions.0.len() * self.numeric_value();
        (score, world)
    }

    fn numeric_value(&self) -> usize {
        String::from(self)
            .chars()
            .filter(|c| c != &'A')
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }
}

impl Codes {
    fn answer(&self, world: World) -> usize {
        self.0
            .iter()
            .fold((0, world), |(_, world), code| code.score(world))
            .0
    }
}

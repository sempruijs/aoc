#![allow(warnings)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let codes = Codes::try_from(s).unwrap();
    let rooms = Rooms::default();
    codes.answer(rooms)
}

impl Default for Rooms {
    fn default() -> Self {
        todo!()
    }
}

impl TryFrom<&str> for Codes {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

struct Codes(Vec<Code>);

struct Point {
    x: u8,
    y: u8,
}

enum Instruction {
    North,
    East,
    South,
    West,
    Press,
}

struct Instructions(Vec<Instruction>);

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
enum DPadKey {
    North,
    East,
    South,
    West,
    Press,
}

enum Pad {
    Numbers(HashMap<Point, NumPadKey>),
    Arrows(HashMap<Point, DPadKey>),
}

struct Arm(Point);

struct Code(Vec<NumPadKey>);

trait GoTo<K> {
    fn go_to(self, key: &K) -> (Instructions, Self);
}

impl GoTo<NumPadKey> for NumPadRoom {
    fn go_to(self, key: &NumPadKey) -> (Instructions, Self) {
        todo!()
    }
}

impl GoTo<DPadKey> for DPadRoom {
    fn go_to(self, key: &DPadKey) -> (Instructions, Self) {
        todo!()
    }
}

struct NumPadRoom {
    key_pad: HashMap<NumPadKey, Point>,
    arm: Arm,
}

struct DPadRoom {
    key_pad: HashMap<NumPadKey, Point>,
    arm: Arm,
}

struct Rooms {
    base: NumPadRoom,
    layers: Vec<DPadRoom>,
}

impl Code {
    fn instructions(&self, rooms: Rooms) -> (Instructions, Rooms) {
        // get base instructions
        // these instructions are executed on inside the numpad room.
        let (base_instructions, base) =
            self.0
                .iter()
                .fold((Vec::new(), rooms.base), |(mut instructions, base), key| {
                    let (new_instructions, base) = base.go_to(key);
                    instructions.extend(new_instructions.0);
                    (instructions, base)
                });
        rooms.layers.into_iter().fold((base_instructions, base), |(mut instructions, layer)| {
            // fold into instructions for specefic layer
            let (instructions, room) =
            instructions
                // .iter()
                // .fold((Vec::new(), layer), |(mut instructions, layer), key| {
                //     let (instructions, layer) = layer.go_to(key);
                //     instructions.extend(new_instructions.0);
                //     (instructions, layer)
                // });
        })
        todo!()
    }

    fn score(&self, rooms: Rooms) -> (usize, Rooms) {
        let (instructions, rooms) = self.instructions(rooms);
        let score = instructions.0.len() * self.numeric_value();
        (score, rooms)
    }

    fn numeric_value(&self) -> usize {
        todo!()
    }
}

impl Codes {
    fn answer(&self, rooms: Rooms) -> usize {
        self.0
            .iter()
            .fold((0, rooms), |(_, rooms), code| code.score(rooms))
            .0
    }
}

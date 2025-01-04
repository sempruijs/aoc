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

struct Room {
    keys: Pad,
    arm: Arm,
}

struct Code(Vec<NumPadKey>);

struct Rooms(Vec<Room>);

impl Code {
    fn instructions(&self, rooms: Rooms) -> (Instructions, Rooms) {
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

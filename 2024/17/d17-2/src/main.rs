// #![allow(warnings)]
use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let w = World::try_from(input).unwrap();
    let correct = Output::from(w.program.clone());
    let answer = w.answer(correct.0.len() - 1, 0, &correct).unwrap();
    println!("The answer is: {}", answer);
    // for i in 0..50000 {
    //     let w = w.clone().init_with(i).execute();
    //     println!("{:b} | {} | {}", i, i, w.output);
    // }
    // panic!("bla");
}

impl From<Program> for Output {
    fn from(program: Program) -> Self {
        Self(
            program
                .0
                .iter()
                .fold(Vec::new(), |mut acc: Vec<i64>, instruction| {
                    let (a, b) = match instruction {
                        Instruction::Adv(c) => (0, c.0),
                        Instruction::Bxl(l) => (1, l.0),
                        Instruction::Bst(c) => (2, c.0),
                        Instruction::Jnz(p) => (3, p.0),
                        Instruction::Bxc(l) => (4, l.0),
                        Instruction::Out(c) => (5, c.0),
                        Instruction::Bdv(c) => (6, c.0),
                        Instruction::Cdv(c) => (7, c.0),
                    };
                    acc.push(a.into());
                    acc.push(b.into());
                    acc
                }),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Output(Vec<i64>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.0.iter().fold(String::new(), |mut acc, n| {
            acc.push_str(&format!(",{}", n));
            acc
        });
        let result = result.split_once(",").unwrap().1;
        write!(f, "{}", result)
    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Pointer(u8);

impl Pointer {
    fn next(&self) -> Self {
        Self(self.0 + 2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Combo(u8);
#[derive(Debug, Clone, PartialEq, Hash)]
struct Literal(u8);

#[derive(Debug, Clone, PartialEq, Hash)]
struct World {
    pointer: Pointer,
    registers: Registers,
    output: Output,
    program: Program,
}

#[derive(Debug, Clone, PartialEq, Hash)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Pointer),
    Bxc(Literal),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Combo {
    fn calc(&self, w: &World) -> i64 {
        let x: i64 = self.0.into();
        if x >= 0 && x <= 3 {
            return x;
        }
        match x {
            4 => w.registers.a,
            5 => w.registers.b,
            6 => w.registers.c,
            7 => panic!("Found a 7 which is not allowed"),
            x => panic!("Unknown number {}", x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Program(Vec<Instruction>);

impl World {
    fn answer(&self, digid: usize, minimum: i64, correct: &Output) -> Option<i64> {
        if *correct == self.clone().init_with(minimum).execute().output {
            return Some(minimum);
        }

        for x in 0..8 {
            let correct_value = correct.0[digid];
            let n = (x << 3 * (digid)) + minimum;
            let output = self.clone().init_with(n).execute().output;
            println!("output: {output}");
            let my_value = output.0.get(digid as usize);

            println!("{:b} | {}", n, n);
            println!("digid: {digid}");
            // println!("correct: {correct_value} | value: {my_value}");

            if my_value == Some(&correct_value) {
                if let Some(x) = self.answer(digid - 1, n, correct) {
                    return Some(x);
                }
            }
        }
        None
    }

    fn init_with(self, i: i64) -> Self {
        Self {
            program: self.program,
            output: self.output,
            pointer: self.pointer,
            registers: Registers {
                a: i,
                b: self.registers.b,
                c: self.registers.c,
            },
        }
    }

    fn current_instruction(&self) -> Option<Instruction> {
        if self.pointer.0.rem_euclid(2) == 1 {
            println!("Returned none because the pointer did not reference to an instruction");
            return None;
        }
        let i: usize = (self.pointer.0 >> 1) as usize;
        self.program
            .0
            .get(i)
            .map(|instruction: &Instruction| instruction.clone())
    }

    fn execute(self) -> Self {
        match self.current_instruction() {
            Some(instruction) => self.apply_instruction(instruction).execute(),
            None => self,
        }
    }

    fn apply_instruction(mut self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Adv(c) => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                let x = self.registers.a / (2 << (c.calc(&self)) - 1);
                self.registers.a = x;
                self.pointer = self.pointer.next();
            }
            Instruction::Bxl(l) => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                let x = self.registers.b ^ l.0 as i64;
                self.registers.b = x;
                self.pointer = self.pointer.next();
            }
            Instruction::Bst(c) => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                let x = c.calc(&self).rem_euclid(8);
                self.registers.b = x;
                self.pointer = self.pointer.next();
            }
            Instruction::Jnz(p) => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if self.registers.a == 0 {
                    self.pointer = self.pointer.next();
                } else {
                    self.pointer = p;
                }
            }
            Instruction::Bxc(_) => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                let x = self.registers.b ^ self.registers.c;
                self.registers.b = x;
                self.pointer = self.pointer.next();
            }
            Instruction::Out(c) => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let x = c.calc(&self).rem_euclid(8);
                self.output.0.push(x);
                self.pointer = self.pointer.next();
            }
            Instruction::Bdv(c) => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let x = self.registers.a / 2_i64.pow(c.calc(&self).try_into().unwrap());
                self.registers.b = x;
                self.pointer = self.pointer.next();
            }
            Instruction::Cdv(c) => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let x = self.registers.a / 2_i64.pow(c.calc(&self).try_into().unwrap());
                self.registers.c = x;
                self.pointer = self.pointer.next();
            }
        };
        self
    }
}

impl TryFrom<&str> for Program {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let numbers: Vec<u8> = s
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Ok(Self(
            numbers[..]
                .chunks(2)
                .map(|slice| {
                    let a = slice[0];
                    let b = slice[1];
                    match a {
                        0 => Instruction::Adv(Combo(b)),
                        1 => Instruction::Bxl(Literal(b)),
                        2 => Instruction::Bst(Combo(b)),
                        3 => Instruction::Jnz(Pointer(b)),
                        4 => Instruction::Bxc(Literal(b)),
                        5 => Instruction::Out(Combo(b)),
                        6 => Instruction::Bdv(Combo(b)),
                        7 => Instruction::Cdv(Combo(b)),
                        x => panic!("Found invalid instruction opcode {}", x),
                    }
                })
                .collect(),
        ))
    }
}

impl TryFrom<&str> for World {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (left, right) = s.split_once("\n\n").unwrap();
        let registers: Vec<i64> = left
            .lines()
            .map(|l| l.split_once(": ").unwrap().1.parse::<i64>().unwrap())
            .collect();
        let registers = Registers {
            a: registers[0],
            b: registers[1],
            c: registers[2],
        };
        let program = Program::try_from(right.split_once(": ").unwrap().1)?;
        let pointer = Pointer(0);
        let output = Output(Vec::new());
        Ok(Self {
            pointer,
            registers,
            program,
            output,
        })
    }
}

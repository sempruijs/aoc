use num_bigint::BigInt;
use num_bigint::ToBigInt;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

struct Machines(Vec<Machine>);

impl TryFrom<&str> for Machine {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let points: Vec<Point> = s
            .lines()
            .enumerate()
            .map(|(i, l)| match i == 2 {
                true => {
                    let (left, right) = l.split_once(": X=").unwrap().1.split_once(", Y=").unwrap();
                    let scaler = 1_000_0000000000_i64;
                    Point {
                        x: scaler.to_bigint().unwrap()
                            + left.to_string().parse::<BigInt>().unwrap(),
                        y: scaler.to_bigint().unwrap()
                            + right.to_string().parse::<BigInt>().unwrap(),
                    }
                }
                false => {
                    let (left, right) = l.split_once(": X+").unwrap().1.split_once(", Y+").unwrap();
                    Point {
                        x: left.to_string().parse().unwrap(),
                        y: right.to_string().parse().unwrap(),
                    }
                }
            })
            .collect();
        Ok(Self {
            price: points[2].clone(),
            button_a: Button {
                price: 3,
                transform: points[0].clone(),
            },
            button_b: Button {
                price: 1,
                transform: points[1].clone(),
            },
        })
    }
}

impl TryFrom<&str> for Machines {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.split("\n\n")
                .map(|a| Machine::try_from(a).unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Machine {
    price: Point,
    button_a: Button,
    button_b: Button,
}

#[derive(Debug, Clone)]
struct Button {
    price: u8,
    transform: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

fn input_to_answer(s: &str) -> BigInt {
    let machines = Machines::try_from(s).unwrap();
    machines.tokens()
}

impl Button {
    // fn press(self, repeat: usize, p: Point) -> Point {
    //     if repeat == 0 {
    //         return p;
    //     }
    //     let p = Point {
    //         x: p.x + self.transform.x,
    //         y: p.y + self.transform.y,
    //     };
    //     self.press(repeat - 1, p)
    // }
}

impl Machine {
    fn tokens(self) -> Option<BigInt> {
        let v1 = self.button_a.transform;
        let v2 = self.button_b.transform;
        let p = self.price;
        let dot: BigInt = (v1.x.clone() * v2.y.clone()) - (v1.y.clone() * v2.x.clone());
        if dot == BigInt::ZERO && !(&p.x / &v1.x == &p.y / &v1.y || &p.x / &v2.x == &p.y / &v2.y) {
            return None;
        }
        let s1: BigInt = (v2.y.clone() * p.x.clone() - v2.x.clone() * p.y.clone()) / dot.clone();
        let s2: BigInt = (-v1.y.clone() * p.x.clone() + v1.x.clone() * p.y.clone()) / dot.clone();
        if s1.clone() * v1.x.clone() + s2.clone() * v2.x.clone() != p.x.clone()
            || s1.clone() * v1.y.clone() + s2.clone() * v2.y.clone() != p.y.clone()
        {
            return None;
        }
        let result: BigInt = (3_i32.to_bigint().unwrap() * s1 + 1_i32.to_bigint().unwrap() * s2)
            .try_into()
            .unwrap();
        Some(result)
    }
}

impl Machines {
    fn tokens(self) -> BigInt {
        self.0.into_iter().filter_map(Machine::tokens).sum()
    }
}

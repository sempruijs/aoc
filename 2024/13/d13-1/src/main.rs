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
            .map(|(i, l)| {
                let (left, right) = if i == 2 {
                    l.split_once(": X=").unwrap().1.split_once(", Y=").unwrap()
                } else {
                    l.split_once(": X+").unwrap().1.split_once(", Y+").unwrap()
                };
                Point {
                    x: left.to_string().parse().unwrap(),
                    y: right.to_string().parse().unwrap(),
                }
            })
            .collect();
        Ok(Self {
            price: points[2].clone(),
            button_a: Button {
                transform: points[0].clone(),
            },
            button_b: Button {
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
    transform: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn input_to_answer(s: &str) -> usize {
    let machines = Machines::try_from(s).unwrap();
    machines.tokens()
}

impl Button {
    fn press(self, repeat: usize, p: Point) -> Point {
        if repeat == 0 {
            return p;
        }
        let p = Point {
            x: p.x + self.transform.x,
            y: p.y + self.transform.y,
        };
        self.press(repeat - 1, p)
    }
}

impl Machine {
    fn tokens(self) -> Option<usize> {
        let mut result = None;
        for i in 0..100 {
            for j in 0..100 {
                let a = self.clone().button_a.press(i, Point::origin());
                let b = self.clone().button_b.press(j, a);
                if b == self.price {
                    let tokens = (i * 3) + j;
                    if let Some(x) = result {
                        if tokens < x {
                            result = Some(tokens);
                        }
                    } else {
                        result = Some(tokens);
                    }
                }
            }
        }
        result
    }
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Machines {
    fn tokens(self) -> usize {
        self.0.into_iter().filter_map(Machine::tokens).sum()
    }
}

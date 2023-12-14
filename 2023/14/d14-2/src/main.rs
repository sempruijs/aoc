use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let height: u32 = s.lines().count() as u32;
    let mut world = World::from_str(s);
    let (start_index, start, cycle_len) = world.start_and_repetition(&(height as i32));
    let amount_of_cycles = (1_000_000_000 - start_index) % cycle_len;
    for _ in 0..amount_of_cycles {
        world = world.cycle(&(height as i32));
    }
    world.weight(&height)
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = HashMap::new();

        self.0.iter().for_each(|r| {
            let p = r.p.clone();
            let kind = r.kind.clone();
            m.insert(p, kind);
        });

        let mut result = String::new();

        let width = self.as_columns().len();
        let height = 10;
        for y in 1..=height {
            for x in 1..=width {
                let p = Point::from(x as i32, y as i32);
                if let Some(k) = m.get(&p) {
                    match k {
                        RockKind::Solid => result.push('#'),
                        RockKind::Round => result.push('O'),
                    }
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum RockKind {
    Solid,
    Round,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rock {
    p: Point,
    kind: RockKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct World(Vec<Rock>);

impl Point {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn transpose(&self, height: &i32) -> Self {
        let x: i32 = height - self.y + 1;
        let y: i32 = self.x;
        Point::from(x, y)
    }
}

impl World {
    fn from_str(s: &str) -> Self {
        let mut result: Vec<Rock> = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' || c == 'O' {
                    let p = Point::from((x + 1) as i32, (y + 1) as i32);
                    let kind = match c {
                        '#' => RockKind::Solid,
                        'O' => RockKind::Round,
                        _ => panic!("could not parse character"),
                    };
                    let r = Rock { p, kind };
                    result.push(r);
                }
            }
        }
        Self(result)
    }

    fn as_columns(&self) -> Vec<Vec<Rock>> {
        let mut m: HashMap<i32, Vec<Rock>> = HashMap::new();

        for rock in &self.0 {
            let r = rock.clone();
            let column_index = rock.p.x;
            m.entry(column_index).or_insert(Vec::new()).push(r);
        }
        let mut result: Vec<Vec<Rock>> = m.values().cloned().collect();
        result.sort_by_key(|v| v[0].p.x);
        result
    }

    fn roll_north(&self) -> Self {
        let mut result: Vec<Vec<Rock>> = Vec::new();
        for column in self.as_columns() {
            let c = World::roll_column(column);
            result.push(c)
        }
        Self(result.into_iter().flatten().collect::<Vec<Rock>>())
    }

    fn roll_column(column: Vec<Rock>) -> Vec<Rock> {
        let mut column: Vec<Rock> = column;
        column.sort_by_key(|r| r.p.y);
        let mut wall = Point::from(column[0].p.x, 0);
        for rock in column.iter_mut() {
            match rock.kind {
                RockKind::Solid => {
                    wall = rock.p.clone();
                }
                RockKind::Round => {
                    let p = Point::from(wall.x, wall.y + 1);
                    rock.p = p.clone();
                    wall = p.clone();
                }
            };
        }
        column
    }

    fn weight(&self, height: &u32) -> u32 {
        self.0
            .iter()
            .filter(|r| r.kind == RockKind::Round)
            .map(|r| (height + 1) - r.p.y as u32)
            .sum()
    }

    fn cycle(&self, height: &i32) -> Self {
        let mut world = self.clone();
        for _ in 0..4 {
            world = world.roll_north();
            world = world.transpose(&height);
        }
        world
    }

    fn transpose(&self, height: &i32) -> Self {
        Self(
            self.0
                .iter()
                .map(|r| {
                    let p = r.p.transpose(&height);
                    Rock {
                        p,
                        kind: r.kind.clone(),
                    }
                })
                .collect::<Vec<Rock>>(),
        )
    }

    fn start_and_repetition(&self, height: &i32) -> (u32, World, u32) {
        let mut m: HashMap<World, u32> = HashMap::new();
        let mut index = 0;
        let mut world = self.clone();
        while index < 10000 {
            if let Some(start_index) = m.get(&world) {
                let cycle_len = index - start_index;
                let start = world.clone();
                return (*start_index, start, cycle_len);
            } else {
                m.insert(world.clone(), index);
                world = world.cycle(height);
                index += 1;
            }
        }
        panic!("could not find repetition");
    }
}

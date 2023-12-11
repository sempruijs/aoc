use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let maze = Maze::from_str(s);
    let steps = Steps::from_str(s);
    // println!("{}", steps.0.len());
    let mut key = "TSA";
    let mut step_count = 0;

    // rust is not lazy, that is why I am lazy.
    for i in 0..100000 {
        // if key == "ZZZ" {
        //     return step_count;
        // }
        println!("{}", key);

        let step = &steps.0[i % steps.0.len()];
        key = match step {
            Step::Left => maze.0.get(key).unwrap().0,
            Step::Right => maze.0.get(key).unwrap().1,
        };
        step_count += 1;
    }

    0
}

#[derive(Debug)]
struct Maze<'a>(HashMap<&'a str, (&'a str, &'a str)>);

#[derive(Debug)]
enum Step {
    Left,
    Right,
}

struct Steps(Vec<Step>);

impl Steps {
    fn from_str(s: &str) -> Self {
        let mut result = Vec::new();
        let steps_line = s.split_once("\n\n").unwrap().0;

        for c in steps_line.chars() {
            match c {
                'L' => result.push(Step::Left),
                'R' => result.push(Step::Right),
                _ => panic!("could not parse char to step"),
            }
        }
        Self(result)
    }
}

impl<'a> Maze<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut m = HashMap::new();
        for line in s.split_once("\n\n").unwrap().1.lines() {
            let (id, pair) = line.split_once(" = (").unwrap();
            let (left, right) = pair.split_once(", ").unwrap();
            let right = &right[..3];
            m.insert(id, (left, right));
        }
        Self(m)
    }
}

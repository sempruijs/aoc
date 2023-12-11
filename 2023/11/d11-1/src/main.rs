fn main() {
    let input = "../../example.txt";
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let galaxy = Galaxy::from_str(s).add_lightyear_distance();
    galaxy.sum_distances()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

struct Galaxy(Vec<Point>);

impl Galaxy {
    fn from_str(s: &str) -> Self {
        Self(Vec::new())
    }

    fn add_lightyear_distance(&self) -> Self {
        Self(Vec::new())
    }

    fn sum_distances(&self) -> u32 {
        0
    }
}

fn distance(p1: &Point, p2: &Point) -> u32 {
    0
}

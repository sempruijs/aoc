struct Matrix(Vec<Vec<char>>);
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", input);
}

impl Matrix {
    pub fn from(s: &str) -> Self {
        Matrix(
            s.lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        )
    }

fn bind_points(&self) -> Vec<Point> {
    //     for y in self.0 {
    //     }        
    // }
}

fn char_can_bind(c: &char) -> bool {
    (!c.is_digit(10)) && c != &'.'
}

use crate::intcode::*;

mod intcode;

fn main() {
    let result = intcode(12, 02);
    println!("{}", result);
}

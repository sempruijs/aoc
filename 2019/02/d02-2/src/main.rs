use crate::intcode::*;
use std::process;

mod intcode;

fn main() {
    for a in 0..99 {
        for b in 0..99 {
            if intcode(a, b) == 19690720 {
                let result = 100 * a + b;
                println!("--- result found ---\n\n {}", result);
                process::exit(0);
            }
        }
    }
    println!("no result found");
}

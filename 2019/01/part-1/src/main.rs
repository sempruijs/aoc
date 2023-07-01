use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn calc_fual(m: u32) -> u32 {
    (m as f32 / 3 as f32).floor() as u32 - 2
}

fn line_to_num(l: String) -> u32 {
    l.trim().parse().expect("failed to read line")
}

fn main() {
    let lines = read_lines("input.txt");

    // let nums = map line_to_num lines
    // let result = sum map calc_fual nums
    let mut result: u32 = 0;

    for line in lines {
        let num = line_to_num(line);
        let fual = calc_fual(num);

        result += fual;
    }

    println!("{}", result);
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn test_fual() {
        assert_eq!(calc_fual(12), 2);
        assert_eq!(calc_fual(14), 2);
        assert_eq!(calc_fual(1969), 654);
    }
}

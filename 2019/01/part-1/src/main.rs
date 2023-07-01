use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let lines = read_lines("input.txt");
    println!("{}", lines[0]);
}

#[cfg(test)]
mod Test {
    #[test]
    fn bla() {
        assert!(true);
    }
}

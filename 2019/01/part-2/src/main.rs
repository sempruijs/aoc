use std::include_str;

fn calc_fual(m: i32) -> i32 {
    (m as f32 / 3 as f32).floor() as i32 - 2
}

fn mass_to_fual(m: i32) -> u32 {
    let mut fuals: Vec<i32> = Vec::new();
    let mut f = calc_fual(m);

    while f > 0 {
        fuals.push(f);
        f = calc_fual(f);
    }

    let mut result: u32 = 0;
    for fual in fuals {
        result += fual as u32;
    }

    result
}

fn line_to_num(l: String) -> i32 {
    l.trim().parse().expect("failed to read line")
}

fn main() {
    let input = include_str!("../../input.txt");
    let lines: Vec<String> = input.lines().map(String::from).collect();

    // let nums = map line_to_num lines
    // let result = sum map mass_to_fual nums
    let mut result: u32 = 0;

    for line in lines {
        let num = line_to_num(line);
        let fual = mass_to_fual(num) as u32;

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

    #[test]
    fn test_mass_to_fual() {
        assert_eq!(mass_to_fual(14), 2);
        assert_eq!(mass_to_fual(100756), 50346);
        assert_eq!(mass_to_fual(1969), 966);
    }
}

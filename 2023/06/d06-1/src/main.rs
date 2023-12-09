use num_bigint::*;

fn main() {
    let answer = answer();
    println!("{}", answer);
}

fn answer() -> BigUint {
    let g1 = Game::from(40, 215).winning_options().to_biguint().unwrap();
    let g2 = Game::from(70, 1051).winning_options().to_biguint().unwrap();
    let g3 = Game::from(98, 2147).winning_options().to_biguint().unwrap();
    let g4 = Game::from(79, 1005).winning_options().to_biguint().unwrap();

    (g1 * g2 * g3 * g4).into()
}

struct Game {
    time: u32,
    record: u32,
}

impl Game {
    fn winning_options(&self) -> u32 {
        (0..=self.time)
            .filter(|t| (t * (self.time - t) > self.record))
            .collect::<Vec<u32>>()
            .len() as u32
    }

    fn from(time: u32, record: u32) -> Self {
        Self { time, record }
    }
}

// 7 0 0
// 6 1 6
// 5 2 10
// 4 3 9
// 3 4 9
// 2 5 10
// 1 6 6

// 15 0 0
// 14 1 14
// 13 2 26
// 12 3 26
// 11 4 44
// 10 5 50
// 9 6 54
// 8 7 56
// 7 8 56

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_of_winning() {
        let g1 = Game::from(7, 9).winning_options();
        let g2 = Game::from(15, 40).winning_options();
        let g3 = Game::from(30, 200).winning_options();

        assert_eq!(g1, 4);
        assert_eq!(g2, 8);
        assert_eq!(g3, 9);

        let result = g1 * g2 * g3;
        assert_eq!(result, 288)
    }
}

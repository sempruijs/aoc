use num_bigint::*;

fn main() {
    let answer = answer();
    println!("{}", answer);
}

fn answer() -> BigUint {
    Game::from(40709879, 215105121471005)
        .winning_options()
        .to_biguint()
        .unwrap()
}

struct Game {
    time: u128,
    record: u128,
}

impl Game {
    fn winning_options(&self) -> u128 {
        (0..=self.time)
            .filter(|t| (t * (self.time - t) > self.record))
            .collect::<Vec<u128>>()
            .len() as u128
    }

    fn from(time: u128, record: u128) -> Self {
        Self { time, record }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_of_winning() {
        let g1 = Game::from(71530, 940200).winning_options();

        assert_eq!(g1, 71503);
    }
}

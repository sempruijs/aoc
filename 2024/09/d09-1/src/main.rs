fn main() {
    let input = include_str!("../../input.txt");
    let answer = input_to_answer(input);
    println!("answer is: {}", answer);
}

fn input_to_answer(s: &str) -> usize {
    let mut disk = Disk::try_from(s).unwrap();
    disk.cleanup();
    disk.checksum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Disk(Vec<Option<u128>>);

impl TryFrom<&str> for Disk {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines()
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .fold(Vec::new(), |mut v, (i, x)| {
                    for _ in 0..x {
                        match i % 2 == 0 {
                            true => v.push(Some((i / 2).try_into().unwrap())),
                            false => v.push(None),
                        }
                    }
                    v
                }),
        ))
    }
}

impl Disk {
    fn cleanup(&mut self) {
        if !self.is_clean() {
            let (i, last) = self.last();
            let first_none = self
                .0
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_none())
                .next()
                .unwrap()
                .0;
            self.0[first_none] = Some(last);
            self.0[i] = None;
            self.cleanup()
        }
    }

    fn last(&self) -> (usize, u128) {
        let (i, x) = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .last()
            .unwrap();
        (i, x.unwrap().clone())
    }

    fn is_clean(&self) -> bool {
        self.0.iter().is_sorted_by(|a, b| {
            (a.is_some() && b.is_some())
                || (a.is_some() && b.is_none())
                || (a.is_none() && b.is_none())
        })
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .filter(|x| x.is_some())
            .enumerate()
            .fold(0, |sum, (i, x)| sum + ((x.unwrap()) as usize * i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "2333133121414131402";
        let expected = Disk(vec![
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ]);
        let result = Disk::try_from(input).unwrap();
        assert_eq!(result, expected);

        let input = "33";
        let expected = Disk(vec![Some(0), Some(0), Some(0), None, None, None]);
        let result = Disk::try_from(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_checksum() {
        let input = Disk(vec![Some(0), Some(0), Some(0), None, None, None]);
        let result = input.checksum();
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_cleanup() {
        let mut result = Disk(vec![Some(0), Some(0), Some(0), None, None, None]);
        result.cleanup();
        let expected = Disk(vec![Some(0), Some(0), Some(0), None, None, None]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_clean() {
        let input = Disk(vec![Some(0), Some(0), Some(0), None, None, None]);
        let result = input.is_clean();
        assert!(result);
    }
}

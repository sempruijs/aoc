use std::fmt::Display;

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
struct Disk(Vec<Space>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Space {
    x: Option<u128>,
    len: usize,
}

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
                    let space = Space {
                        x: match i % 2 == 0 {
                            true => Some((i / 2).try_into().unwrap()),
                            false => None,
                        },
                        len: x as usize,
                    };
                    v.push(space);
                    v
                }),
        ))
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.0
            .iter()
            .for_each(|space| s.push_str(&format!("{}", space)));
        write!(f, "{}", s)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        (0..self.len).for_each(|_| match self.x {
            Some(x) => s.push_str(&format!("{}", x)),
            None => s.push('.'),
        });
        write!(f, "{}", s)
    }
}

impl Space {
    fn insert(&self, space: &Space) -> Option<(Self, Self)> {
        match self.x.is_none() && self.len >= space.len && space.x.is_some() {
            true => {
                let s2 = Self {
                    x: None,
                    len: self.len - space.len,
                };
                Some((space.clone(), s2))
            }
            false => None,
        }
    }

    fn make_empty(&self) -> Self {
        Self {
            x: None,
            len: self.len,
        }
    }
}

impl Disk {
    fn flatten(&self) -> Vec<Option<u128>> {
        self.0.iter().fold(Vec::new(), |mut v, space| {
            (0..space.len).for_each(|_| v.push(space.x));
            v
        })
    }
    fn cleanup(&mut self) -> Self {
        for (i, space) in self.0.iter().enumerate().rev() {
            for (j, space2) in self.0.iter().enumerate() {
                if let Some((s1, s2)) = space2.insert(space) {
                    if j < i {
                        // let mut v = self.0.clone();
                        self.0[i] = space.make_empty();
                        self.0[j] = s1;
                        self.0.insert(j + 1, s2);
                        return self.cleanup();
                    }
                }
            }
        }
        self.clone()
    }

    fn checksum(&self) -> usize {
        self.flatten()
            .iter()
            .enumerate()
            .fold(0, |sum, (i, x)| match x {
                Some(y) => sum + (*y as usize * i),
                None => sum,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let s1 = Space { x: None, len: 4 };
        let s2 = Space { x: Some(4), len: 2 };
        let expected_s1 = Space { x: Some(4), len: 2 };
        let expected_s2 = Space { x: None, len: 2 };
        let (result_s1, result_s2) = s1.insert(&s2).unwrap();
        assert_eq!(result_s1, expected_s1);
        assert_eq!(result_s2, expected_s2);
    }
}

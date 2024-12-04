use std::fmt::Display;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = World::from(input).xmas_count();
    println!("{}", answer);
}

#[derive(Debug, Clone)]
struct World(Vec<Vec<char>>);

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = &self.0;
        let result = v.iter().fold(String::from(""), |mut result, current| {
            let line = current.iter().collect::<String>();
            result.push_str(&line);
            result.push_str("\n");
            result
        });
        write!(f, "{}", result)
    }
}

impl From<&str> for World {
    fn from(s: &str) -> Self {
        Self(
            s.lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        )
    }
}

impl From<&World> for String {
    fn from(w: &World) -> Self {
        w.clone().0.into_iter().flatten().collect()
    }
}

impl World {
    fn xmas_count(&self) -> usize {
        let w = self.clone();
        vec![
            w.clone(),
            w.clone().transpose(),
            w.clone().transpose_diagnoal(),
            w.clone().reverse().transpose_diagnoal(),
        ]
        .iter()
        .fold(0, |sum, current| sum + current.horizontal_count())
    }

    fn horizontal_count(&self) -> usize {
        self.0.iter().fold(0, |sum, current| {
            let x = current.iter().collect::<String>().matches("XMAS").count();
            let y = current.iter().collect::<String>().matches("SAMX").count();
            sum + x + y
        })
    }

    fn reverse(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|v| v.clone().into_iter().rev().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        )
    }

    fn transpose_diagnoal(&self) -> Self {
        let pairs = |i| {
            (0..i)
                .map(|j| (j, (i - j - 1)))
                .collect::<Vec<(usize, usize)>>()
        };
        let diagonal_count = self.0.len() * 3;
        Self(
            (0..(diagonal_count  - 1))
               .map(|i| {
                    pairs(i)
                        .into_iter()
                        .map(|(x, y)| match self.0.get(x) {
                            Some(colum) => colum.get(y),
                            None => None,
                        })
                        .filter(|p| p != &None)
                        .map(|p| p.unwrap().clone())
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
        )
    }

    fn transpose(&self) -> Self {
        World(transpose2(self.0.clone()))
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let v = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let expected = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]];
        let result = transpose2(v);
        assert_eq!(expected, result);
    }
}

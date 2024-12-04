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
    fn has_x(&self, x: usize, y: usize) -> bool {
        if x < 1 || y < 1 || x > self.0.len() - 2 || y > self.0.len() - 2 {
            return false;
        }
        let current = self.get_char(x, y);
        let top_left = self.get_char(x - 1, y - 1);
        let top_right = self.get_char(x + 1, y - 1);
        let bottom_left = self.get_char(x - 1, y + 1);
        let bottom_right = self.get_char(x + 1, y + 1);
        if let (Some(c), Some(tl), Some(tr), Some(bl), Some(br)) =
            (current, top_left, top_right, bottom_left, bottom_right)
        {
            let valid_chars = vec![tl, tr, bl, br]
                .iter()
                .filter(|x| x == &&'M' || x == &&'S')
                .count()
                == 4;
            let is_cross = c == 'A' && tl != br && tr != bl && valid_chars;
            return is_cross && valid_chars;
        }
        false
    }

    fn get_char(&self, x: usize, y: usize) -> Option<char> {
        if let Some(r) = self.0.get(x) {
            if let Some(c) = r.get(y) {
                return Some(*c);
            }
        }
        None
    }
    fn xmas_count(&self) -> usize {
        self.clone()
            .0
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(y, _)| self.has_x(x, *y))
                    .count()
            })
            .sum()
    }
}

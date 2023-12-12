fn main() {
    let input = include_str!("../../example.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let records: Vec<Record> = s.lines().map(|line| Record::from_line(line)).collect();
    // records
    //     .iter()
    //     .map(|r| r.amount_of_possibilaties())
    //     .sum::<u32>()
    let bla = records
        .iter()
        .map(|r| r.amount_of_possibilaties())
        .collect::<Vec<u32>>();
    dbg!(bla);
    0
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Spring,
    Unknown,
    None,
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    cells: Vec<Cell>,
    group_sizes: Vec<u32>,
}

impl Record {
    fn from_line(s: &str) -> Self {
        let (cells, group_sizes) = s.split_once(" ").unwrap();
        let cells = cells
            .chars()
            .map(|c| match c {
                '.' => Cell::None,
                '#' => Cell::Spring,
                '?' => Cell::Unknown,
                _ => panic!("could not parse char"),
            })
            .collect::<Vec<Cell>>();
        let group_sizes: Vec<u32> = group_sizes
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        Self { cells, group_sizes }
    }

    // I do not know how I can write this in a pure way. If someone knows how I can fold this please let me know
    fn is_valid(&self) -> bool {
        let mut previous = Cell::None;
        let mut group_len = 0;
        let mut result = Vec::new();

        for (i, cell) in self.cells.iter().enumerate() {
            if cell == &Cell::Spring {
                group_len += 1;
                if i == &self.cells.len() - 1 {
                    result.push(group_len);
                }
            }
            if cell != &Cell::Spring && previous == Cell::Spring {
                result.push(group_len);
                group_len = 0;
            }
            previous = *cell;
        }
        result == self.group_sizes
    }

    fn possibilities(&self) -> Vec<Self> {
        let unknown_amount: u32 = self
            .cells
            .iter()
            .filter(|c| c == &&Cell::Unknown)
            .count()
            .try_into()
            .unwrap();

        let amount_of_combinations = (2 as u32).pow(unknown_amount);

        println!("WAf {}", amount_of_combinations);
        let mut combinations: Vec<Vec<Cell>> = Vec::new();

        // init combinations
        for _ in 0..amount_of_combinations {
            combinations.push(Vec::new());
        }

        // push all possible combinations
        for x in 0..unknown_amount {
            for y in 0..amount_of_combinations {
                let cell = match y % ((2 as u32).pow(x + 1)) == 0 {
                    true => Cell::None,
                    false => Cell::Spring,
                };
                combinations[y as usize].push(cell);
            }
        }

        let mut result: Vec<Record> = Vec::new();
        let group_sizes = &self.group_sizes;
        for combination in combinations {
            let mut combination_index = 0;

            let mut cells = self.cells.clone();

            for (i, cell) in self.cells.iter().enumerate() {
                if cell == &Cell::Unknown {
                    cells[i] = combination[combination_index];
                    combination_index += 1;
                }
            }

            let r = Self {
                cells,
                group_sizes: group_sizes.clone(),
            };
            result.push(r);
        }

        result
    }

    fn amount_of_possibilaties(&self) -> u32 {
        let ps = self.possibilities();
        ps.into_iter().filter(|r| r.is_valid()).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s1 = "..#...#...###. 1,1,3";
        let r1 = Record::from_line(s1);
        assert!(r1.is_valid());
    }

    #[test]
    fn amount_of_possibilaties() {
        let s1 = "?? 0";
        let r1 = Record::from_line(s1).possibilities();
        let expected_1 = vec![
            Record {
                cells: vec![Cell::None, Cell::None],
                group_sizes: vec![0],
            },
            Record {
                cells: vec![Cell::None, Cell::Spring],
                group_sizes: vec![0],
            },
            Record {
                cells: vec![Cell::Spring, Cell::Spring],
                group_sizes: vec![0],
            },
            Record {
                cells: vec![Cell::Spring, Cell::None],
                group_sizes: vec![0],
            },
        ];
        assert_eq!(r1, expected_1);
    }
}

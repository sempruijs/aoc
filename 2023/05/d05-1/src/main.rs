fn main() {
    let input = include_str!("../../example.txt");
    let answer = puzzle_input_to_answer(input);
    println!("{}", answer);
}

fn puzzle_input_to_answer(s: &str) -> u32 {
    let seeds = Seeds::from_input(s);
    let source_maps = SourceMaps::from_str(s);
    seeds
        .0
        .into_iter()
        .map(|n| source_maps.pipe_trough(n))
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
struct SourceMap(Vec<SourceLine>);

#[derive(Debug, Clone)]
struct SourceMaps(Vec<SourceMap>);

#[derive(Debug, Clone)]
struct SourceLine {
    input_start: u32,
    output_start: u32,
    len: u32,
}

#[derive(Debug)]
struct Seeds(Vec<u32>);

impl Seeds {
    fn from_input(s: &str) -> Self {
        Seeds(
            s.lines()
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(" ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    }
}

impl SourceMaps {
    fn from_str(s: &str) -> Self {
        let source_maps_str = s.split_once("\n\n").unwrap().1;
        // println!("{}", source_maps_str);
        SourceMaps(
            source_maps_str
                .split("\n\n")
                .map(|s| SourceMap::from_str(s))
                .collect(),
        )
    }

    fn pipe_trough(&self, n: u32) -> u32 {
        if self.0.is_empty() {
            return n;
        }
        let (x, xs) = (self.0[0].clone(), self.0[1..].to_vec());
        let n = x.pipe(n);
        SourceMaps(xs).pipe_trough(n)
    }
}

impl SourceMap {
    fn from_str(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        Self(
            lines[1..]
                .into_iter()
                .map(|line| SourceLine::from_line(line))
                .collect::<Vec<SourceLine>>(),
        )
    }

    pub fn pipe(&self, n: u32) -> u32 {
        for source_line in &self.0 {
            if source_line.input_start <= n && (source_line.input_start + source_line.len) > n {
                return (n as i32 + source_line.div()) as u32;
            }
        }
        n
    }
}

impl SourceLine {
    fn from_line(s: &str) -> Self {
        let xs: Vec<u32> = s
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Self {
            input_start: xs[1],
            output_start: xs[0],
            len: xs[2],
        }
    }

    fn div(&self) -> i32 {
        self.output_start as i32 - self.input_start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe() {
        let source_line_1 = SourceLine {
            input_start: 98,
            output_start: 50,
            len: 2,
        };
        let source_line_2 = SourceLine {
            input_start: 50,
            output_start: 52,
            len: 48,
        };
        let source_map = SourceMap(vec![source_line_1, source_line_2]);

        let result_1 = source_map.pipe(0); // should be 0
        let result_2 = source_map.pipe(50); // should be 52
        let result_3 = source_map.pipe(98); // should be 50

        assert_eq!(result_1, 0);
        assert_eq!(result_2, 52);
        assert_eq!(result_3, 50);
    }
}

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
    source_start: u32,
    destination_start: u32,
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
        0
    }
}

impl SourceLine {
    fn from_line(s: &str) -> Self {
        let xs: Vec<u32> = s
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Self {
            source_start: xs[0],
            destination_start: xs[1],
            len: xs[2],
        }
    }
}

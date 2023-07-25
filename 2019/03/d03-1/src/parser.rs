use crate::Step;

pub fn str_to_steps(s: &str) -> Vec<Step> {
    let str_steps: Vec<&str> = s.split(",").collect::<Vec<&str>>();
    let steps: Vec<Step> = str_steps.iter().map(|s| str_to_step(s)).collect();

    steps
}

fn str_to_step(s: &str) -> Step {
    let (dir, steps) = s.split_at(1);
    let steps: i32 = steps.parse().unwrap();
    match dir {
        "U" => Step::Y(steps),
        "R" => Step::X(steps),
        "D" => Step::Y(-steps),
        "L" => Step::X(-steps),
        _ => panic!("could not convert dir with intput: {}", dir),
    }
}

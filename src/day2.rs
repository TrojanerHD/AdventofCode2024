use std::cmp::Ordering;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn check_line(line: Vec<u32>) -> bool {
    let mut prev = None;
    let mut direction = None;
    for report in line {
        if let Some(act_prev) = prev {
            if report.abs_diff(act_prev) > 3
                || report == act_prev
                || (direction == Some(Direction::Increasing) && report < act_prev)
                || (direction == Some(Direction::Decreasing) && report > act_prev)
            {
                return false;
            }
            if direction.is_none() {
                direction = match report.cmp(&act_prev) {
                    Ordering::Greater => Some(Direction::Increasing),
                    Ordering::Less => Some(Direction::Decreasing),
                    Ordering::Equal => {
                        panic!("report {} must be different from prev {}", report, act_prev)
                    }
                }
            }
        }
        prev = Some(report);
    }
    true
}

pub fn part1(input: &str) -> String {
    let lines = input.lines().map(|line| {
        line.split_whitespace()
            .map(|it| it.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut res = 0;
    for line in lines {
        if check_line(line) {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let lines = input.lines().map(|line| {
        line.split_whitespace()
            .map(|it| it.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut res: u32 = 0;
    let mut corr_lines: Vec<(usize, _)> = Vec::new();
    for (i, line) in lines.enumerate() {
        corr_lines.push((i, line.clone()));
        for report in 0..line.len() {
            corr_lines.push((
                i,
                line.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != report)
                    .map(|(_, &e)| e)
                    .collect::<Vec<u32>>(),
            ))
        }
    }
    let mut taken = Vec::new();
    for (i, line) in corr_lines {
        if taken.contains(&i) {
            continue;
        }

        if check_line(line) {
            taken.push(i);
            res += 1;
        }
    }
    res.to_string().to_owned()
}

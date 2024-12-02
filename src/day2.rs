use std::cmp::Ordering;

pub fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut res = 0;
    'lines: for line in lines {
        let mut prev: Option<u32> = None;
        let mut increasing: i8 = 0;
        for report in line.split_whitespace().map(|it| it.parse::<u32>().unwrap()) {
            if let Some(act_prev) = prev {
                if report.abs_diff(act_prev) > 3
                    || report == act_prev
                    || (increasing == 1 && report < act_prev)
                    || (increasing == -1 && report > act_prev)
                {
                    continue 'lines;
                }
                if increasing == 0 {
                    match report.cmp(&act_prev) {
                        Ordering::Greater => increasing = 1,
                        Ordering::Less => increasing = -1,
                        Ordering::Equal => {}
                    }
                }
            }
            prev = Some(report);
        }
        res += 1;
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|it| it.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut res = 0;
    let mut corr_lines: Vec<(usize, Vec<u32>)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
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
    let mut taken: Vec<_> = Vec::new();
    'line: for (i, line) in corr_lines {
        if taken.contains(&i) {
            continue;
        }
        let mut prev = None;
        let mut increasing: i8 = 0;
        for report in line.clone() {
            if let Some(act_prev) = prev {
                if report.abs_diff(act_prev) > 3
                    || report == act_prev
                    || (increasing == 1 && report < act_prev)
                    || (increasing == -1 && report > act_prev)
                {
                    continue 'line;
                }
                if increasing == 0 {
                    match report.cmp(&act_prev) {
                        Ordering::Greater => increasing = 1,
                        Ordering::Less => increasing = -1,
                        Ordering::Equal => {}
                    }
                }
            }
            prev = Some(report);
        }
        taken.push(i);
        res += 1;
    }
    res.to_string().to_owned()
}

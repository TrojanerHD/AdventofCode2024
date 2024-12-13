use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug)]
struct Problem {
    a: Point,
    b: Point,
    dest: Point,
}

fn process_problems(problems: Vec<Problem>) -> u64 {
    let mut res = 0;
    for problem in problems {
        println!("Problem: {:?}", problem);
        let a_presses;
        let b_presses;
        match (problem.b.x * problem.a.y).cmp(&(problem.a.x * problem.b.y)) {
            Ordering::Less => {
                println!("det is less");
                if problem.a.x * problem.dest.y < problem.a.y * problem.dest.x {
                    println!("Unsolvable");
                    continue;
                }
                b_presses = Some(
                    (problem.a.x * problem.dest.y - problem.a.y * problem.dest.x)
                        / (problem.a.x * problem.b.y - problem.b.x * problem.a.y),
                );
                if problem.dest.x < problem.b.x * b_presses.unwrap() {
                    println!("Unsolvable");
                    continue;
                }
                a_presses = Some((problem.dest.x - problem.b.x * b_presses.unwrap()) / problem.a.x);
            }
            Ordering::Greater => {
                println!("det is greater");
                if problem.b.x * problem.dest.y < problem.b.y * problem.dest.x {
                    println!("Unsolvable");
                    continue;
                }
                a_presses = Some(
                    (problem.b.x * problem.dest.y - problem.b.y * problem.dest.x)
                        / (problem.b.x * problem.a.y - problem.a.x * problem.b.y),
                );

                if problem.dest.x < problem.a.x * a_presses.unwrap() {
                    println!("Unsolvable");
                    continue;
                }
                b_presses = Some((problem.dest.x - problem.a.x * a_presses.unwrap()) / problem.b.x);
            }
            _ => {
                println!("Unsolvable");
                continue;
            }
        }
        if a_presses.is_none() || b_presses.is_none() {
            println!("Unsolvable");
            continue;
        }
        let (a_presses, b_presses) = (a_presses.unwrap(), b_presses.unwrap());
        if problem.a.x * a_presses + problem.b.x * b_presses != problem.dest.x
            || problem.a.y * a_presses + problem.b.y * b_presses != problem.dest.y
        {
            println!("Unsolvable");
            continue;
        }
        println!("Final presses: a: {a_presses}, b: {b_presses}");
        res += a_presses * 3 + b_presses;
    }

    res
}

pub fn part1(input: &str) -> String {
    let empty_problem = Problem {
        a: Point { x: 0, y: 0 },
        b: Point { x: 0, y: 0 },
        dest: Point { x: 0, y: 0 },
    };
    let problems = input.lines().fold(vec![empty_problem], |mut acc, line| {
        if line.is_empty() {
            acc.push(empty_problem);
            acc
        } else if line.starts_with("Button") {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let x = parts[2].split("+").collect::<Vec<_>>()[1]
                .replace(",", "")
                .parse::<u64>()
                .unwrap();
            let y = parts[3].split("+").collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap();
            match parts[1] {
                "A:" => acc.last_mut().unwrap().a = Point { x, y },
                "B:" => acc.last_mut().unwrap().b = Point { x, y },
                a => panic!("Unexpected button {a}"),
            }
            acc
        } else {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let x = parts[1].split("=").collect::<Vec<_>>()[1]
                .replace(",", "")
                .parse::<u64>()
                .unwrap();
            let y = parts[2].split("=").collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap();
            acc.last_mut().unwrap().dest = Point { x, y };
            acc
        }
    });
    process_problems(problems).to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let empty_problem = Problem {
        a: Point { x: 0, y: 0 },
        b: Point { x: 0, y: 0 },
        dest: Point { x: 0, y: 0 },
    };
    let problems = input.lines().fold(vec![empty_problem], |mut acc, line| {
        if line.is_empty() {
            acc.push(empty_problem);
            acc
        } else if line.starts_with("Button") {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let x = parts[2].split("+").collect::<Vec<_>>()[1]
                .replace(",", "")
                .parse::<u64>()
                .unwrap();
            let y = parts[3].split("+").collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap();
            match parts[1] {
                "A:" => acc.last_mut().unwrap().a = Point { x, y },
                "B:" => acc.last_mut().unwrap().b = Point { x, y },
                a => panic!("Unexpected button {a}"),
            }
            acc
        } else {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let x = parts[1].split("=").collect::<Vec<_>>()[1]
                .replace(",", "")
                .parse::<u64>()
                .unwrap()
                + 10000000000000;
            let y = parts[2].split("=").collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap()
                + 10000000000000;
            acc.last_mut().unwrap().dest = Point { x, y };
            acc
        }
    });
    process_problems(problems).to_string().to_owned()
}

use std::{
    sync::{Arc, Mutex},
    thread,
};

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
    let res = Arc::new(Mutex::new(0));
    let handles = problems.into_iter().map(|problem| {
        let res = Arc::clone(&res);
        println!("Spawning thread");
        thread::spawn(move || {
            println!("Problem: {:?}", problem);
            let mut b_presses = (problem.dest.x / problem.b.x).min(problem.dest.y / problem.b.y);
            let mut a_presses = ((problem.dest.x - problem.b.x * b_presses) / problem.a.x)
                .min((problem.dest.y - problem.b.y * b_presses) / problem.a.y);
            let mut found = true;
            while problem.a.x * a_presses + problem.b.x * b_presses != problem.dest.x
                || problem.a.y * a_presses + problem.b.y * b_presses != problem.dest.y
            {
                if b_presses == 0 {
                    found = false;
                    break;
                }
                b_presses -= 1;
                // println!("Presses: a: {a_presses}, b: {b_presses}");
                a_presses = ((problem.dest.x - problem.b.x * b_presses) / problem.a.x)
                    .min((problem.dest.y - problem.b.y * b_presses) / problem.a.y);
            }

            if found && a_presses <= 100 && b_presses <= 100 {
                println!("Final presses: a: {a_presses}, b: {b_presses}");
                let mut res = res.lock().unwrap();
                *res += a_presses * 3 + b_presses;
            }
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    let res = res.lock().unwrap();
    res.to_string().to_owned()
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
    let res = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for problem in problems {
        let res = Arc::clone(&res);
        handles.push(thread::spawn(move || {
            println!("Problem: {:?}", problem);
            let mut b_presses = (problem.dest.x / problem.b.x).min(problem.dest.y / problem.b.y);
            let mut a_presses = ((problem.dest.x - problem.b.x * b_presses) / problem.a.x)
                .min((problem.dest.y - problem.b.y * b_presses) / problem.a.y);
            let mut found = true;
            while problem.a.x * a_presses + problem.b.x * b_presses != problem.dest.x
                || problem.a.y * a_presses + problem.b.y * b_presses != problem.dest.y
            {
                if b_presses == 0 {
                    found = false;
                    break;
                }
                b_presses -= 1;
                // println!("Presses: a: {a_presses}, b: {b_presses}");
                a_presses = ((problem.dest.x - problem.b.x * b_presses) / problem.a.x)
                    .min((problem.dest.y - problem.b.y * b_presses) / problem.a.y);
            }

            if found {
                println!("Final presses: a: {a_presses}, b: {b_presses}");
                let mut res = res.lock().unwrap();
                *res += a_presses * 3 + b_presses;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let res = res.lock().unwrap();
    res.to_string().to_owned()
}

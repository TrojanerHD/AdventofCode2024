use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let mut height = 0;
    let mut width = 0;
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            width = line.chars().count();
        }
        if line.is_empty() {
            height = i - 2;
            break;
        }
    }
    let mut row1 = true;
    let mut lock = true;
    let mut locks = HashSet::new();
    let mut keys = HashSet::new();
    let mut pattern = vec![0; width];
    for line in input.lines() {
        if line.is_empty() {
            row1 = true;
            if lock {
                locks.insert(pattern.iter().map(|p| p - 1).collect::<Vec<_>>());
            } else {
                keys.insert(pattern.iter().map(|p| p - 1).collect::<Vec<_>>());
            }
            pattern = vec![0; width];
            continue;
        }
        if row1 {
            if line.chars().next().unwrap() == '#' {
                lock = true;
            } else {
                lock = false;
            }
            row1 = false;
        }
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                pattern[j] += 1;
            }
        }
    }
    if lock {
        locks.insert(pattern.iter().map(|p| p - 1).collect::<Vec<_>>());
    } else {
        keys.insert(pattern.iter().map(|p| p - 1).collect::<Vec<_>>());
    }
    // println!("{:?}", locks);
    // println!("{:?}", keys);
    // println!("{}, {}", width, height);
    let mut res = 0;
    for lock in locks {
        'key: for key in keys.iter() {
            for i in 0..width {
                if lock[i] + key[i] > height {
                    continue 'key;
                }
            }
            res += 1;
        }
    }
    res.to_string().to_owned()
}

pub fn part2(_input: &str) -> String {
    "Easy 😉".to_owned()
}

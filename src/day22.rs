use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut secret = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            let result = secret << 6;
            secret ^= result;
            secret %= 16777216;
            let result = secret >> 5;
            secret ^= result;
            secret %= 16777216;
            let result = secret << 11;
            secret ^= result;
            secret %= 16777216;
        }
        res += secret;
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut prev_changes: HashMap<VecDeque<i32>, u64> = HashMap::new();
    for line in input.lines() {
        let mut seen = HashSet::new();
        let mut prev_4: VecDeque<i32> = VecDeque::new();
        let mut secret = line.parse::<u64>().unwrap();
        let mut prev = secret % 10;
        for _ in 0..2000 {
            let result = secret << 6;
            secret ^= result;
            secret %= 16777216;
            let result = secret >> 5;
            secret ^= result;
            secret %= 16777216;
            let result = secret << 11;
            secret ^= result;
            secret %= 16777216;
            prev_4.push_back(((secret % 10) as i32) - (prev as i32));
            prev = secret % 10;
            if prev_4.len() == 4 {
                // println!("{:?}", prev_4);
                if seen.contains(&prev_4) || secret % 10 == 0 {
                    prev_4.pop_front();
                    continue;
                }
                // if let Some(found) = combinations.get_mut(&prev_4) {
                //     *found += 1;
                // } else {
                //     combinations.insert(prev_4.clone(), 1);
                // }
                if let Some(found) = prev_changes.get_mut(&prev_4) {
                    *found += secret % 10;
                } else {
                    prev_changes.insert(prev_4.clone(), secret % 10);
                }
                seen.insert(prev_4.clone());
                prev_4.pop_front();
            }
        }
    }
    let mut max = 0;
    // let mut comb = None;
    println!("{:?}", prev_changes);
    for (combination, &val) in prev_changes.iter() {
        if max < val {
            println!("{:?}", combination);
            max = val;
            // comb = Some(combination.0.clone());
        }
    }
    println!("{}", prev_changes.values().max().unwrap());
    // let comb = comb.expect("Did not find highest combination");
    // println!("{:?}", comb);
    // let res: u64 = prev_changes.get(&comb).unwrap().iter().sum();
    max.to_string().to_owned()
}

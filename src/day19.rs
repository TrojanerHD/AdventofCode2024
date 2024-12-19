use std::collections::HashMap;

fn find_combination(
    cache: &mut HashMap<String, bool>,
    possible: &Vec<String>,
    produce: String,
) -> bool {
    if let Some(&found) = cache.get(&produce) {
        return found;
    }

    if produce.is_empty() {
        return true;
    }

    for towel in possible {
        if produce.starts_with(towel) {
            let valid = find_combination(cache, possible, produce.replacen(towel, "", 1));
            if valid {
                cache.insert(produce, true);
                return true;
            }
        }
    }
    cache.insert(produce, false);
    false
}
fn find_all_combinations(
    cache: &mut HashMap<String, Option<u64>>,
    possible: &Vec<String>,
    produce: String,
) -> Option<u64> {
    if let Some(&found) = cache.get(&produce) {
        return found;
    }

    if produce.is_empty() {
        return Some(1);
    }

    let mut res = 0;

    for towel in possible {
        if produce.starts_with(towel) {
            res += find_all_combinations(cache, possible, produce.replacen(towel, "", 1))
                .unwrap_or_default();
        }
    }

    if res == 0 {
        cache.insert(produce, None);
        return None;
    }
    cache.insert(produce, Some(res));
    Some(res)
}

pub fn part1(input: &str) -> String {
    let mut parse_1 = true;
    let mut possible = Vec::new();
    let mut produce = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            parse_1 = false;
            continue;
        }

        if parse_1 {
            possible = line.split(", ").map(|p| p.to_owned()).collect::<Vec<_>>();
        } else {
            produce.push(line);
        }
    }
    let mut cache = HashMap::new();
    let mut res = 0;
    for towel in produce {
        if find_combination(&mut cache, &possible, towel.to_owned()) {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut parse_1 = true;
    let mut possible = Vec::new();
    let mut produce = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            parse_1 = false;
            continue;
        }

        if parse_1 {
            possible = line.split(", ").map(|p| p.to_owned()).collect::<Vec<_>>();
        } else {
            produce.push(line);
        }
    }
    let mut cache = HashMap::new();
    let mut res = 0;
    for towel in produce {
        res += find_all_combinations(&mut cache, &possible, towel.to_owned()).unwrap_or_default();
    }
    res.to_string().to_owned()
}

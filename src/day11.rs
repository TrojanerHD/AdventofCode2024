use std::collections::HashMap;

fn insert_or_increment(hash_map: &mut HashMap<u64, u64>, val: u64, times: u64) {
    *hash_map.entry(val).or_default() += times;
}

fn calc_stones(input: &str, times: u8) -> u64 {
    let stones_input = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap());

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in stones_input {
        insert_or_increment(&mut stones, stone, 1);
    }

    for _ in 0..times {
        let mut new_stones = HashMap::new();
        for (stone, times) in stones {
            let stone_string = stone.to_string();
            if stone == 0 {
                insert_or_increment(&mut new_stones, 1, times);
            } else if stone_string.len() % 2 == 0 {
                let (stone1, stone2) = stone_string.split_at(stone_string.len() / 2);
                insert_or_increment(&mut new_stones, stone1.parse::<u64>().unwrap(), times);
                insert_or_increment(&mut new_stones, stone2.parse::<u64>().unwrap(), times);
            } else {
                insert_or_increment(&mut new_stones, stone * 2024, times);
            }
        }
        stones = new_stones;
    }
    stones.into_values().sum()
}

pub fn part1(input: &str) -> String {
    calc_stones(input, 25).to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    calc_stones(input, 75).to_string().to_owned()
}

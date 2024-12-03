use regex::Regex;

fn apply_mul(input: &str) -> u32 {
    let mut res = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        let num1 = &cap[1].parse::<u32>().unwrap();
        let num2 = &cap[2].parse::<u32>().unwrap();
        res += num1 * num2;
    }
    res
}

pub fn part1(input: &str) -> String {
    apply_mul(input).to_string().to_owned()
}
pub fn part2(input: &str) -> String {
    let real_input = input.lines().collect::<Vec<_>>().join("");
    let split_re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();
    let split_in = split_re
        .split(real_input.as_str())
        .collect::<Vec<_>>()
        .join("");
    apply_mul(split_in.as_str()).to_string().to_owned()
}

pub fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut ruleset_end = false;
    let mut rules = Vec::new();
    let mut res = 0;
    for line in lines {
        if line.is_empty() {
            ruleset_end = true;
            continue;
        }
        if !ruleset_end {
            let mut split = line.split("|");
            rules.push((split.next().unwrap(), split.next().unwrap()));
        } else {
            let values = line.split(",").collect::<Vec<_>>();
            let mut add = true;
            'linecheck: for (i, value) in values.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                for other_val in values.iter().take(i) {
                    if rules
                        .iter()
                        .any(|rule| rule.0 == *value && rule.1 == *other_val)
                    {
                        add = false;
                        break 'linecheck;
                    }
                }
            }
            if add {
                res += values[values.len() / 2].parse::<u32>().unwrap();
            }
        }
    }
    res.to_string().to_owned()
}
pub fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut ruleset_end = false;
    let mut rules = Vec::new();
    let mut res = 0;
    for line in lines {
        if line.is_empty() {
            ruleset_end = true;
            continue;
        }
        if !ruleset_end {
            let mut split = line.split("|");
            rules.push((split.next().unwrap(), split.next().unwrap()));
        } else {
            let values = line.split(",").collect::<Vec<_>>();
            let mut new_values = values.clone();
            let mut correct = true;
            loop {
                let mut swap = None;
                'linecheck: for (i, value) in new_values.iter().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    for (j, other_val) in new_values.iter().take(i).enumerate() {
                        if rules
                            .iter()
                            .any(|rule| rule.0 == *value && rule.1 == *other_val)
                        {
                            swap = Some((i, j));
                            correct = false;
                            break 'linecheck;
                        }
                    }
                }
                if let Some(act_swap) = swap {
                    let value = new_values[act_swap.0];
                    let other_val = new_values[act_swap.1];
                    new_values[act_swap.1] = value;
                    new_values[act_swap.0] = other_val;
                } else {
                    break;
                }
            }
            if !correct {
                res += new_values[new_values.len() / 2].parse::<u32>().unwrap();
            }
        }
    }
    res.to_string().to_owned()
}

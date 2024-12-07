use anyhow::Result;

pub fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let (result_str, operations_unsplit) = line.split_once(": ").unwrap();
        let result = result_str.parse::<u64>().unwrap();
        let operations = operations_unsplit
            .split_whitespace()
            .map(|operation| operation.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        for bin in 0..2u64.pow((operations.len() - 1) as u32) {
            let total =
                operations
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(operations[0], |acc, (i, operation)| {
                        if bin & (1 << i) != 0 {
                            acc * operation
                        } else {
                            acc + operation
                        }
                    });
            if total == result {
                res += result;
                break;
            }
        }
    }
    res.to_string().to_owned()
}

fn add_or_wrap(vec: &mut Vec<u8>, max_len: usize) -> Result<(), ()> {
    if !vec.is_empty() {
        for entry in &mut *vec {
            if *entry < 2 {
                *entry += 1;
                return Ok(());
            } else {
                *entry = 0;
            }
        }
    }
    if vec.len() < max_len {
        vec.push(0);
        Ok(())
    } else {
        Err(())
    }
}

pub fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let (result_str, operations_unsplit) = line.split_once(": ").unwrap();
        let result = result_str.parse::<u64>().unwrap();
        let operations = operations_unsplit
            .split_whitespace()
            .map(|operation| operation.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut all_operations: Vec<u8> = Vec::with_capacity(operations.len() - 1);
        loop {
            let total =
                operations
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(operations[0], |acc, (i, operation)| {
                        match all_operations.get(i) {
                            Some(0) | None => acc * operation,
                            Some(1) => acc + operation,
                            Some(2) => format!("{acc}{operation}").parse::<u64>().unwrap(),
                            Some(_) => panic!(
                                "Unexpected value {} for all_operations found",
                                all_operations[i]
                            ),
                        }
                    });
            if total == result {
                res += result;
                break;
            }
            if add_or_wrap(&mut all_operations, operations.len() - 1).is_err() {
                break;
            }
        }
    }
    res.to_string().to_owned()
}

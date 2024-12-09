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
        for bin in 0..3u64.pow((operations.len() - 1) as u32) {
            let total =
                operations
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(operations[0], |acc, (i, operation)| {
                        match bin / (3u64.pow(i as u32)) % 3 {
                            0 => acc * operation,
                            1 => acc + operation,
                            2 => format!("{acc}{operation}").parse::<u64>().unwrap(),
                            x => panic!("Unexpected value {x}"),
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

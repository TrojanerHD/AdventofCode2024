fn div(val: u32, val2: u32) -> u32 {
    if val2 <= val {
        val >> val2
    } else {
        0
    }
}

fn div64(val: u64, val2: u64) -> u64 {
    if val2 <= val {
        val >> val2
    } else {
        0
    }
}

pub fn part1(input: &str) -> String {
    let mut registers = [0; 3];
    let mut read_registers = true;
    let mut reg_count = 0;
    let mut program = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            read_registers = false;
            continue;
        }
        if read_registers {
            registers[reg_count] = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            reg_count += 1;
        } else {
            program = line
                .split_whitespace()
                .last()
                .unwrap()
                .split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
        }
    }
    let mut i = 0;
    let mut output = Vec::new();
    while i < program.len() {
        let opcode = program[i];
        let literal = program[i + 1];
        let combo = match literal {
            0..=3 => literal,
            4..=6 => registers[(literal - 4) as usize],
            7 => {
                i += 2;
                continue;
            }
            x => panic!("Invalid opcode {x}"),
        };
        match opcode {
            0 => {
                registers[0] = div(registers[0], combo);
            }
            1 => registers[1] ^= literal,
            2 => registers[1] = combo % 8,
            3 => {
                if registers[0] != 0 {
                    i = literal as usize;
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                // println!("Registers: {:?}", registers);
                output.push(combo % 8);
            }
            6 => registers[1] = div(registers[0], combo),
            7 => registers[2] = div(registers[0], combo),
            x => panic!("Invalid opcode {x}"),
        }
        i += 2;
    }
    output
        .iter()
        .map(|&val| val.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let mut registers = [0; 3];
    let mut read_registers = true;
    let mut reg_count = 0;
    let mut program = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            read_registers = false;
            continue;
        }
        if read_registers {
            registers[reg_count] = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            reg_count += 1;
        } else {
            program = line
                .split_whitespace()
                .last()
                .unwrap()
                .split(",")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
        }
    }
    // let mut a_reg = 281474976710656u64;
    let rev_prog = program.iter().rev().copied().collect::<Vec<_>>();
    // println!("Rev: {:?}", rev_prog);
    let mut a_reg = 0;
    let orig_registers = registers;
    let mut output = Vec::new();
    // This solution only works because my input ignores everything in registers b and c and overrides that
    // with some operations on a. It then reads precisely one time from b. Register a on the other hand, gets bitshifted to the right by three bits,
    // thus for this input, we only have to find numbers such that if we bitshift a, we will get the correct number in the reversed order of the program.
    'all_iters: loop {
        registers = orig_registers;
        registers[0] = a_reg;
        // println!("{a_reg}");
        let mut i = 0;
        while i < program.len() {
            let opcode = program[i];
            let literal = program[i + 1];
            let combo = match literal {
                0..=3 => literal,
                4..=6 => registers[(literal - 4) as usize],
                7 => {
                    i += 2;
                    continue;
                }
                x => panic!("Invalid opcode {x}"),
            };
            match opcode {
                0 => {
                    registers[0] = div64(registers[0], combo);
                }
                1 => registers[1] ^= literal,
                2 => registers[1] = combo % 8,
                3 => {
                    if registers[0] != 0 {
                        i = literal as usize;
                        continue;
                    }
                }
                4 => registers[1] ^= registers[2],
                5 => {
                    if combo % 8 != rev_prog[output.len()] {
                        break;
                    }
                    // println!("place {} correct", output.len());
                    output.push(combo % 8);
                    if output == rev_prog {
                        // println!("{:?}", output);
                        break 'all_iters a_reg.to_string().to_owned();
                    }
                    a_reg <<= 3;
                    // println!("{:?}", output);
                    continue 'all_iters;
                }
                6 => registers[1] = div64(registers[0], combo),
                7 => registers[2] = div64(registers[0], combo),
                x => panic!("Invalid opcode {x}"),
            }
            i += 2;
        }
        // println!("{:?}", output);
        if (a_reg + 1) % 8 == 0 {
            if output.pop().is_none() {
                panic!("Could not find number");
            }
            a_reg >>= 3;
        }
        a_reg += 1;
    }
}

pub fn part1(input: &str) -> String {
    let lines = input
        .lines()
        .map(|it| it.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, letter) in line.iter().enumerate() {
            if *letter != 'X' {
                continue;
            }
            for i in 0..=2 {
                for j in 0..=2 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    if !(x == 2 && y == 2 && j == 0 && i == 0) {
                        if y <= 2 && j == 0 {
                            continue;
                        }
                        if x <= 2 && i == 0 {
                            continue;
                        }
                    }
                    if !(x == lines.len() - 1 - 2 && y == lines.len() - 1 - 2 && j == 2 && i == 2) {
                        if y > lines.len() - 1 - 3 && j == 2 {
                            continue;
                        }
                        if x > lines[0].len() - 1 - 3 && i == 2 {
                            continue;
                        }
                    }

                    if lines[y + j - 1][x + i - 1] != 'M' {
                        continue;
                    }
                    if lines[y + 2 * j - 2][x + 2 * i - 2] != 'A' {
                        continue;
                    }
                    if lines[y + 3 * j - 3][x + 3 * i - 3] != 'S' {
                        continue;
                    }
                    res += 1;
                }
            }
        }
    }
    res.to_string().to_owned()
}
pub fn part2(input: &str) -> String {
    let lines = input
        .lines()
        .map(|it| it.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (y, line) in lines.iter().enumerate() {
        if y == 0 || y == lines.len() - 1 {
            continue;
        }
        for (x, letter) in line.iter().enumerate() {
            if x == 0 || x == lines[0].len() - 1 {
                continue;
            }
            if *letter != 'A' {
                continue;
            }
            if !((lines[y - 1][x - 1] == 'M' && lines[y + 1][x + 1] == 'S')
                || (lines[y - 1][x - 1] == 'S' && lines[y + 1][x + 1] == 'M'))
            {
                continue;
            }
            if !((lines[y - 1][x + 1] == 'M' && lines[y + 1][x - 1] == 'S')
                || (lines[y - 1][x + 1] == 'S' && lines[y + 1][x - 1] == 'M'))
            {
                continue;
            }
            res += 1;
        }
    }
    res.to_string().to_owned()
}

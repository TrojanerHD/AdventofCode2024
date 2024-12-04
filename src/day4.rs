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
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if !(x == 2 && y == 2 && j == -1 && i == -1) {
                        if y <= 2 && j == -1 {
                            continue;
                        }
                        if x <= 2 && i == -1 {
                            continue;
                        }
                    }
                    let last_row_index = lines.len() - 1;
                    let last_column_index = lines[0].len() - 1;
                    if !(x == last_row_index - 2 && y == last_row_index - 2 && j == 1 && i == 1) {
                        if y > last_row_index - 3 && j == 1 {
                            continue;
                        }
                        if x > last_column_index - 3 && i == 1 {
                            continue;
                        }
                    }

                    if lines[y.saturating_add_signed(j)][x.saturating_add_signed(i)] != 'M' {
                        continue;
                    }
                    if lines[y.saturating_add_signed(2 * j)][x.saturating_add_signed(2 * i)] != 'A'
                    {
                        continue;
                    }
                    if lines[y.saturating_add_signed(3 * j)][x.saturating_add_signed(3 * i)] != 'S'
                    {
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

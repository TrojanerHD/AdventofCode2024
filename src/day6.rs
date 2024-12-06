use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

fn by_symbol(symbol: char) -> Dir {
    match symbol {
        '^' => Dir::Up,
        '>' => Dir::Right,
        'v' => Dir::Down,
        '<' => Dir::Left,
        _ => panic!("Unexpected symbol: {symbol} while parsing direction of guard"),
    }
}

fn rotate_clck(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Pos {
    guard: Option<Dir>,
    obst: bool,
}

fn guard_move(
    positions: Vec<Vec<Pos>>,
    mut guard_pos: Point,
    care_inf: bool,
) -> (HashSet<Point>, bool) {
    let mut guard = positions[guard_pos.y][guard_pos.x].guard.unwrap();
    let mut seen = Vec::new();
    let mut inf_loop = false;
    seen.push((guard_pos, guard));
    'dirLoop: loop {
        let mut move_x = 0;
        let mut move_y = 0;
        match guard {
            Dir::Up => move_y = -1,
            Dir::Left => move_x = -1,
            Dir::Right => move_x = 1,
            Dir::Down => move_y = 1,
        }
        while !positions[guard_pos.y.saturating_add_signed(move_y)]
            [guard_pos.x.saturating_add_signed(move_x)]
        .obst
        {
            guard_pos.y = guard_pos.y.saturating_add_signed(move_y);
            guard_pos.x = guard_pos.x.saturating_add_signed(move_x);
            if care_inf && seen.iter().any(|pos| pos.0 == guard_pos && pos.1 == guard) {
                inf_loop = true;
                break 'dirLoop;
            }
            seen.push((guard_pos, guard));

            if (guard_pos.x == 0 && guard == Dir::Left)
                || (guard_pos.x == positions[0].len() - 1 && guard == Dir::Right)
                || (guard_pos.y == 0 && guard == Dir::Up)
                || (guard_pos.y == positions.len() - 1 && guard == Dir::Down)
            {
                break 'dirLoop;
            }
        }
        guard = rotate_clck(guard);
    }
    let seen_set = HashSet::from_iter(seen.iter().map(|pos| pos.0));
    (seen_set, inf_loop)
}

pub fn part1(input: &str) -> String {
    let mut guard_pos = None;
    let positions = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, pos)| match pos {
                    '#' => Pos {
                        guard: None,
                        obst: true,
                    },
                    '^' | '>' | 'v' | '<' => {
                        guard_pos = Some(Point { x, y });
                        Pos {
                            guard: Some(by_symbol(pos)),
                            obst: false,
                        }
                    }
                    _ => Pos {
                        guard: None,
                        obst: false,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let Some(guard_pos) = guard_pos else {
        panic!("No guard on field");
    };

    let (seen, _) = guard_move(positions, guard_pos, false);

    seen.len().to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut guard_pos = None;
    let positions = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, pos)| match pos {
                    '#' => Pos {
                        guard: None,
                        obst: true,
                    },
                    '^' | '>' | 'v' | '<' => {
                        guard_pos = Some(Point { x, y });
                        Pos {
                            guard: Some(by_symbol(pos)),
                            obst: false,
                        }
                    }
                    _ => Pos {
                        guard: None,
                        obst: false,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let Some(guard_pos) = guard_pos else {
        panic!("No guard on field");
    };

    let (seen, _) = guard_move(positions.clone(), guard_pos, false);

    let mut res = 0;

    for pos in seen.iter().filter(|pos| **pos != guard_pos) {
        let mut new_positions = positions.clone();
        new_positions[pos.y][pos.x].obst = true;
        let (_, inf_loop) = guard_move(new_positions, guard_pos, true);
        if inf_loop {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

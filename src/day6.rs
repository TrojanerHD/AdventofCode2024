use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Field {
    Obst,
    Guard,
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    guard: Option<Dir>,
    obst: bool,
}

impl Pos {
    fn get_field(self) -> Option<Field> {
        if self.guard.is_some() {
            Some(Field::Guard)
        } else if self.obst {
            Some(Field::Obst)
        } else {
            None
        }
    }
}

fn guard_move(positions: Vec<Vec<Pos>>, mut guard_pos: Point) -> (HashSet<Point>, bool) {
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
            if seen.iter().any(|pos| pos.0 == guard_pos && pos.1 == guard) {
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
    let positions = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|pos| match pos {
                    '#' => Pos {
                        guard: None,
                        obst: true,
                    },
                    '^' | '>' | 'v' | '<' => Pos {
                        guard: Some(by_symbol(pos)),
                        obst: false,
                    },
                    _ => Pos {
                        guard: None,
                        obst: false,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    for (y, line) in positions.iter().enumerate() {
        for (x, pos) in line.iter().enumerate() {
            if pos.get_field() == Some(Field::Guard) {
                let mut guard = pos.guard.unwrap();
                let mut guard_pos_x = x;
                let mut guard_pos_y = y;
                seen.insert(Point { x, y });
                'dirLoop: loop {
                    let mut move_x = 0;
                    let mut move_y = 0;
                    match guard {
                        Dir::Up => move_y = -1,
                        Dir::Left => move_x = -1,
                        Dir::Right => move_x = 1,
                        Dir::Down => move_y = 1,
                    }
                    while !positions[guard_pos_y.saturating_add_signed(move_y)]
                        [guard_pos_x.saturating_add_signed(move_x)]
                    .obst
                    {
                        guard_pos_y = guard_pos_y.saturating_add_signed(move_y);
                        guard_pos_x = guard_pos_x.saturating_add_signed(move_x);
                        seen.insert(Point {
                            x: guard_pos_x,
                            y: guard_pos_y,
                        });
                        if (guard_pos_x == 0 && guard == Dir::Left)
                            || (guard_pos_x == positions[0].len() - 1 && guard == Dir::Right)
                            || (guard_pos_y == 0 && guard == Dir::Up)
                            || (guard_pos_y == positions.len() - 1 && guard == Dir::Down)
                        {
                            break 'dirLoop;
                        }
                    }
                    guard = rotate_clck(guard);
                }
            }
        }
    }

    seen.len().to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut guard_pos = Point { x: 0, y: 0 };
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
                        guard_pos = Point { x, y };
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

    let (seen, _) = guard_move(positions.clone(), guard_pos);

    let mut res = 0;

    for pos in seen.iter().filter(|pos| **pos != guard_pos) {
        let mut new_positions = positions.clone();
        new_positions[pos.y][pos.x].obst = true;
        // for row in new_positions.iter() {
        //     for pos in row.iter() {
        //         print!(
        //             "{}",
        //             match pos.get_field() {
        //                 Some(Field::Guard) => '^',
        //                 Some(Field::Obst) => '#',
        //                 _ => '.',
        //             }
        //         );
        //     }
        //     println!();
        // }
        let (_, inf_loop) = guard_move(new_positions, guard_pos);
        if inf_loop {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct JustPoint {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
    val: u32,
    origin: JustPoint,
}

pub fn part1(input: &str) -> String {
    let top_map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { 10 } else { c as u32 - '0' as u32 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start_poses = top_map.iter().enumerate().flat_map(move |(y, line)| {
        line.iter().enumerate().filter_map(move |(x, val)| {
            if *val == 0 {
                Some(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    val: *val,
                    origin: JustPoint {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                })
            } else {
                None
            }
        })
    });

    let mut good_neighbors = start_poses.clone().collect::<VecDeque<_>>();
    let mut reachable = HashSet::new();
    loop {
        let Some(neighbor) = good_neighbors.pop_front() else {
            break;
        };
        if neighbor.val == 9 {
            reachable.insert(neighbor);
            continue;
        }
        for offset in [-1, 1] {
            if !((neighbor.x == 0 && offset == -1)
                || (neighbor.x == (top_map[0].len() - 1).try_into().unwrap()) && offset == 1)
                && top_map[neighbor.y as usize][neighbor.x.saturating_add_signed(offset) as usize]
                    == neighbor.val + 1
            {
                good_neighbors.push_back(Point {
                    x: neighbor.x.saturating_add_signed(offset),
                    y: neighbor.y,
                    val: neighbor.val + 1,
                    origin: neighbor.origin,
                });
            }
            if !((neighbor.y == 0 && offset == -1)
                || (neighbor.y == (top_map.len() - 1).try_into().unwrap() && offset == 1))
                && top_map[neighbor.y.saturating_add_signed(offset) as usize][neighbor.x as usize]
                    == neighbor.val + 1
            {
                good_neighbors.push_back(Point {
                    x: neighbor.x,
                    y: neighbor.y.saturating_add_signed(offset),
                    val: neighbor.val + 1,
                    origin: neighbor.origin,
                });
            }
        }
        // println!("{:?}", good_neighbors);
    }

    reachable.len().to_string().to_owned()
}
pub fn part2(input: &str) -> String {
    let top_map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { 10 } else { c as u32 - '0' as u32 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start_poses = top_map.iter().enumerate().flat_map(move |(y, line)| {
        line.iter().enumerate().filter_map(move |(x, val)| {
            if *val == 0 {
                Some(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    val: *val,
                    origin: JustPoint {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                })
            } else {
                None
            }
        })
    });

    let mut good_neighbors = start_poses.clone().collect::<VecDeque<_>>();
    let mut res = 0;
    loop {
        let Some(neighbor) = good_neighbors.pop_front() else {
            break;
        };
        if neighbor.val == 9 {
            res += 1;
            continue;
        }
        for offset in [-1, 1] {
            if !((neighbor.x == 0 && offset == -1)
                || (neighbor.x == (top_map[0].len() - 1).try_into().unwrap()) && offset == 1)
                && top_map[neighbor.y as usize][neighbor.x.saturating_add_signed(offset) as usize]
                    == neighbor.val + 1
            {
                good_neighbors.push_back(Point {
                    x: neighbor.x.saturating_add_signed(offset),
                    y: neighbor.y,
                    val: neighbor.val + 1,
                    origin: neighbor.origin,
                });
            }
            if !((neighbor.y == 0 && offset == -1)
                || (neighbor.y == (top_map.len() - 1).try_into().unwrap() && offset == 1))
                && top_map[neighbor.y.saturating_add_signed(offset) as usize][neighbor.x as usize]
                    == neighbor.val + 1
            {
                good_neighbors.push_back(Point {
                    x: neighbor.x,
                    y: neighbor.y.saturating_add_signed(offset),
                    val: neighbor.val + 1,
                    origin: neighbor.origin,
                });
            }
        }
        // println!("{:?}", good_neighbors);
    }

    res.to_string().to_owned()
}

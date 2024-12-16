use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
enum Obj {
    Wall,
    Start,
    End,
    Air,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Top,
    Left,
    Bot,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn step(
    map: &Vec<Vec<Obj>>,
    score: u32,
    scores: &mut HashMap<Point, HashMap<Dir, u32>>,
    pos: Point,
    dir: Dir,
    mut visited: HashSet<Point>,
    end: Point,
    all_visited: &mut HashSet<Point>,
) {
    if let Some(p) = scores.get_mut(&pos) {
        if let Some(&d) = p.get(&dir) {
            if score > d {
                return;
            }
        }
        p.insert(dir, score);
    } else {
        let mut p = HashMap::new();
        p.insert(dir, score);
        scores.insert(pos, p);
    }
    if pos == end {
        for val in scores.get_mut(&pos).unwrap().values_mut() {
            *val = score
        }
        // for y in 0..map.len() {
        //     for x in 0..map[0].len() {
        //         match map[y][x] {
        //             Obj::Wall => print!("#"),
        //             Obj::Start => print!("S"),
        //             Obj::End => print!("E"),
        //             Obj::Air => {
        //                 let point = Point {
        //                     x: x.try_into().unwrap(),
        //                     y: y.try_into().unwrap(),
        //                 };
        //                 if visited.contains(&point) {
        //                     print!("O")
        //                 } else {
        //                     print!(".")
        //                 }
        //             }
        //         }
        //     }
        //     println!();
        // }
        all_visited.extend(visited);
        return;
    }
    visited.insert(pos);
    let (x, y) = (pos.x as usize, pos.y as usize);
    let above = Point {
        x: pos.x,
        y: pos.y - 1,
    };
    let right = Point {
        x: pos.x + 1,
        y: pos.y,
    };
    let below = Point {
        x: pos.x,
        y: pos.y + 1,
    };
    let left = Point {
        x: pos.x - 1,
        y: pos.y,
    };
    if dir == Dir::Top && !visited.contains(&above) && map[y - 1][x] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Top { 1 } else { 1001 },
            scores,
            above,
            Dir::Top,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir == Dir::Right && !visited.contains(&right) && map[y][x + 1] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Right { 1 } else { 1001 },
            scores,
            right,
            Dir::Right,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir == Dir::Bot && !visited.contains(&below) && map[y + 1][x] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Bot { 1 } else { 1001 },
            scores,
            below,
            Dir::Bot,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir == Dir::Left && !visited.contains(&left) && map[y][x - 1] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Left { 1 } else { 1001 },
            scores,
            left,
            Dir::Left,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir != Dir::Top && !visited.contains(&above) && map[y - 1][x] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Top { 1 } else { 1001 },
            scores,
            above,
            Dir::Top,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir != Dir::Right && !visited.contains(&right) && map[y][x + 1] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Right { 1 } else { 1001 },
            scores,
            right,
            Dir::Right,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir != Dir::Bot && !visited.contains(&below) && map[y + 1][x] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Bot { 1 } else { 1001 },
            scores,
            below,
            Dir::Bot,
            visited.clone(),
            end,
            all_visited,
        );
    }
    if dir != Dir::Left && !visited.contains(&left) && map[y][x - 1] != Obj::Wall {
        step(
            map,
            score + if dir == Dir::Left { 1 } else { 1001 },
            scores,
            left,
            Dir::Left,
            visited.clone(),
            end,
            all_visited,
        );
    }
}

pub fn part1(input: &str) -> String {
    let mut start = None;
    let mut end = None;
    let mut scores = HashMap::new();
    let min: u32 = (input.lines().collect::<Vec<_>>().len()
        * input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len()
        * 1000)
        .try_into()
        .unwrap();
    let objs = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let point = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    let mut dirs = HashMap::new();
                    dirs.insert(Dir::Top, min);
                    dirs.insert(Dir::Left, min);
                    dirs.insert(Dir::Bot, min);
                    dirs.insert(Dir::Right, min);
                    scores.insert(point, dirs);
                    let obj = match c {
                        '#' => Obj::Wall,
                        'S' => Obj::Start,
                        'E' => Obj::End,
                        '.' => Obj::Air,
                        _ => panic!("Failed to parse unknown symbol {c} in map"),
                    };
                    if obj == Obj::Start {
                        start = Some(Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        });
                    }
                    if obj == Obj::End {
                        end = Some(Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        });
                    }
                    obj
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = start.expect("Could not find start on map");
    let end = end.expect("Could not find end on map");

    let mut visited = HashSet::new();

    step(
        &objs,
        0,
        &mut scores,
        start,
        Dir::Right,
        HashSet::new(),
        end,
        &mut visited,
    );
    scores
        .get(&end)
        .unwrap()
        .values()
        .min()
        .unwrap()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let mut start = None;
    let mut end = None;
    let mut scores = HashMap::new();
    let min: u32 = (input.lines().collect::<Vec<_>>().len()
        * input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len()
        * 1000)
        .try_into()
        .unwrap();
    let objs = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let point = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    let mut dirs = HashMap::new();
                    dirs.insert(Dir::Top, min);
                    dirs.insert(Dir::Left, min);
                    dirs.insert(Dir::Bot, min);
                    dirs.insert(Dir::Right, min);
                    scores.insert(point, dirs);
                    let obj = match c {
                        '#' => Obj::Wall,
                        'S' => Obj::Start,
                        'E' => Obj::End,
                        '.' => Obj::Air,
                        _ => panic!("Failed to parse unknown symbol {c} in map"),
                    };
                    if obj == Obj::Start {
                        start = Some(Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        });
                    }
                    if obj == Obj::End {
                        end = Some(Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        });
                    }
                    obj
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = start.expect("Could not find start on map");
    let end = end.expect("Could not find end on map");

    let mut visited = HashSet::new();

    step(
        &objs,
        0,
        &mut scores,
        start,
        Dir::Right,
        HashSet::new(),
        end,
        &mut visited,
    );
    visited = HashSet::new();
    step(
        &objs,
        0,
        &mut scores,
        start,
        Dir::Right,
        HashSet::new(),
        end,
        &mut visited,
    );
    // for y in 0..objs.len() {
    //     for x in 0..objs[0].len() {
    //         match objs[y][x] {
    //             Obj::Wall => print!("#"),
    //             Obj::Start => print!("S"),
    //             Obj::End => print!("E"),
    //             Obj::Air => {
    //                 let point = Point {
    //                     x: x.try_into().unwrap(),
    //                     y: y.try_into().unwrap(),
    //                 };
    //                 if visited.contains(&point) {
    //                     print!("O")
    //                 } else {
    //                     print!(".")
    //                 }
    //             }
    //         }
    //     }
    //     println!();
    // }
    (visited.len() + 1).to_string().to_owned()
}

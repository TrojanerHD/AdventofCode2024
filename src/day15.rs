use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Point {
    x: u32,
    y: u32,
    // neighbors: HashMap<Dir, &'a Point<'a>>,
    obj: Obj,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point2 {
    x: u32,
    other_x: u32,
    y: u32,
    obj: Obj,
}

// impl Point<'_> {
//     fn all_in_way<'a>(&'a mut self, dir: Dir) -> Vec<&'a mut Point<'a>> {
//         let mut points = vec![self];
//         while let Some(neighbor) = points.last_mut().unwrap().neighbors.get_mut(&dir) {
//             points.push(neighbor);
//         }
//         points
//     }

//     fn ends_in_wall(self, dir: Dir) -> bool {
//         if let Some(last) = self.all_in_way(dir).last() {
//             last.obj != Obj::Wall
//         } else {
//             false
//         }
//     }
// }

fn dir_offset(dir: Dir) -> (i32, i32) {
    let mut delta_y = 0;
    let mut delta_x = 0;

    match dir {
        Dir::Top => delta_y = -1,
        Dir::Left => delta_x = -1,
        Dir::Bot => delta_y = 1,
        Dir::Right => delta_x = 1,
    }
    (delta_x, delta_y)
}

impl Point {
    fn all_in_way(self, points: Vec<Point>, dir: Dir) -> Vec<usize> {
        let mut point = self;
        let mut found = Vec::new();

        let (delta_x, delta_y) = dir_offset(dir);
        while let Some(neighbor) = points.iter().position(|p| {
            if let Some(new_x) = point.x.checked_add_signed(delta_x) {
                if let Some(new_y) = point.y.checked_add_signed(delta_y) {
                    p.x == new_x && p.y == new_y
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            point = points[neighbor].clone();
            found.push(neighbor);
            if point.obj == Obj::Wall {
                break;
            }
        }
        found
    }
}

impl Point2 {
    fn all_in_way_2(self, points: Vec<Point2>, dir: Dir) -> Result<HashSet<usize>, ()> {
        let (mut x, mut y) = (self.x, self.y);
        let (delta_x, delta_y) = dir_offset(dir);
        let mut found = HashSet::new();
        if dir == Dir::Left || dir == Dir::Right {
            loop {
                x = x.checked_add_signed(delta_x).unwrap();
                let neighbors = points
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| {
                        if (p.x == x || p.other_x == x) && p.y == y {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if neighbors.is_empty() {
                    return Ok(found);
                }
                let neighbor = if dir == Dir::Left {
                    neighbors.last()
                } else {
                    neighbors.first()
                }
                .unwrap();
                if points[*neighbor].obj == Obj::Wall {
                    return Err(());
                }
                found.insert(*neighbor);
            }
        } else {
            let mut xs = VecDeque::new();
            xs.push_back(x);
            loop {
                if xs.is_empty() {
                    return Ok(found);
                }
                y = y.checked_add_signed(delta_y).unwrap();
                let mut xs_new = xs.clone();
                xs = VecDeque::new();
                while let Some(x) = xs_new.pop_front() {
                    let neighbors = points
                        .iter()
                        .enumerate()
                        .filter_map(|(i, p)| {
                            if (p.x == x || p.other_x == x) && p.y == y {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    if neighbors.is_empty() {
                        continue;
                    }
                    let neighbor = if dir == Dir::Left {
                        neighbors.last()
                    } else {
                        neighbors.first()
                    }
                    .unwrap();
                    if points[*neighbor].obj == Obj::Wall {
                        return Err(());
                    }
                    found.insert(*neighbor);
                    xs.push_back(points[*neighbor].x);
                    xs.push_back(points[*neighbor].other_x);
                }
            }
        }
        // let mut all_points = VecDeque::new();
        // all_points.push_back(self.clone());
        // let mut found = Vec::new();

        // let (mut delta_x, mut delta_y) = dir_offset(dir);
        // loop {
        //     let Some(point) = all_points.pop_front() else {
        //         break;
        //     };
        //     if point.obj == Obj::Wall && point != self {
        //         return Err(());
        //     }
        //     for neighbor in
        //         HashSet::<usize>::from_iter(points.iter().enumerate().flat_map(|(i, p)| {
        //             let mut found = Vec::new();
        //             if let Some(new_x) = point.x.checked_add_signed(delta_x) {
        //                 if let Some(new_y) = point.y.checked_add_signed(delta_y) {
        //                     if (p.x == new_x || p.other_x == new_x) && p.y == new_y {
        //                         found.push(i);
        //                     }
        //                 }
        //             }
        //             if let Some(new_x) = point.other_x.checked_add_signed(delta_x) {
        //                 if let Some(new_y) = point.y.checked_add_signed(delta_y) {
        //                     if (p.x == new_x || p.other_x == new_x) && p.y == new_y {
        //                         found.push(i);
        //                     }
        //                 }
        //             }
        //             found
        //         }))
        //     {
        //         all_points.push_back(points[neighbor].clone());
        //         found.push(neighbor);
        //     }
        //     (delta_x, delta_y) = dir_offset_2(dir);
        // }
        // Ok(found)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Top,
    Left,
    Bot,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Obj {
    Wall,
    Box,
}

fn parse_instr(c: char) -> Dir {
    match c {
        '^' => Dir::Top,
        '>' => Dir::Right,
        'v' => Dir::Bot,
        '<' => Dir::Left,
        _ => panic!("Unexpected character when parsing instructions: {c}"),
    }
}

pub fn part1(input: &str) -> String {
    let mut parsing_map = true;
    let mut map: Vec<Point> = Vec::new();
    let mut bot = None;
    let mut instrs = Vec::new();
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            parsing_map = false;
        }
        for (x, c) in line.chars().enumerate() {
            if parsing_map {
                let mut point = Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    // neighbors: HashMap::new(),
                    obj: Obj::Wall,
                };
                match c {
                    '#' => {}
                    '.' => continue,
                    'O' => point.obj = Obj::Box,
                    '@' => {
                        bot = Some(point);
                        continue;
                    }
                    _ => panic!("Invalid letter at parsing map: {c}"),
                }
                map.push(point);
            } else {
                instrs.push(parse_instr(c));
            }
        }
    }

    // let old_map = map_without_ngbhrs.clone();

    // let map = map_without_ngbhrs.iter_mut().map(|p: &mut Point<'_>| {
    //     let mut new_point = p.clone();
    //     if p.x != 0 {
    //         if let Some(found) = old_map.iter().find(|p2| p2.x == p.x - 1 && p2.y == p.y) {
    //             new_point.neighbors.insert(Dir::Left, found);
    //         }
    //     }
    //     if p.y != 0 {
    //         if let Some(found) = old_map.iter().find(|p2| p2.x == p.x && p2.y == p.y - 1) {
    //             new_point.neighbors.insert(Dir::Left, found);
    //         }
    //     }
    //     if let Some(found) = old_map.iter().find(|p2| p2.x == p.x + 1 && p2.y == p.y) {
    //         new_point.neighbors.insert(Dir::Left, found);
    //     }
    //     if let Some(found) = old_map.iter().find(|p2| p2.x == p.x && p2.y == p.y + 1) {
    //         new_point.neighbors.insert(Dir::Left, found);
    //     }

    //     new_point
    // });

    let mut bot = bot.expect("No bot found on map");

    for dir in instrs {
        let (delta_x, delta_y) = dir_offset(dir);
        let all = bot.clone().all_in_way(map.clone(), dir);
        // println!("{}, {}", bot.x, bot.y);
        if !all.is_empty() && map[*all.last().unwrap()].obj == Obj::Wall {
            continue;
        }
        for i in all {
            let p = &mut map[i];
            p.x = p.x.checked_add_signed(delta_x).unwrap();
            p.y = p.y.checked_add_signed(delta_y).unwrap();
        }
        bot.x = bot.x.checked_add_signed(delta_x).unwrap();
        bot.y = bot.y.checked_add_signed(delta_y).unwrap();
        // for y in 0..10 {
        //     for x in 0..10 {
        //         if bot.x == x && bot.y == y {
        //             print!("@");
        //         } else if let Some(found) = map.iter().find(|p| p.x == x && p.y == y) {
        //             let obj = match found.obj {
        //                 Obj::Box => "O",
        //                 Obj::Wall => "#",
        //             };
        //             print!("{obj}");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
    }

    let res = map
        .iter()
        .filter(|p| p.obj == Obj::Box)
        .fold(0, |acc, box_found| acc + 100 * box_found.y + box_found.x);
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut parsing_map = true;
    let mut map: Vec<Point2> = Vec::new();
    let mut bot = None;
    let mut instrs = Vec::new();
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            parsing_map = false;
        }
        'aa: for (x, c) in line.chars().enumerate() {
            if parsing_map {
                let mut point = Point2 {
                    x: (x * 2).try_into().unwrap(),
                    other_x: (x * 2 + 1).try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    // neighbors: HashMap::new(),
                    obj: Obj::Wall,
                };
                match c {
                    '#' => {}
                    '.' => continue,
                    'O' => {
                        point.obj = Obj::Box;
                    }
                    '@' => {
                        bot = Some(point);
                        continue 'aa;
                    }
                    _ => panic!("Invalid letter at parsing map: {c}"),
                }
                map.push(point);
            } else {
                instrs.push(parse_instr(c));
            }
        }
    }

    // let old_map = map_without_ngbhrs.clone();

    // let map = map_without_ngbhrs.iter_mut().map(|p: &mut Point<'_>| {
    //     let mut new_point = p.clone();
    //     if p.x != 0 {
    //         if let Some(found) = old_map.iter().find(|p2| p2.x == p.x - 1 && p2.y == p.y) {
    //             new_point.neighbors.insert(Dir::Left, found);
    //         }
    //     }
    //     if p.y != 0 {
    //         if let Some(found) = old_map.iter().find(|p2| p2.x == p.x && p2.y == p.y - 1) {
    //             new_point.neighbors.insert(Dir::Left, found);
    //         }
    //     }
    //     if let Some(found) = old_map.iter().find(|p2| p2.x == p.x + 1 && p2.y == p.y) {
    //         new_point.neighbors.insert(Dir::Left, found);
    //     }
    //     if let Some(found) = old_map.iter().find(|p2| p2.x == p.x && p2.y == p.y + 1) {
    //         new_point.neighbors.insert(Dir::Left, found);
    //     }

    //     new_point
    // });

    let mut bot = bot.expect("No bot found on map");

    for dir in instrs {
        // println!("{:?}", dir);
        let (delta_x, delta_y) = dir_offset(dir);
        let Ok(all) = bot.clone().all_in_way_2(map.clone(), dir) else {
            continue;
        };
        // println!("{}, {}", bot.x, bot.y);
        for i in all {
            let p = &mut map[i];
            p.x = p.x.checked_add_signed(delta_x).unwrap();
            p.other_x = p.other_x.checked_add_signed(delta_x).unwrap();
            p.y = p.y.checked_add_signed(delta_y).unwrap();
        }
        bot.x = bot.x.checked_add_signed(delta_x).unwrap();
        bot.y = bot.y.checked_add_signed(delta_y).unwrap();
        // for y in 0..10 {
        //     for x in 0..20 {
        //         if bot.x == x && bot.y == y {
        //             print!("@");
        //             continue;
        //         }
        //         if let Some(found) = map.iter().find(|p| p.x == x && p.y == y) {
        //             let obj = match found.obj {
        //                 Obj::Box => "[",
        //                 Obj::Wall => "#",
        //             };
        //             print!("{obj}");
        //         } else if let Some(found) = map.iter().find(|p| p.other_x == x && p.y == y) {
        //             let obj = match found.obj {
        //                 Obj::Box => "]",
        //                 Obj::Wall => "#",
        //             };
        //             print!("{obj}");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
    }

    let res = map
        .iter()
        .filter(|p| p.obj == Obj::Box)
        .fold(0, |acc, box_found| acc + 100 * box_found.y + box_found.x);
    res.to_string().to_owned()
}

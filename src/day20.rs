use std::collections::{HashMap, HashSet, VecDeque};

const IMPROVEMENT: u32 = 100;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
    wall: bool,
}

fn min_neighbor<'a>(
    dist: &HashMap<Point, Option<u32>>,
    map: &'a [Vec<Point>],
    point: &'a Point,
    map_x: u32,
    map_y: u32,
) -> Option<&'a Point> {
    let (x, y) = (point.x as usize, point.y as usize);
    let mut min = dist[point];
    let mut min_point = None;
    if y != 0 {
        let above = &map[y - 1][x];
        if dist[above].is_some() && (min.is_none() || dist[above] < min) {
            min_point = Some(above);
            min = dist[above];
        } else {
            min = dist[above];
        }
    }
    if point.y != map_y - 1 {
        let below = &map[y + 1][x];
        if dist[below].is_some() && (min.is_none() || dist[below] < min) {
            min_point = Some(below);
            min = dist[below];
        } else {
            min = dist[below];
        }
    }
    if x != 0 {
        let left = &map[y][x - 1];
        if dist[left].is_some() && (min.is_none() || dist[left] < min) {
            min_point = Some(left);
            min = dist[left];
        } else {
            min = dist[left];
        }
    }
    if point.x != map_x - 1 {
        let right = &map[y][x + 1];
        if dist[right].is_some() && (min.is_none() || dist[right] < min) {
            min_point = Some(right);
        }
    }
    min_point
}

fn max_neighbor<'a>(
    dist: &HashMap<Point, Option<u32>>,
    map: &'a [Vec<Point>],
    point: &'a Point,
    map_x: u32,
    map_y: u32,
) -> Option<&'a Point> {
    let (x, y) = (point.x as usize, point.y as usize);
    let mut max = dist[point];
    let mut max_point = None;
    if y != 0 {
        let above = &map[y - 1][x];
        if dist[above] > max {
            max_point = Some(above);
            max = dist[above];
        } else {
            max = dist[above];
        }
    }
    if point.y != map_y - 1 {
        let below = &map[y + 1][x];
        if dist[below] > max {
            max_point = Some(below);
            max = dist[below];
        } else {
            max = dist[below];
        }
    }
    if x != 0 {
        let left = &map[y][x - 1];
        if dist[left] > max {
            max_point = Some(left);
            max = dist[left];
        } else {
            max = dist[left];
        }
    }
    if point.x != map_x - 1 {
        let right = &map[y][x + 1];
        if dist[right] > max {
            max_point = Some(right);
        }
    }
    max_point
}

fn dijkstra(
    map: Vec<Vec<Point>>,
    start: Point,
    end: Point,
    map_x: u32,
    map_y: u32,
) -> HashMap<Point, Option<u32>> {
    let mut queue = VecDeque::new();
    let mut dist: HashMap<Point, Option<u32>> = HashMap::new();
    for &point in map.iter().flatten() {
        dist.insert(point, None);
    }
    let mut visited = HashSet::new();

    queue.push_back(start);
    *dist.get_mut(&start).unwrap() = Some(0);
    while let Some(u) = queue.pop_front() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);

        let (x, y) = (u.x as usize, u.y as usize);

        if u.x != 0 && !map[y][x - 1].wall {
            update_dist(&mut dist, visited.clone(), u, map[y][x - 1], &mut queue);
        }
        if u.x != map_x - 1 && !map[y][x + 1].wall {
            update_dist(&mut dist, visited.clone(), u, map[y][x + 1], &mut queue);
        }
        if u.y != 0 && !map[y - 1][x].wall {
            update_dist(&mut dist, visited.clone(), u, map[y - 1][x], &mut queue);
        }
        if u.y != map_y - 1 && !map[y + 1][x].wall {
            update_dist(&mut dist, visited.clone(), u, map[y + 1][x], &mut queue);
        }
        if u == end {
            break;
        }
    }
    dist
}

fn update_dist(
    dist: &mut HashMap<Point, Option<u32>>,
    visited: HashSet<Point>,
    u: Point,
    neighbor: Point,
    queue: &mut VecDeque<Point>,
) {
    if !visited.contains(&neighbor) && dist[&neighbor].is_none_or(|d| dist[&u].unwrap() + 1 < d) {
        *dist.get_mut(&neighbor).unwrap() = Some(dist[&u].unwrap() + 1);
        queue.push_back(neighbor);
    }
}

pub fn part1(input: &str) -> String {
    let mut start = None;
    let mut end = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let mut point = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        wall: false,
                    };
                    match c {
                        '#' => point.wall = true,
                        'S' => start = Some(point),
                        'E' => end = Some(point),
                        '.' => {}
                        _ => panic!("Invalid symbol in map: {c}"),
                    }
                    point
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = start.expect("Could not find start");
    let end = end.expect("Could not find end");

    let map_x = map.len().try_into().unwrap();
    let map_y = map[0].len().try_into().unwrap();

    let regular_dist = dijkstra(map.clone(), start, end, map_x, map_y);

    let mut res = 0;

    for wall in map.iter().flatten().filter(|p| p.wall) {
        let Some(min) = min_neighbor(&regular_dist, &map, wall, map_x, map_y) else {
            continue;
        };
        let Some(max) = max_neighbor(&regular_dist, &map, wall, map_x, map_y) else {
            continue;
        };
        // println!("{:?}, {:?}", regular_dist[min], regular_dist[max]);
        if regular_dist[min].unwrap() + 2 <= regular_dist[max].unwrap()
            && regular_dist[max].unwrap() - regular_dist[min].unwrap() - 2 >= IMPROVEMENT
        // && map_improves(map.clone(), &regular_dist, *wall, start, &end, map_x, map_y)
        {
            println!(
                "{}",
                regular_dist[max].unwrap() - regular_dist[min].unwrap() - 2
            );
            res += 1;
        }
    }

    res.to_string().to_owned()
}

fn neighbors<'a>(
    map: &'a [Vec<Point>],
    pt: &'a Point,
    map_x: u32,
    map_y: u32,
) -> HashSet<&'a Point> {
    let mut all = HashSet::new();
    let (x, y) = (pt.x as usize, pt.y as usize);
    if pt.x != 0 && !map[y][x - 1].wall {
        all.insert(&map[y][x - 1]);
    }
    if pt.x != map_x - 1 && !map[y][x + 1].wall {
        all.insert(&map[y][x + 1]);
    }
    if pt.y != 0 && !map[y - 1][x].wall {
        all.insert(&map[y - 1][x]);
    }
    if pt.y != map_y - 1 && !map[y + 1][x].wall {
        all.insert(&map[y + 1][x]);
    }
    all
}

pub fn part2(input: &str) -> String {
    let mut start = None;
    let mut end = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let mut point = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        wall: false,
                    };
                    match c {
                        '#' => point.wall = true,
                        'S' => start = Some(point),
                        'E' => end = Some(point),
                        '.' => {}
                        _ => panic!("Invalid symbol in map: {c}"),
                    }
                    point
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = start.expect("Could not find start");
    let end = end.expect("Could not find end");

    let map_x = map.len().try_into().unwrap();
    let map_y = map[0].len().try_into().unwrap();

    let regular_dist = dijkstra(map.clone(), start, end, map_x, map_y);

    let mut res = 0;
    let mut seen: HashMap<Point, HashSet<Point>> = HashMap::new();

    for wall in map.iter().flatten().filter(|p| p.wall) {
        let neighs = neighbors(&map, wall, map_x, map_y);
        // let mut visited = HashSet::new();
        // let a = neighbor_region(&map, wall, map_x, map_y, 19, &mut visited);
        // for y in 0..map_y {
        //     for x in 0..map_x {
        //         print!(
        //             "{}",
        //             if x == wall.x && y == wall.y {
        //                 "a"
        //             } else if a.contains(&map[y as usize][x as usize]) {
        //                 "#"
        //             } else {
        //                 "."
        //             }
        //         );
        //     }
        //     println!();
        // }
        for neighbor in neighs {
            for not_wall in map.iter().flatten().filter(|p| !p.wall) {
                if let Some(seen_min) = seen.get(neighbor) {
                    if seen_min.contains(not_wall) {
                        continue;
                    }
                } else {
                    seen.insert(*neighbor, HashSet::new());
                }
                seen.get_mut(neighbor).unwrap().insert(*not_wall);
                let pt_dst = neighbor.x.abs_diff(not_wall.x) + neighbor.y.abs_diff(not_wall.y);
                if pt_dst > 20 || pt_dst == 0 {
                    continue;
                }
                // println!("{:?}, {:?}", min, neighbor);
                // println!(
                //     "{:?}, {:?}",
                //     regular_dist[min].unwrap(),
                //     regular_dist[neighbor].unwrap()
                // );
                if regular_dist[neighbor].unwrap() + pt_dst <= regular_dist[not_wall].unwrap()
                    && regular_dist[not_wall].unwrap() - regular_dist[neighbor].unwrap() - pt_dst
                        >= IMPROVEMENT
                {
                    // println!(
                    //     "{}",
                    //     regular_dist[neighbor].unwrap() - regular_dist[min].unwrap() - pt_dst
                    // );
                    res += 1;
                }
            }
        }
        // println!("{:?}, {:?}", regular_dist[min], regular_dist[max]);
        // println!("{}", a.0);
        // println!("{:?}, {:?}", min, max);
        // if regular_dist[min].unwrap() + 19 - a.0 <= regular_dist[max].unwrap()
        //     && regular_dist[max].unwrap() - regular_dist[min].unwrap() + 19 - a.0 >= IMPROVEMENT
        // {
        //     println!(
        //         "{}",
        //         regular_dist[max].unwrap() - regular_dist[min].unwrap() + 19 - a.0
        //     );
        //     res += 1;
        // }
    }

    res.to_string().to_owned()
}

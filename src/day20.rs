use std::collections::{HashMap, HashSet, VecDeque};

const IMPROVEMENT: u32 = 50;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
    wall: bool,
}

fn map_improves(
    mut map: Vec<Vec<Point>>,
    regular_dist: &HashMap<Point, Option<u32>>,
    wall: Point,
    start: Point,
    end: &Point,
    map_x: u32,
    map_y: u32,
) -> bool {
    map[wall.y as usize][wall.x as usize].wall = false;
    let other_dist = dijkstra(map, start, *end, map_x, map_y);
    if other_dist[end].unwrap() + IMPROVEMENT <= regular_dist[end].unwrap() {
        println!(
            "{}, {}: {}",
            regular_dist[end].unwrap(),
            other_dist[end].unwrap(),
            regular_dist[end].unwrap() - other_dist[end].unwrap()
        );
        true
    } else {
        false
    }
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
        // else {
        //     if wall.x != 0 {
        //         let mut tmp = *wall;
        //         tmp.x -= 1;
        //         let point = &map[tmp.y as usize][tmp.x as usize];
        //         let max = max_neighbor(&regular_dist, &map, point, map_x, map_y);

        //         if max.is_some()
        //             && regular_dist[min] < regular_dist[max.unwrap()]
        //             && map_improves(
        //                 map.clone(),
        //                 &regular_dist,
        //                 *wall,
        //                 Some(*point),
        //                 start,
        //                 &end,
        //                 map_x,
        //                 map_y,
        //             )
        //         {
        //             res += 1;
        //         }
        //     }
        //     if wall.y != 0 {
        //         let mut tmp = *wall;
        //         tmp.y -= 1;
        //         let point = &map[tmp.y as usize][tmp.x as usize];
        //         let max = max_neighbor(&regular_dist, &map, point, map_x, map_y);

        //         if max.is_some()
        //             && regular_dist[min] < regular_dist[max.unwrap()]
        //             && map_improves(
        //                 map.clone(),
        //                 &regular_dist,
        //                 *wall,
        //                 Some(*point),
        //                 start,
        //                 &end,
        //                 map_x,
        //                 map_y,
        //             )
        //         {
        //             res += 1;
        //         }
        //     }
        //     if wall.x != map_x - 1 {
        //         let mut tmp = *wall;
        //         tmp.x += 1;
        //         let point = &map[tmp.y as usize][tmp.x as usize];
        //         let max = max_neighbor(&regular_dist, &map, point, map_x, map_y);

        //         if max.is_some()
        //             && regular_dist[min] < regular_dist[max.unwrap()]
        //             && map_improves(
        //                 map.clone(),
        //                 &regular_dist,
        //                 *wall,
        //                 Some(*point),
        //                 start,
        //                 &end,
        //                 map_x,
        //                 map_y,
        //             )
        //         {
        //             res += 1;
        //         }
        //     }
        //     if wall.y != map_y - 1 {
        //         let mut tmp = *wall;
        //         tmp.y += 1;
        //         let point = &map[tmp.y as usize][tmp.x as usize];
        //         let max = max_neighbor(&regular_dist, &map, point, map_x, map_y);

        //         if max.is_some()
        //             && regular_dist[min] < regular_dist[max.unwrap()]
        //             && map_improves(
        //                 map.clone(),
        //                 &regular_dist,
        //                 *wall,
        //                 Some(*point),
        //                 start,
        //                 &end,
        //                 map_x,
        //                 map_y,
        //             )
        //         {
        //             res += 1;
        //         }
        //     }
        // }
        // if wall.x != 0 && wall.x != map_x - 1 {
        //     let mut tmp_point = *wall;
        //     tmp_point.x -= 1;
        //     let point = map.iter().flatten().find(|&&p| p.equal(tmp_point)).unwrap();
        //     let mut tmp_point = *wall;
        //     tmp_point.x += 1;
        //     let point2 = map.iter().flatten().find(|&&p| p.equal(tmp_point)).unwrap();
        //     let point3 = if wall.x != map_x - 2 {
        //         let mut tmp_point = *wall;
        //         tmp_point.x += 2;
        //         map.iter().flatten().find(|&&p| p.equal(tmp_point))
        //     } else {
        //         None
        //     };
        //     if regular_dist[point].is_some() {
        //         if regular_dist[point2].is_some() {
        //             let min = min_dist(&regular_dist, point, point2);
        //             if let Some(dist) = min.1 {
        //                 if min.0 == point {
        //                     if dist + 1 < regular_dist[point2].unwrap() {
        //                         let mut new_map = map.clone();
        //                         new_map[wall.y as usize][wall.x as usize].wall = false;
        //                         let other_dist = dijkstra(new_map, start, map_x, map_y);
        //                         if other_dist[&end].unwrap() <= regular_dist[&end].unwrap() {
        //                             println!("{:?}, {:?}", regular_dist[&end], other_dist[&end]);
        //                             res += 1;
        //                         }
        //                     }
        //                 } else if min.0 == point2 && dist + 1 < regular_dist[point].unwrap() {
        //                     let mut new_map = map.clone();
        //                     new_map[wall.y as usize][wall.x as usize].wall = false;
        //                     let other_dist = dijkstra(new_map, start, map_x, map_y);
        //                     if other_dist[&end].unwrap() <= regular_dist[&end].unwrap() {
        //                         println!("{:?}, {:?}", regular_dist[&end], other_dist[&end]);
        //                         res += 1;
        //                     }
        //                 }
        //             }
        //         } else if let Some(point3) = point3 {
        //             if regular_dist[point3].is_some() {
        //                 let min = min_dist(&regular_dist, point, point3);
        //                 if let Some(dist) = min.1 {
        //                     if min.0 == point {
        //                         if dist + 2 < regular_dist[point3].unwrap() {
        //                             let mut new_map = map.clone();
        //                             new_map[wall.y as usize][wall.x as usize].wall = false;
        //                             new_map[point2.y as usize][point2.x as usize].wall = false;
        //                             let other_dist = dijkstra(new_map, start, map_x, map_y);
        //                             if other_dist[&end].unwrap() <= regular_dist[&end].unwrap() {
        //                                 println!(
        //                                     "{:?}, {:?}",
        //                                     regular_dist[&end], other_dist[&end]
        //                                 );
        //                                 res += 1;
        //                             }
        //                         }
        //                     } else if min.0 == point2 && dist + 2 < regular_dist[point].unwrap() {
        //                         let mut new_map = map.clone();
        //                         new_map[wall.y as usize][wall.x as usize].wall = false;
        //                         new_map[point2.y as usize][point2.x as usize].wall = false;
        //                         let other_dist = dijkstra(new_map, start, map_x, map_y);
        //                         if other_dist[&end].unwrap() <= regular_dist[&end].unwrap() {
        //                             println!("{:?}, {:?}", regular_dist[&end], other_dist[&end]);
        //                             res += 1;
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        // if wall.y != 0 && wall.y != map_y - 1 {
        //     let mut tmp_point = *wall;
        //     tmp_point.y -= 1;
        //     let point = map.iter().flatten().find(|&&p| p.equal(tmp_point)).unwrap();
        //     let mut tmp_point = *wall;
        //     tmp_point.y += 1;
        //     let point2 = map.iter().flatten().find(|&&p| p.equal(tmp_point)).unwrap();
        //     let point3 = if wall.y != map_y - 2 {
        //         let mut tmp_point = *wall;
        //         tmp_point.y += 2;
        //         map.iter().flatten().find(|&&p| p.equal(tmp_point))
        //     } else {
        //         None
        //     };
        //     if regular_dist[point].is_some() {
        //         if regular_dist[point2].is_some() {
        //             let min = min_dist(&regular_dist, point, point2);
        //             if let Some(dist) = min.1 {
        //                 if min.0 == point {
        //                     if dist + 1 < regular_dist[point2].unwrap() {
        //                         let mut new_map = map.clone();
        //                         new_map[wall.y as usize][wall.x as usize].wall = false;
        //                         if dijkstra(new_map, start, map_x, map_y)[&end].unwrap() + 100
        //                             <= regular_dist[&end].unwrap()
        //                         {
        //                             res += 2;
        //                         }
        //                     }
        //                 } else if min.0 == point2 && dist + 1 < regular_dist[point].unwrap() {
        //                     let mut new_map = map.clone();
        //                     new_map[wall.y as usize][wall.x as usize].wall = false;
        //                     if dijkstra(new_map, start, map_x, map_y)[&end] < regular_dist[&end] {
        //                         res += 2;
        //                     }
        //                 }
        //             }
        //         } else if let Some(point3) = point3 {
        //             if regular_dist[point3].is_some() {
        //                 let min = min_dist(&regular_dist, point, point3);
        //                 if let Some(dist) = min.1 {
        //                     if min.0 == point {
        //                         if dist + 2 < regular_dist[point3].unwrap() {
        //                             let mut new_map = map.clone();
        //                             new_map[wall.y as usize][wall.x as usize].wall = false;
        //                             new_map[point2.y as usize][point2.x as usize].wall = false;
        //                             if dijkstra(new_map, start, map_x, map_y)[&end].unwrap() + 100
        //                                 <= regular_dist[&end].unwrap()
        //                             {
        //                                 res += 1;
        //                             }
        //                         }
        //                     } else if min.0 == point2 && dist + 1 < regular_dist[point].unwrap() {
        //                         let mut new_map = map.clone();
        //                         new_map[wall.y as usize][wall.x as usize].wall = false;
        //                         new_map[point2.y as usize][point2.x as usize].wall = false;
        //                         if dijkstra(new_map, start, map_x, map_y)[&end] < regular_dist[&end]
        //                         {
        //                             res += 1;
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    res.to_string().to_owned()
}

fn explore<'a>(
    dist: &HashMap<Point, Option<u32>>,
    map: &'a [Vec<Point>],
    new_point: &'a Point,
    map_x: u32,
    map_y: u32,
    region_size: u32,
    visited: &mut HashSet<Point>,
    max: &mut (u32, Option<&'a Point>),
) {
    let neighbor1 = max_neighbor(dist, map, new_point, map_x, map_y);
    let neighbor2 =
        max_neighbor_region(dist, map, new_point, map_x, map_y, region_size - 1, visited);
    if neighbor1.is_some() || neighbor2.1.is_some() {
        if neighbor1.is_none() {
            if max.1.is_some() {
                if dist[max.1.unwrap()] < dist[neighbor2.1.unwrap()] {
                    *max = neighbor2;
                }
            } else {
                *max = neighbor2;
            }
        } else if neighbor2.1.is_none() {
            if max.1.is_some() {
                if dist[max.1.unwrap()] < dist[neighbor1.unwrap()] {
                    *max = (region_size, neighbor1);
                }
            } else {
                *max = (region_size, neighbor1);
            }
        } else {
            let loc_max = if dist[neighbor1.unwrap()] < dist[neighbor2.1.unwrap()] {
                neighbor2
            } else {
                (region_size, neighbor1)
            };
            if max.1.is_some() {
                if dist[max.1.unwrap()] < dist[loc_max.1.unwrap()] {
                    *max = loc_max;
                }
            } else {
                *max = loc_max;
            }
        }
    }
}

fn max_neighbor_region<'a>(
    dist: &HashMap<Point, Option<u32>>,
    map: &'a [Vec<Point>],
    point: &'a Point,
    map_x: u32,
    map_y: u32,
    region_size: u32,
    visited: &mut HashSet<Point>,
) -> (u32, Option<&'a Point>) {
    if region_size == 0 || visited.contains(point) {
        return (0, None);
    }
    visited.insert(*point);
    let mut max = (0, None);
    if point.x != map_x - 1 {
        let mut tmp = *point;
        tmp.x += 1;
        let new_point = &map[tmp.y as usize][tmp.x as usize];
        explore(
            dist,
            map,
            new_point,
            map_x,
            map_y,
            region_size,
            visited,
            &mut max,
        );
    }
    if point.y != map_y - 1 {
        let mut tmp = *point;
        tmp.y += 1;
        let new_point = &map[tmp.y as usize][tmp.x as usize];
        explore(
            dist,
            map,
            new_point,
            map_x,
            map_y,
            region_size,
            visited,
            &mut max,
        );
    }
    if point.x != 0 {
        let mut tmp = *point;
        tmp.x -= 1;
        let new_point = &map[tmp.y as usize][tmp.x as usize];
        explore(
            dist,
            map,
            new_point,
            map_x,
            map_y,
            region_size,
            visited,
            &mut max,
        );
    }
    if point.y != 0 {
        let mut tmp = *point;
        tmp.y -= 1;
        let new_point = &map[tmp.y as usize][tmp.x as usize];
        explore(
            dist,
            map,
            new_point,
            map_x,
            map_y,
            region_size,
            visited,
            &mut max,
        );
    }
    max
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

    for wall in map.iter().flatten().filter(|p| p.wall) {
        let Some(min) = min_neighbor(&regular_dist, &map, wall, map_x, map_y) else {
            continue;
        };
        let mut visited = HashSet::new();
        let a = max_neighbor_region(&regular_dist, &map, wall, map_x, map_y, 20, &mut visited);
        let Some(max) = a.1 else {
            continue;
        };
        println!("{:?}, {:?}", regular_dist[min], regular_dist[max]);
        println!("{}", a.0);
        println!("{:?}, {:?}", min, max);
        if regular_dist[min].unwrap() + 20 - a.0 <= regular_dist[max].unwrap()
            && regular_dist[max].unwrap() - regular_dist[min].unwrap() + 20 - a.0 >= IMPROVEMENT
        {
            println!(
                "{}",
                regular_dist[max].unwrap() - regular_dist[min].unwrap() + 20 - a.0
            );
            res += 1;
        }
    }

    res.to_string().to_owned()
}

use std::collections::{HashMap, HashSet, VecDeque};

const MAP_SIZE: u32 = 70;
const NUM_BYTES: u32 = 1024;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
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
    let map = (0..=MAP_SIZE)
        .map(|y| (0..=MAP_SIZE).map(|x| Point { x, y }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map[0][0];
    let ignore = input
        .lines()
        .take(NUM_BYTES as usize)
        .map(|line| {
            let poses_str = line.split_once(",").unwrap();
            let poses = (
                poses_str.0.parse::<u32>().unwrap(),
                poses_str.1.parse::<u32>().unwrap(),
            );
            Point {
                x: poses.0,
                y: poses.1,
            }
        })
        .collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    let mut dist: HashMap<Point, Option<u32>> = HashMap::new();
    for &point in map.iter().flatten() {
        dist.insert(point, None);
    }
    let mut visited = HashSet::new();

    queue.push_back(Point { x: 0, y: 0 });
    *dist.get_mut(&start).unwrap() = Some(0);
    while let Some(u) = queue.pop_front() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);

        let (x, y) = (u.x as usize, u.y as usize);

        if u.x != 0 && !ignore.iter().any(|p| p.x == u.x - 1 && p.y == u.y) {
            update_dist(&mut dist, visited.clone(), u, map[y][x - 1], &mut queue);
        }
        if u.x != MAP_SIZE && !ignore.iter().any(|p| p.x == u.x + 1 && p.y == u.y) {
            update_dist(&mut dist, visited.clone(), u, map[y][x + 1], &mut queue);
        }
        if u.y != 0 && !ignore.iter().any(|p| p.x == u.x && p.y == u.y - 1) {
            update_dist(&mut dist, visited.clone(), u, map[y - 1][x], &mut queue);
        }
        if u.y != MAP_SIZE && !ignore.iter().any(|p| p.x == u.x && p.y == u.y + 1) {
            update_dist(&mut dist, visited.clone(), u, map[y + 1][x], &mut queue);
        }
    }
    dist[&map[MAP_SIZE as usize][MAP_SIZE as usize]]
        .expect("No path to destination found")
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let map = (0..=MAP_SIZE)
        .map(|y| (0..=MAP_SIZE).map(|x| Point { x, y }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map[0][0];
    let mut first_byte = None;
    for i in (NUM_BYTES as usize)..=input.lines().count() {
        let ignore = input
            .lines()
            .take(i)
            .map(|line| {
                let poses_str = line.split_once(",").unwrap();
                let poses = (
                    poses_str.0.parse::<u32>().unwrap(),
                    poses_str.1.parse::<u32>().unwrap(),
                );
                Point {
                    x: poses.0,
                    y: poses.1,
                }
            })
            .collect::<Vec<_>>();
        let mut queue = VecDeque::new();
        let mut dist: HashMap<Point, Option<u32>> = HashMap::new();
        for &point in map.iter().flatten() {
            dist.insert(point, None);
        }
        let mut visited = HashSet::new();

        queue.push_back(Point { x: 0, y: 0 });
        *dist.get_mut(&start).unwrap() = Some(0);
        while let Some(u) = queue.pop_front() {
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);

            let (x, y) = (u.x as usize, u.y as usize);

            if u.x != 0 && !ignore.iter().any(|p| p.x == u.x - 1 && p.y == u.y) {
                update_dist(&mut dist, visited.clone(), u, map[y][x - 1], &mut queue);
            }
            if u.x != MAP_SIZE && !ignore.iter().any(|p| p.x == u.x + 1 && p.y == u.y) {
                update_dist(&mut dist, visited.clone(), u, map[y][x + 1], &mut queue);
            }
            if u.y != 0 && !ignore.iter().any(|p| p.x == u.x && p.y == u.y - 1) {
                update_dist(&mut dist, visited.clone(), u, map[y - 1][x], &mut queue);
            }
            if u.y != MAP_SIZE && !ignore.iter().any(|p| p.x == u.x && p.y == u.y + 1) {
                update_dist(&mut dist, visited.clone(), u, map[y + 1][x], &mut queue);
            }
        }
        if !visited.contains(&map[MAP_SIZE as usize][MAP_SIZE as usize]) {
            first_byte = Some(i - 1);
            break;
        }
    }
    input
        .lines()
        .nth(first_byte.expect("Did not find index of point that makes destination unreachable"))
        .expect("Did not find point that makes destination unreachable")
        .to_owned()
}

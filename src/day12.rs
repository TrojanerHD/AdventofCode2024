use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y == other.y && self.x == other.x {
            Ordering::Equal
        } else if self.y < other.y || (self.y == other.y && self.x < other.x) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Region {
    points: Vec<Point>,
    letter: char,
}

pub fn part1(input: &str) -> String {
    let mut regions: Vec<Region> = Vec::new();
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (y, line) in map.iter().enumerate() {
        for (x, letter) in line.iter().enumerate() {
            let x: u32 = x.try_into().unwrap();
            let y: u32 = y.try_into().unwrap();
            let mut matching_regions = regions
                .iter_mut()
                .filter(|region| {
                    region.letter == *letter
                        && region.points.iter().any(|point| {
                            if point.x == x {
                                return y != 0 && point.y == y - 1;
                            }
                            if point.y == y {
                                return x != 0 && point.x == x - 1;
                            }
                            false
                        })
                })
                .collect::<Vec<_>>();
            if matching_regions.is_empty() {
                regions.push(Region {
                    points: vec![Point { x, y }],
                    letter: *letter,
                });
            } else if matching_regions.len() == 1 {
                matching_regions[0].points.push(Point { x, y });
            } else {
                let mut super_region = vec![Point { x, y }];
                for region in matching_regions.iter_mut() {
                    super_region.extend(region.points.clone());
                    region.points = Vec::new();
                }
                regions.retain(|region| !region.points.is_empty());
                regions.push(Region {
                    points: super_region,
                    letter: *letter,
                });
            }
        }
    }

    let mut res = 0;
    for region in regions.iter() {
        let area = region.points.len();
        let mut non_neighbors = 0;
        for point in region.points.iter() {
            if point.x == 0 || map[point.y as usize][(point.x - 1) as usize] != region.letter {
                non_neighbors += 1;
            }
            if point.x as usize == map[0].len() - 1
                || map[point.y as usize][(point.x + 1) as usize] != region.letter
            {
                non_neighbors += 1;
            }
            if point.y == 0 || map[(point.y - 1) as usize][point.x as usize] != region.letter {
                non_neighbors += 1;
            }
            if point.y as usize == map.len() - 1
                || map[(point.y + 1) as usize][point.x as usize] != region.letter
            {
                non_neighbors += 1;
            }
        }
        res += area * non_neighbors;
    }
    res.to_string().to_owned()
}
pub fn part2(input: &str) -> String {
    let mut regions: Vec<Region> = Vec::new();
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (y, line) in map.iter().enumerate() {
        for (x, letter) in line.iter().enumerate() {
            let x: u32 = x.try_into().unwrap();
            let y: u32 = y.try_into().unwrap();
            let mut matching_regions = regions
                .iter_mut()
                .filter(|region| {
                    region.letter == *letter
                        && region.points.iter().any(|point| {
                            if point.x == x {
                                return y != 0 && point.y == y - 1;
                            }
                            if point.y == y {
                                return x != 0 && point.x == x - 1;
                            }
                            false
                        })
                })
                .collect::<Vec<_>>();
            if matching_regions.is_empty() {
                regions.push(Region {
                    points: vec![Point { x, y }],
                    letter: *letter,
                });
            } else if matching_regions.len() == 1 {
                matching_regions[0].points.push(Point { x, y });
            } else {
                let mut super_region = vec![Point { x, y }];
                for region in matching_regions.iter_mut() {
                    super_region.extend(region.points.clone());
                    region.points = Vec::new();
                }
                regions.retain(|region| !region.points.is_empty());
                regions.push(Region {
                    points: super_region,
                    letter: *letter,
                });
            }
        }
    }
    for region in regions.iter_mut() {
        region.points.sort_unstable();
    }

    // println!("{:?}", regions);

    let mut res = 0;
    for region in regions.iter() {
        let mut dirs: HashMap<Point, Vec<Dir>> = HashMap::new();
        let area = region.points.len();
        let mut non_neighbors = 0;
        for point in region.points.iter() {
            if point.x == 0 || map[point.y as usize][(point.x - 1) as usize] != region.letter {
                if point.y == 0
                    || dirs
                        .get(&Point {
                            x: point.x,
                            y: point.y - 1,
                        })
                        .is_none_or(|dir| !dir.contains(&Dir::Left))
                {
                    // println!("Counting {:?} left", point);
                    non_neighbors += 1;
                }
                if let Some(dir) = dirs.get_mut(point) {
                    dir.push(Dir::Left);
                } else {
                    dirs.insert(*point, vec![Dir::Left]);
                }
            }
            if point.x as usize == map[0].len() - 1
                || map[point.y as usize][(point.x + 1) as usize] != region.letter
            {
                if point.y == 0
                    || dirs
                        .get(&Point {
                            x: point.x,
                            y: point.y - 1,
                        })
                        .is_none_or(|dir| !dir.contains(&Dir::Right))
                {
                    // println!("Counting {:?} right", point);
                    non_neighbors += 1;
                }
                if let Some(dir) = dirs.get_mut(point) {
                    dir.push(Dir::Right);
                } else {
                    dirs.insert(*point, vec![Dir::Right]);
                }
            }
            if point.y == 0 || map[(point.y - 1) as usize][point.x as usize] != region.letter {
                if point.x == 0
                    || dirs
                        .get(&Point {
                            x: point.x - 1,
                            y: point.y,
                        })
                        .is_none_or(|dir| !dir.contains(&Dir::Up))
                {
                    // println!("Counting {:?} up", point);
                    non_neighbors += 1;
                }
                if let Some(dir) = dirs.get_mut(point) {
                    dir.push(Dir::Up);
                } else {
                    dirs.insert(*point, vec![Dir::Up]);
                }
            }
            if point.y as usize == map.len() - 1
                || map[(point.y + 1) as usize][point.x as usize] != region.letter
            {
                if point.x == 0
                    || dirs
                        .get(&Point {
                            x: point.x - 1,
                            y: point.y,
                        })
                        .is_none_or(|dir| !dir.contains(&Dir::Down))
                {
                    // println!("Counting {:?} down", point);
                    non_neighbors += 1;
                }
                if let Some(dir) = dirs.get_mut(point) {
                    dir.push(Dir::Down);
                } else {
                    dirs.insert(*point, vec![Dir::Down]);
                }
            }
        }
        // println!(
        //     "{} has area {area} and non_neighbors {non_neighbors}",
        //     region.letter
        // );
        // println!("With {:?}", dirs);
        res += area * non_neighbors;
    }
    res.to_string().to_owned()
}

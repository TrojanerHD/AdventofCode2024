use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn sub_signed(self, other: OffsetPoint) -> Option<Point> {
        let x = match other.x {
            Offset::Pos(x) => self.x.checked_sub(x)?,
            Offset::Neg(x) => self.x + x,
        };
        let y = match other.y {
            Offset::Pos(y) => self.y.checked_sub(y)?,
            Offset::Neg(y) => self.y + y,
        };
        Some(Point { x, y })
    }

    fn add_signed(self, other: OffsetPoint) -> Option<Point> {
        let x = match other.x {
            Offset::Pos(x) => self.x + x,
            Offset::Neg(x) => self.x.checked_sub(x)?,
        };
        let y = match other.y {
            Offset::Pos(y) => self.y + y,
            Offset::Neg(y) => self.y.checked_sub(y)?,
        };
        Some(Point { x, y })
    }
}

#[derive(Clone, Copy)]
enum Offset {
    Neg(u32),
    Pos(u32),
}

#[derive(Clone, Copy)]
struct OffsetPoint {
    x: Offset,
    y: Offset,
}

struct Antenna {
    point: Point,
    letter: char,
}

pub fn part1(input: &str) -> String {
    let points = input
        .lines()
        .enumerate()
        .flat_map(move |(y, chars)| {
            chars.chars().enumerate().filter_map(move |(x, pos)| {
                if pos != '.' {
                    Some(Antenna {
                        point: Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        letter: pos,
                    })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let max = Point {
        y: input.lines().count() as u32,
        x: input.lines().next().unwrap().chars().count() as u32,
    };

    let mut antinodes = HashSet::<Point>::new();
    for (i, antenna) in points.iter().enumerate() {
        for (j, antenna2) in points.iter().enumerate() {
            if i >= j || antenna.letter != antenna2.letter {
                continue;
            }
            let x_smaller = antenna.point.x < antenna2.point.x;
            let diff = OffsetPoint {
                x: if x_smaller {
                    Offset::Pos(antenna2.point.x - antenna.point.x)
                } else {
                    Offset::Neg(antenna.point.x - antenna2.point.x)
                },
                y: Offset::Pos(antenna2.point.y - antenna.point.y),
            };
            let top_left = antenna.point.sub_signed(diff);
            let bot_right = antenna2.point.add_signed(diff);
            if let Some(top_left) = top_left {
                if top_left.y < max.y && top_left.x < max.x {
                    antinodes.insert(top_left);
                }
            }
            if let Some(bot_right) = bot_right {
                if bot_right.y < max.y && bot_right.x < max.x {
                    antinodes.insert(bot_right);
                }
            }
        }
    }

    antinodes.len().to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let points = input
        .lines()
        .enumerate()
        .flat_map(move |(y, chars)| {
            chars.chars().enumerate().filter_map(move |(x, pos)| {
                if pos != '.' {
                    Some(Antenna {
                        point: Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        letter: pos,
                    })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let max = Point {
        y: input.lines().count() as u32,
        x: input.lines().next().unwrap().chars().count() as u32,
    };

    let mut antinodes = HashSet::<Point>::new();
    for (i, antenna) in points.iter().enumerate() {
        antinodes.insert(antenna.point);
        for (j, antenna2) in points.iter().enumerate() {
            if i >= j || antenna.letter != antenna2.letter {
                continue;
            }
            let x_smaller = antenna.point.x < antenna2.point.x;
            let diff = OffsetPoint {
                x: if x_smaller {
                    Offset::Pos(antenna2.point.x - antenna.point.x)
                } else {
                    Offset::Neg(antenna.point.x - antenna2.point.x)
                },
                y: Offset::Pos(antenna2.point.y - antenna.point.y),
            };
            let mut top_left = antenna.point.sub_signed(diff);
            while let Some(r_top_left) = top_left {
                if r_top_left.y < max.y && r_top_left.x < max.x {
                    antinodes.insert(r_top_left);
                } else {
                    break;
                }
                top_left = r_top_left.sub_signed(diff);
            }
            let mut bot_right = antenna2.point.add_signed(diff);
            while let Some(r_bot_right) = bot_right {
                if r_bot_right.y < max.y && r_bot_right.x < max.x {
                    antinodes.insert(r_bot_right);
                } else {
                    break;
                }
                bot_right = r_bot_right.add_signed(diff);
            }
        }
    }

    antinodes.len().to_string().to_owned()
}

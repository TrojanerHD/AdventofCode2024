use std::{fs::OpenOptions, io::Write};

const MAP_X: u32 = 101;
const MAP_Y: u32 = 103;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_point = Point {
            x: (self.x + rhs.x) % MAP_X as i32,
            y: (self.y + rhs.y) % MAP_Y as i32,
        };
        while new_point.x < 0 {
            new_point.x += MAP_X as i32;
        }
        while new_point.y < 0 {
            new_point.y += MAP_Y as i32;
        }
        new_point
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

struct Robot {
    pos: Point,
    mov: Point,
}

pub fn part1(input: &str) -> String {
    let robots = input.lines().map(|line| {
        let mut split = line.split_whitespace();
        let init = split.next().unwrap().split("=").last().unwrap();
        let (x_init, y_init) = init.split_once(",").unwrap();
        let init_pnt = Point {
            x: x_init.parse::<i32>().unwrap(),
            y: y_init.parse::<i32>().unwrap(),
        };
        let mov = split.next().unwrap().split("=").last().unwrap();
        let (x_mov, y_mov) = mov.split_once(",").unwrap();
        let mov_pnt = Point {
            x: x_mov.parse::<i32>().unwrap(),
            y: y_mov.parse::<i32>().unwrap(),
        };
        Robot {
            pos: init_pnt,
            mov: mov_pnt,
        }
    });
    let mut quadrants: Vec<u32> = vec![0; 4];
    for mut robot in robots {
        robot.pos += Point {
            x: robot.mov.x * 100,
            y: robot.mov.y * 100,
        };
        let (x, y): (u32, u32) = (robot.pos.x as u32, robot.pos.y as u32);
        // println!("Final pos: {x}, {y}");
        if x < MAP_X / 2 && y < MAP_Y / 2 {
            quadrants[0] += 1;
        } else if x > MAP_X / 2 && y < MAP_Y / 2 {
            quadrants[1] += 1;
        } else if x < MAP_X / 2 && y > MAP_Y / 2 {
            quadrants[2] += 1;
        } else if x > MAP_X / 2 && y > MAP_Y / 2 {
            quadrants[3] += 1
        }
    }
    quadrants.iter().product::<u32>().to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut robots = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let init = split.next().unwrap().split("=").last().unwrap();
            let (x_init, y_init) = init.split_once(",").unwrap();
            let init_pnt = Point {
                x: x_init.parse::<i32>().unwrap(),
                y: y_init.parse::<i32>().unwrap(),
            };
            let mov = split.next().unwrap().split("=").last().unwrap();
            let (x_mov, y_mov) = mov.split_once(",").unwrap();
            let mov_pnt = Point {
                x: x_mov.parse::<i32>().unwrap(),
                y: y_mov.parse::<i32>().unwrap(),
            };
            Robot {
                pos: init_pnt,
                mov: mov_pnt,
            }
        })
        .collect::<Vec<_>>();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("2024-14.txt")
        .unwrap();
    let mut maps = "".to_string();
    for i in 0..10000 {
        for robot in robots.iter_mut() {
            robot.pos += Point {
                x: robot.mov.x,
                y: robot.mov.y,
            };
        }
        // 62 only works for my input. To find the correct offset for your input,
        // comment out the following three lines and look for the first iteration
        // where the points are mostly distributed in the middle of the map, vertically
        // and subtract 1 (because i starts at 0).
        // The 103 is suspiciously MAP_Y, but to be sure, look when the next iteration
        // occurs that has the similar pattern to my 63 (i+1) iteration and subtract the iteration
        // values. That will be your modulo.
        if i < 62 || (i - 62) % 103 != 0 {
            continue;
        }
        println!("Iteration {}", i + 1);
        let mut map = format!("Iteration {}\n", i + 1).to_string();
        for y in 0..MAP_Y {
            for x in 0..MAP_X {
                let num_robots = robots
                    .iter()
                    .filter(|robot| robot.pos.x as u32 == x && robot.pos.y as u32 == y)
                    .collect::<Vec<_>>()
                    .len();
                map += match num_robots {
                    0 => ".",
                    _ => "#",
                };
            }
            map += "\n";
        }
        maps += &map;
    }
    if let Err(e) = file.write_all(maps.as_bytes()) {
        eprintln!("Could not write to file: {}", e);
    };
    "Check the file 2024-14.txt in the root of the project for an iteration with a tree".to_owned()
}

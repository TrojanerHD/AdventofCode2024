use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct NumPoint {
    point: Point,
    val: u32,
}

impl NumPoint {
    fn new(x: u32, y: u32, val: u32) -> Self {
        Self {
            point: Point { x, y },
            val,
        }
    }
    fn move_one_prio_x<'a, I>(&'a self, other: Point, mut all: I) -> (Dir, &'a Self)
    where
        I: Iterator<Item = &'a NumPoint>,
    {
        if self.point.x != other.x
            && !(self.point.x > other.x && self.point.x == 1 && self.point.y == 3)
        {
            if self.point.x < other.x {
                (
                    Dir::Right,
                    all.find(|p| p.point.x == self.point.x + 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Left,
                    all.find(|p| p.point.x == self.point.x - 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            }
        } else if self.point.y != other.y {
            if self.point.y < other.y {
                (
                    Dir::Down,
                    all.find(|p| p.point.y == self.point.y + 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Up,
                    all.find(|p| p.point.y == self.point.y - 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            }
        } else {
            (Dir::Activate, self)
        }
    }
    fn move_one_prio_y<'a, I>(&'a self, other: Point, mut all: I) -> (Dir, &'a Self)
    where
        I: Iterator<Item = &'a NumPoint>,
    {
        if self.point.y != other.y
            && !(self.point.y < other.y && self.point.x == 0 && self.point.y == 2)
        {
            if self.point.y < other.y {
                (
                    Dir::Down,
                    all.find(|p| p.point.y == self.point.y + 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Up,
                    all.find(|p| p.point.y == self.point.y - 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            }
        } else if self.point.x != other.x {
            if self.point.x < other.x {
                (
                    Dir::Right,
                    all.find(|p| p.point.x == self.point.x + 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Left,
                    all.find(|p| p.point.x == self.point.x - 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            }
        } else {
            (Dir::Activate, self)
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct DirPoint {
    point: Point,
    dir: Dir,
}

impl DirPoint {
    fn new(x: u32, y: u32, dir: Dir) -> Self {
        Self {
            point: Point { x, y },
            dir,
        }
    }
    fn move_one_prio_x<'a, I>(&'a self, other: Point, mut all: I) -> (Dir, &'a Self)
    where
        I: Iterator<Item = &'a Self>,
    {
        if self.point.x != other.x
            && !(self.point.x > other.x && self.point.x == 1 && self.point.y == 0)
        {
            if self.point.x < other.x {
                (
                    Dir::Right,
                    all.find(|p| p.point.x == self.point.x + 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Left,
                    all.find(|p| p.point.x == self.point.x - 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            }
        } else if self.point.y != other.y {
            if self.point.y < other.y {
                (
                    Dir::Down,
                    all.find(|p| p.point.y == self.point.y + 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Up,
                    all.find(|p| p.point.y == self.point.y - 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            }
        } else {
            (Dir::Activate, self)
        }
    }
    fn move_one_prio_y<'a, I>(&'a self, other: Point, mut all: I) -> (Dir, &'a Self)
    where
        I: Iterator<Item = &'a Self>,
    {
        if self.point.y != other.y
            && !(self.point.y > other.y && self.point.x == 0 && self.point.y == 1)
        {
            if self.point.y < other.y {
                (
                    Dir::Down,
                    all.find(|p| p.point.y == self.point.y + 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Up,
                    all.find(|p| p.point.y == self.point.y - 1 && p.point.x == self.point.x)
                        .unwrap(),
                )
            }
        } else if self.point.x != other.x {
            if self.point.x < other.x {
                (
                    Dir::Right,
                    all.find(|p| p.point.x == self.point.x + 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            } else {
                (
                    Dir::Left,
                    all.find(|p| p.point.x == self.point.x - 1 && p.point.y == self.point.y)
                        .unwrap(),
                )
            }
        } else {
            (Dir::Activate, self)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
    Activate,
}

pub fn part1(input: &str) -> String {
    let numpad = vec![
        // First row
        NumPoint::new(0, 0, 7),
        NumPoint::new(1, 0, 8),
        NumPoint::new(2, 0, 9),
        // Second row
        NumPoint::new(0, 1, 4),
        NumPoint::new(1, 1, 5),
        NumPoint::new(2, 1, 6),
        // Third row
        NumPoint::new(0, 2, 1),
        NumPoint::new(1, 2, 2),
        NumPoint::new(2, 2, 3),
        // Fourth row
        NumPoint::new(1, 3, 0),
        // Exploit 10 as A
        NumPoint::new(2, 3, 10),
    ];
    let dirpad = vec![
        // First row
        DirPoint::new(1, 0, Dir::Up),
        DirPoint::new(2, 0, Dir::Activate),
        // Second row
        DirPoint::new(0, 1, Dir::Left),
        DirPoint::new(1, 1, Dir::Down),
        DirPoint::new(2, 1, Dir::Right),
    ];
    let mut res = 0;
    for line in input.lines() {
        let mut first_input: Vec<Dir> = Vec::new();
        let mut cur_pos = &numpad[10];
        let mut last_dir = Dir::Up;
        for c in line.chars() {
            let numpos = numpad
                .iter()
                .find(|p| {
                    let val = if c == 'A' {
                        10
                    } else {
                        c.to_digit(10).unwrap()
                    };
                    p.val == val
                })
                .unwrap();
            if ((cur_pos.point.y >= numpos.point.y && cur_pos.point.x >= numpos.point.x)
                || (cur_pos.point.y <= numpos.point.y && cur_pos.point.x >= numpos.point.x)
                || (numpos.point.y == 3 && cur_pos.point.x == 0))
                && !(numpos.point.x == 0 && cur_pos.point.y == 3)
            {
                last_dir = Dir::Left;
            } else {
                last_dir = Dir::Down;
            }
            // println!("{:?}, {:?}", cur_pos, numpos);
            // println!(
            //     "{:?}",
            //     cur_pos.point.y >= numpos.point.y && cur_pos.point.x <= numpos.point.x
            // );
            // if cur_pos.point.y == 0 && numpos.point.y != 3 {
            //     last_dir = Dir::Down;
            // }
            // if numpos.point.x != 0 && cur_pos.point.y >= numpos.point.y {
            //     last_dir = Dir::Left;
            // }
            // if cur_pos.point.y >= numpos.point.y && cur_pos.point.x <= numpos.point.y {
            //     last_dir = Dir::Up;
            // }
            while cur_pos != numpos {
                let (dir, move_point) = if last_dir == Dir::Up || last_dir == Dir::Down {
                    cur_pos.move_one_prio_y(numpos.point, numpad.iter())
                } else {
                    cur_pos.move_one_prio_x(numpos.point, numpad.iter())
                };

                first_input.push(dir);
                last_dir = if move_point.point.x == 2
                    || (move_point.point.x == 1 && move_point.point.y == 3)
                {
                    Dir::Up
                } else {
                    dir
                };
                // println!("{:?}", cur_pos);
                // println!("{:?}, {:?}", last_dir, dir);
                cur_pos = move_point;
            }
            first_input.push(Dir::Activate);
            if cur_pos.point.y != 3 {
                last_dir = Dir::Left;
            }
        }
        let mut robot_inputs = first_input;
        // println!("{:?}", robot_inputs);
        for _ in 0..2 {
            let last_inputs = robot_inputs.clone();
            robot_inputs.clear();
            let mut cur_pos = &dirpad[1];
            // println!("Inputs: {:?}", last_inputs);
            for button in last_inputs.iter() {
                let mut last_dir = if *button != Dir::Left {
                    Dir::Left
                } else {
                    Dir::Up
                };
                let dirpos = dirpad.iter().find(|p| p.dir == *button).unwrap();
                while cur_pos != dirpos {
                    let (dir, move_point) = if last_dir == Dir::Up || last_dir == Dir::Down {
                        cur_pos.move_one_prio_y(dirpos.point, dirpad.iter())
                    } else {
                        cur_pos.move_one_prio_x(dirpos.point, dirpad.iter())
                    };

                    robot_inputs.push(dir);
                    last_dir = if move_point.point.x == 2 {
                        Dir::Up
                    } else {
                        dir
                    };
                    cur_pos = move_point;
                }
                robot_inputs.push(Dir::Activate);
            }
            // println!("{:?}", robot_inputs);
        }
        // println!("{}", robot_inputs.len());
        res += robot_inputs.len() * line.replace("A", "").parse::<usize>().unwrap();
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let numpad = vec![
        // First row
        NumPoint::new(0, 0, 7),
        NumPoint::new(1, 0, 8),
        NumPoint::new(2, 0, 9),
        // Second row
        NumPoint::new(0, 1, 4),
        NumPoint::new(1, 1, 5),
        NumPoint::new(2, 1, 6),
        // Third row
        NumPoint::new(0, 2, 1),
        NumPoint::new(1, 2, 2),
        NumPoint::new(2, 2, 3),
        // Fourth row
        NumPoint::new(1, 3, 0),
        // Exploit 10 as A
        NumPoint::new(2, 3, 10),
    ];
    let dirpad = vec![
        // First row
        DirPoint::new(1, 0, Dir::Up),
        DirPoint::new(2, 0, Dir::Activate),
        // Second row
        DirPoint::new(0, 1, Dir::Left),
        DirPoint::new(1, 1, Dir::Down),
        DirPoint::new(2, 1, Dir::Right),
    ];
    let mut res = 0;
    for line in input.lines() {
        let mut activate_sequences = HashMap::new();
        let mut cur_pos = &numpad[10];
        let mut last_dir = Dir::Up;
        for c in line.chars() {
            let mut first_input: Vec<Dir> = Vec::new();
            let numpos = numpad
                .iter()
                .find(|p| {
                    let val = if c == 'A' {
                        10
                    } else {
                        c.to_digit(10).unwrap()
                    };
                    p.val == val
                })
                .unwrap();
            if ((cur_pos.point.y >= numpos.point.y && cur_pos.point.x >= numpos.point.x)
                || (cur_pos.point.y <= numpos.point.y && cur_pos.point.x >= numpos.point.x)
                || (numpos.point.y == 3 && cur_pos.point.x == 0))
                && !(numpos.point.x == 0 && cur_pos.point.y == 3)
            {
                last_dir = Dir::Left;
            } else {
                last_dir = Dir::Down;
            }
            // println!("{:?}, {:?}", cur_pos, numpos);
            // println!(
            //     "{:?}",
            //     cur_pos.point.y >= numpos.point.y && cur_pos.point.x <= numpos.point.x
            // );
            // if cur_pos.point.y == 0 && numpos.point.y != 3 {
            //     last_dir = Dir::Down;
            // }
            // if numpos.point.x != 0 && cur_pos.point.y >= numpos.point.y {
            //     last_dir = Dir::Left;
            // }
            // if cur_pos.point.y >= numpos.point.y && cur_pos.point.x <= numpos.point.y {
            //     last_dir = Dir::Up;
            // }
            while cur_pos != numpos {
                let (dir, move_point) = if last_dir == Dir::Up || last_dir == Dir::Down {
                    cur_pos.move_one_prio_y(numpos.point, numpad.iter())
                } else {
                    cur_pos.move_one_prio_x(numpos.point, numpad.iter())
                };

                first_input.push(dir);
                last_dir = if move_point.point.x == 2
                    || (move_point.point.x == 1 && move_point.point.y == 3)
                {
                    Dir::Up
                } else {
                    dir
                };
                // println!("{:?}", cur_pos);
                // println!("{:?}, {:?}", last_dir, dir);
                cur_pos = move_point;
            }
            first_input.push(Dir::Activate);
            activate_sequences.insert(first_input, 1);
            if cur_pos.point.y != 3 {
                last_dir = Dir::Left;
            }
        }
        // let mut robot_inputs = activate_sequences;
        // println!("{:?}", activate_sequences);
        // let mut cache: HashMap<Vec<Dir>, Vec<Dir>> = HashMap::new();
        for _ in 0..25 {
            let last_sequences = activate_sequences.clone();
            activate_sequences.clear();
            let mut cur_pos = &dirpad[1];
            // println!("Inputs: {:?}", last_inputs);
            // let mut cache_test = Vec::new();
            // let mut skip_indices = HashMap::new();
            // for (i, button) in last_inputs.iter().enumerate() {
            //     cache_test.push((i, *button));
            //     if *button == Dir::Activate {
            //         if let Some(found) =
            //             cache.get(&cache_test.iter().map(|c| c.1).collect::<Vec<_>>())
            //         {
            //             // println!("Cache hit");
            //             for c in cache_test.iter() {
            //                 skip_indices.insert(c.0, None);
            //             }
            //             skip_indices.insert(i, Some(found.clone()));

            //             cache_test.clear();
            //         }
            //     }
            // }
            // let mut cache_value = Vec::new();
            // let mut cache_key = Vec::new();
            for (seq, i) in last_sequences.iter().filter(|seq| *seq.1 != 0) {
                // if let Some(inputs) = skip_indices.get(&i) {
                //     if let Some(act_inputs) = inputs {
                //         robot_inputs.extend(act_inputs);
                //     }
                //     continue;
                // }
                // cache_key.push(*button);
                for button in seq {
                    let mut new_seq = Vec::new();
                    let mut last_dir = if *button != Dir::Left {
                        Dir::Left
                    } else {
                        Dir::Up
                    };
                    let dirpos = dirpad.iter().find(|p| p.dir == *button).unwrap();
                    if ((cur_pos.point.y >= dirpos.point.y && cur_pos.point.x >= dirpos.point.x)
                        || (cur_pos.point.y <= dirpos.point.y && cur_pos.point.x >= dirpos.point.x))
                        && !(dirpos.point.x == 0 && dirpos.point.y == 1)
                    {
                        last_dir = Dir::Left;
                    } else {
                        last_dir = Dir::Down;
                    }
                    while cur_pos != dirpos {
                        let (dir, move_point) = if last_dir == Dir::Up || last_dir == Dir::Down {
                            cur_pos.move_one_prio_y(dirpos.point, dirpad.iter())
                        } else {
                            cur_pos.move_one_prio_x(dirpos.point, dirpad.iter())
                        };

                        new_seq.push(dir);
                        // cache_value.push(dir);
                        last_dir = if move_point.point.x == 2 {
                            Dir::Up
                        } else {
                            dir
                        };
                        cur_pos = move_point;
                    }
                    new_seq.push(Dir::Activate);
                    if let Some(found) = activate_sequences.get_mut(&new_seq) {
                        *found += *i;
                    } else {
                        activate_sequences.insert(new_seq, *i);
                    }
                }
                // cache_value.push(Dir::Activate);
                // if *button == Dir::Activate {
                //     cache.insert(cache_key.clone(), cache_value.clone());
                //     cache_key.clear();
                //     cache_value.clear();
                // }
            }
            // cache.insert(last_inputs, robot_inputs.clone());
            // println!("{:?}", activate_sequences);
        }
        // println!(
        //     "{}",
        //     activate_sequences
        //         .iter()
        //         .map(|s| s.0.len() * s.1)
        //         .sum::<usize>()
        // );
        res += activate_sequences
            .iter()
            .map(|s| s.0.len() * s.1)
            .sum::<usize>()
            * line.replace("A", "").parse::<usize>().unwrap();
    }
    res.to_string().to_owned()
}

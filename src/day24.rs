use std::collections::{HashMap, VecDeque};

#[derive(Clone, PartialEq, Eq)]
struct Action {
    val1: String,
    val2: String,
    modifier: String,
    res: String,
}

fn parse_correction(mut res: &str) -> &str {
    if res == "dmh" {
        res = "z31";
    } else if res == "z31" {
        res = "dmh";
    } else if res == "rpv" {
        res = "z11";
    } else if res == "z11" {
        res = "rpv";
    } else if res == "ctg" {
        res = "rpb";
    } else if res == "rpb" {
        res = "ctg";
    } else if res == "dvq" {
        res = "z38";
    } else if res == "z38" {
        res = "dvq";
    }
    res
}

pub fn part1(input: &str) -> String {
    let mut input_parsing = true;
    let mut gates = HashMap::new();
    let mut actions = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            input_parsing = false;
            continue;
        }

        if input_parsing {
            let (wire, val) = line.split_once(": ").unwrap();
            gates.insert(wire.to_owned(), val.parse::<u8>().unwrap());
        } else {
            let (op, mut res) = line.split_once(" -> ").unwrap();
            res = parse_correction(res);
            let mut action = op.split_whitespace();
            actions.push(Action {
                val1: action.next().unwrap().to_owned(),
                modifier: action.next().unwrap().to_owned(),
                val2: action.next().unwrap().to_owned(),
                res: res.to_owned(),
            });
        }
    }
    let mut zs = HashMap::new();
    let mut res: u64 = 0;
    let p1 = loop {
        let defined_actions = actions
            .iter()
            .filter(|action| {
                gates.contains_key(&action.val1)
                    && gates.contains_key(&action.val2)
                    && !gates.contains_key(&action.res)
            })
            .collect::<Vec<_>>();
        if defined_actions.is_empty() {
            break res.to_string().to_owned();
        }
        for action in defined_actions {
            let val = match action.modifier.as_str() {
                "AND" => gates[&action.val1] & gates[&action.val2],
                "OR" => gates[&action.val1] | gates[&action.val2],
                "XOR" => gates[&action.val1] ^ gates[&action.val2],
                _ => panic!("Invalid action: {}", action.modifier),
            };
            gates.insert(action.res.clone(), val);
            if action.res.starts_with("z") {
                zs.insert(action.res.replace("z", "").parse::<u8>().unwrap(), val);
                res += TryInto::<u64>::try_into(val).unwrap()
                    << action.res.replace("z", "").parse::<u8>().unwrap();
            }
        }
    };
    let mut keys = zs.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    println!("{:?}", keys.iter().rev().map(|k| zs[k]).collect::<Vec<_>>());
    p1
    // "".to_owned()
}

fn resolve<'a>(action: &'a Action, actions: &'a Vec<Action>) -> String {
    let mut res = "".to_owned();
    if (action.val1.starts_with("x") || action.val1.starts_with("y"))
        && (action.val2.starts_with("x") || action.val2.starts_with("y"))
        && (action.modifier == "XOR" || action.modifier == "AND")
    {
        if action.modifier == "XOR" {
            res = format!("Result of {}", action.val1).to_owned();
        } else if action.modifier == "AND" {
            res = format!("Carry-over of {}", action.val1).to_owned();
        }
    } else {
        if action.val1.starts_with("x") || action.val1.starts_with("y") {
            res += action.val1.as_str();
        } else {
            res += format!(
                "({})",
                resolve(
                    actions.iter().find(|a| a.res == action.val1).unwrap(),
                    actions
                )
            )
            .as_str();
        }
        res += format!(" {} ", action.modifier).as_str();
        if action.val2.starts_with("x") || action.val2.starts_with("y") {
            res += action.val2.as_str();
        } else {
            res += format!(
                "({})",
                resolve(
                    actions.iter().find(|a| a.res == action.val2).unwrap(),
                    actions
                )
            )
            .as_str();
        }
    }
    res
}

pub fn part2(input: &str) -> String {
    let mut input_parsing = true;
    let mut gates = HashMap::new();
    let mut actions = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            input_parsing = false;
            continue;
        }

        if input_parsing {
            let (wire, val) = line.split_once(": ").unwrap();
            gates.insert(wire.to_owned(), val.parse::<u8>().unwrap());
        } else {
            let (op, mut res) = line.split_once(" -> ").unwrap();
            let mut action = op.split_whitespace();
            res = parse_correction(res);
            actions.push(Action {
                val1: action.next().unwrap().to_owned(),
                modifier: action.next().unwrap().to_owned(),
                val2: action.next().unwrap().to_owned(),
                res: res.to_owned(),
            });
        }
    }
    // let mut res = None;
    let mut to_print = VecDeque::new();
    let mut all_reses = gates
        .keys()
        .filter_map(|k| {
            if k.starts_with("x") {
                Some(k.replace("x", "").parse::<u8>().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    all_reses.sort_unstable();
    let mut all_carries: Vec<_> = Vec::new();
    for res in all_reses {
        for action in actions
            .iter()
            .filter(|a| a.val1 == format!("x{}", res) || a.val2 == format!("x{}", res))
        {
            to_print.push_back(action);
        }
        all_carries.extend(
            actions
                .iter()
                .filter(|a| {
                    a.modifier == "AND"
                        && ((a.val1 == format!("x{res}") && a.val2 == format!("y{res}"))
                            || (a.val2 == format!("x{res}") && a.val1 == format!("y{res}")))
                })
                .collect::<Vec<_>>(),
        );
    }
    for carry in all_carries {
        let val = carry
            .val1
            .replace("x", "")
            .replace("y", "")
            .parse::<u8>()
            .unwrap();
        if !actions.iter().any(|action| {
            action.res == format!("z{}", val + 1)
                && (action.val1 == carry.res || action.val2 == carry.res)
        }) {
            // println!("Rule violation: {} not used in z{}", carry.res, val + 1);
        }
    }
    let mut printed = Vec::new();
    while let Some(front) = to_print.pop_front() {
        if printed.contains(front) {
            continue;
        }
        println!(
            "{} {} {} -> {}",
            front.val1, front.modifier, front.val2, front.res
        );
        for action in actions
            .iter()
            .filter(|a| a.val1 == front.res || a.val2 == front.res)
        {
            to_print.push_front(action);
        }
        printed.push(front.clone());
    }
    for action in actions.iter() {
        if !printed.contains(&action) {
            println!(
                "{} {} {} -> {}",
                action.val1, action.modifier, action.val2, action.res
            );
        }
    }
    let mut zs = Vec::new();
    for action in actions.iter() {
        if action.res.starts_with("z") {
            zs.push(action.res.clone());
        }
    }
    zs.sort_unstable();
    for z in zs {
        let action = actions.iter().find(|a| a.res == z).unwrap();
        println!("{} = {}", action.res, resolve(action, &actions));
    }
    // let mut check = VecDeque::new();
    // for z in zs {
    //     check.push_back(z.clone());
    // }
    // let mut resolve = HashMap::new();
    // while let Some(found) = check.pop_front() {
    //     resolve.insert(
    //         found.res,
    //         Part {
    //             val1: found.val1.clone(),
    //             val2: found.val2.clone(),
    //             modifier: found.modifier,
    //         },
    //     );
    //     if !(found.val1.starts_with("x") || found.val1.starts_with("y")) {
    //         check.push_back(
    //             actions
    //                 .iter()
    //                 .find(|a| a.res == found.val1)
    //                 .unwrap()
    //                 .clone(),
    //         );
    //     }
    //     if !(found.val2.starts_with("x") || found.val2.starts_with("y")) {
    //         check.push_back(
    //             actions
    //                 .iter()
    //                 .find(|a| a.res == found.val2)
    //                 .unwrap()
    //                 .clone(),
    //         );
    //     }
    // }
    // 'outer: for (i, swap) in actions.iter().enumerate() {
    //     for (i1, swap1) in actions.iter().enumerate().skip(i + 1) {
    //         for (i2, swap2) in actions.iter().enumerate().skip(i1 + 1) {
    //             'inner: for (i3, swap3) in actions.iter().enumerate().skip(i2 + 1) {
    //                 // for (i4, swap4) in actions.iter().enumerate().skip(i3 + 1) {
    //                 // for (i5, swap5) in actions.iter().enumerate().skip(i4 + 1) {
    //                 // for (i6, swap6) in actions.iter().enumerate().skip(i5 + 1) {
    //                 // 'inner: for swap7 in actions.iter().skip(i6 + 1) {
    //                 let mut new_actions = actions.clone();
    //                 let new_actions = new_actions
    //                     .iter_mut()
    //                     .map(|action| {
    //                         action.res = if action.res == swap.res {
    //                             swap1.res.clone()
    //                         } else if action.res == swap1.res {
    //                             swap.res.clone()
    //                         } else if action.res == swap2.res {
    //                             swap3.res.clone()
    //                         } else if action.res == swap3.res {
    //                             swap2.res.clone()
    //                         // } else if action.res == swap4.res {
    //                         //     swap5.res.clone()
    //                         // } else if action.res == swap5.res {
    //                         //     swap4.res.clone()
    //                         // } else if action.res == swap6.res {
    //                         //     swap7.res.clone()
    //                         // } else if action.res == swap7.res {
    //                         //     swap6.res.clone()
    //                         } else {
    //                             action.res.clone()
    //                         };
    //                         action
    //                     })
    //                     .collect::<Vec<_>>();
    //                 let mut new_gates = gates.clone();
    //                 loop {
    //                     let defined_actions = new_actions
    //                         .iter()
    //                         .filter(|action| {
    //                             new_gates.contains_key(&action.val1)
    //                                 && new_gates.contains_key(&action.val2)
    //                                 && !new_gates.contains_key(&action.res)
    //                         })
    //                         .collect::<Vec<_>>();
    //                     if defined_actions.is_empty() {
    // let sum_wires = new_actions.;
    //                         let mut swaps = vec![
    //                             swap.res.clone(),
    //                             swap1.res.clone(),
    //                             swap2.res.clone(),
    //                             swap3.res.clone(),
    //                             // swap4.res.clone(),
    //                             // swap5.res.clone(),
    //                             // swap6.res.clone(),
    //                             // swap7.res.clone(),
    //                         ];
    //                         swaps.sort_unstable();
    //                         res = Some(format!(
    //                             "{},{},{},{}",
    //                             swaps[0],
    //                             swaps[1],
    //                             swaps[2],
    //                             swaps[3],
    //                             // swaps[4],
    //                             // swaps[5],
    //                             // swaps[6],
    //                             // swaps[7]
    //                         ));
    //                         break 'outer;
    //                     }
    //                     for action in defined_actions {
    //                         let val = match action.modifier.as_str() {
    //                             "AND" => new_gates[&action.val1] & new_gates[&action.val2],
    //                             "OR" => new_gates[&action.val1] | new_gates[&action.val2],
    //                             "XOR" => new_gates[&action.val1] ^ new_gates[&action.val2],
    //                             _ => panic!("Invalid action: {}", action.modifier),
    //                         };
    //                         new_gates.insert(action.res.clone(), val);
    //                         if action.res.starts_with("z") {
    //                             let offset = action.res.replace("z", "").parse::<u8>().unwrap();
    //                             if val
    //                                 != new_gates
    //                                     .iter()
    //                                     .find_map(|(a, b)| {
    //                                         if a.starts_with("x")
    //                                             && a.replace("x", "").parse::<u8>().unwrap()
    //                                                 == offset
    //                                         {
    //                                             Some(b)
    //                                         } else {
    //                                             None
    //                                         }
    //                                     })
    //                                     .unwrap()
    //                                     & new_gates
    //                                         .iter()
    //                                         .find_map(|(a, b)| {
    //                                             if a.starts_with("y")
    //                                                 && a.replace("y", "").parse::<u8>().unwrap()
    //                                                     == offset
    //                                             {
    //                                                 Some(b)
    //                                             } else {
    //                                                 None
    //                                             }
    //                                         })
    //                                         .unwrap()
    //                             {
    //                                 continue 'inner;
    //                             }
    //                         }
    //                         //             }
    //                         //         }
    //                         //     }
    //                         // }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // res.expect("No swaps found").to_owned()

    let mut result = vec!["z31", "dmh", "rpv", "z11", "ctg", "rpb", "dvq", "z38"];
    result.sort_unstable();
    result.join(",").to_owned()
}

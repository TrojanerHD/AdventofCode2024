use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> String {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (pc1, pc2) = line.split_once("-").unwrap();
        if let Some(found) = connections.get_mut(&pc1) {
            found.insert(pc2);
        } else {
            let mut insert = HashSet::new();
            insert.insert(pc2);
            connections.insert(pc1, insert);
        }
        if let Some(found) = connections.get_mut(&pc2) {
            found.insert(pc1);
        } else {
            let mut insert = HashSet::new();
            insert.insert(pc1);
            connections.insert(pc2, insert);
        }
    }
    // println!("{:?}", connections);
    let mut three: HashSet<Vec<&str>> = HashSet::new();
    for (pc, connected) in connections.iter() {
        for (i, pc1) in connected.iter().enumerate() {
            'pc2: for pc2 in connected.iter().skip(i + 1) {
                if connections.get(pc1).unwrap().contains(pc2) {
                    for pcs in three.iter() {
                        let mut pcs_hash: HashSet<&str, _> = HashSet::new();
                        pcs_hash.insert(pcs[0]);
                        pcs_hash.insert(pcs[1]);
                        pcs_hash.insert(pcs[2]);

                        let mut new_pcs_hash: HashSet<&str, _> = HashSet::new();
                        new_pcs_hash.insert(*pc);
                        new_pcs_hash.insert(*pc1);
                        new_pcs_hash.insert(*pc2);
                        if pcs_hash == new_pcs_hash {
                            continue 'pc2;
                        }
                    }
                    three.insert(vec![pc, pc1, pc2]);
                }
            }
        }
    }
    // println!("{:?}", three);
    three
        .iter()
        .filter(|pcs| pcs.iter().any(|pc| pc.starts_with("t")))
        .count()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let mut all_pcs = HashSet::new();
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (pc1, pc2) = line.split_once("-").unwrap();
        all_pcs.insert(pc1);
        all_pcs.insert(pc2);

        if let Some(found) = connections.get_mut(&pc1) {
            found.insert(pc2);
        } else {
            let mut insert = HashSet::new();
            insert.insert(pc2);
            connections.insert(pc1, insert);
        }
        if let Some(found) = connections.get_mut(&pc2) {
            found.insert(pc1);
        } else {
            let mut insert = HashSet::new();
            insert.insert(pc1);
            connections.insert(pc2, insert);
        }
    }
    let mut all_cliques = Vec::new();
    for (pc, connected) in connections.iter() {
        let mut selected = HashSet::new();
        selected.insert(*pc);
        for pc1 in connected {
            if connections.get(pc1).unwrap().is_superset(&selected) {
                selected.insert(*pc1);
            }
        }
        all_cliques.push(selected);
    }
    // println!("{:?}", all_cliques);
    let mut pcs = Vec::from_iter(
        all_cliques
            .iter()
            .max_by(|&x, y| x.len().cmp(&y.len()))
            .map(|p| p.clone())
            .unwrap(),
    );
    pcs.sort_unstable();
    pcs.join(",").to_owned()
}

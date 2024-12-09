#[derive(Copy, Clone, Debug)]
struct Block {
    id: Option<u32>,
    pos: u32,
    length: u32,
}

pub fn part1(input: &str) -> String {
    let line = input.lines().next().unwrap();
    let mut free = false;
    let mut blocks = Vec::new();
    let mut pos = 0;
    let mut id = 0;
    for num in line.chars().map(|c| c as u32 - '0' as u32) {
        if !free {
            blocks.push(Block {
                id: Some(id),
                pos,
                length: num,
            });
            id += 1;
        } else {
            blocks.push(Block {
                id: None,
                pos,
                length: num,
            });
        }
        pos += num;
        free = !free;
    }

    loop {
        let (j, block) = blocks
            .iter()
            .enumerate()
            .rev()
            .find(|block| block.1.id.is_some())
            .unwrap();

        let mut copy_block = *block;

        let Some(i) = blocks
            .iter()
            .enumerate()
            .find(|block| block.1.id.is_none())
            .map(|block| block.0)
        else {
            break;
        };

        if blocks[i].length >= copy_block.length {
            copy_block.pos = blocks[i].pos;
            let old_block = blocks[i];
            blocks[i] = copy_block;
            blocks[j].id = None;
            if old_block.length != copy_block.length {
                blocks.insert(
                    i + 1,
                    Block {
                        id: None,
                        pos: copy_block.pos + copy_block.length,
                        length: old_block.length - copy_block.length,
                    },
                );
            }
            while blocks.iter().next_back().unwrap().id.is_none() {
                blocks.pop();
            }
        } else {
            let old_block = copy_block;
            copy_block.pos = blocks[i].pos;
            copy_block.length = blocks[i].length;
            blocks[i] = copy_block;
            blocks[j].length = old_block.length - copy_block.length;
        }
    }

    blocks
        .iter()
        .map(|block| {
            let mut res = 0;
            for i in block.pos..(block.pos + block.length) {
                res += i * block.id.unwrap();
            }
            res as u64
        })
        .sum::<u64>()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let line = input.lines().next().unwrap();
    let mut free = false;
    let mut blocks = Vec::new();
    let mut pos = 0;
    let mut id = 0;
    for num in line.chars().map(|c| c as u32 - '0' as u32) {
        if !free {
            blocks.push(Block {
                id: Some(id),
                pos,
                length: num,
            });
            id += 1;
        } else {
            blocks.push(Block {
                id: None,
                pos,
                length: num,
            });
        }
        pos += num;
        free = !free;
    }

    let mut skip = Vec::new();

    loop {
        let Some((j, block)) = blocks
            .iter()
            .enumerate()
            .rev()
            .find(|block| block.1.id.is_some_and(|ex_block| !skip.contains(&ex_block)))
        else {
            break;
        };

        let mut copy_block = *block;

        let Some(i) = blocks
            .iter()
            .enumerate()
            .find(|block| {
                block.1.id.is_none()
                    && block.1.length >= copy_block.length
                    && block.1.pos < copy_block.pos
            })
            .map(|block| block.0)
        else {
            skip.push(block.id.unwrap());
            continue;
        };

        copy_block.pos = blocks[i].pos;
        let old_block = blocks[i];
        blocks[i] = copy_block;
        blocks[j].id = None;
        if old_block.length != copy_block.length {
            blocks.insert(
                i + 1,
                Block {
                    id: None,
                    pos: copy_block.pos + copy_block.length,
                    length: old_block.length - copy_block.length,
                },
            );
        }
        while blocks.iter().next_back().unwrap().id.is_none() {
            blocks.pop();
        }
        skip.push(blocks[i].id.unwrap());
    }

    blocks
        .iter()
        .filter_map(|block| {
            let mut res = 0;
            for i in block.pos..(block.pos + block.length) {
                res += i * block.id?;
            }
            Some(res as u64)
        })
        .sum::<u64>()
        .to_string()
        .to_owned()
}

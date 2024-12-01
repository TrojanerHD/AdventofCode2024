pub fn part1(input: &str) -> String {
    let split = input.lines();
    let mut sorted1 = Vec::new();
    let mut sorted2 = Vec::new();
    for line in split.into_iter() {
        let tot = line.split_whitespace();
        let mut tot_new = tot
            .map(|it| it.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .into_iter();
        sorted1.push(tot_new.next().unwrap());
        sorted2.push(tot_new.next().unwrap());
    }

    sorted1.sort_unstable();
    sorted2.sort_unstable();

    sorted1
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| {
            let y = sorted2[i];
            acc + if x < y { y - x } else { x - y }
        })
        .to_string()
        .to_owned()
}
struct Seen {
    times: u32,
    number: u32,
}
pub fn part2(input: &str) -> String {
    let mut cache: Vec<Seen> = Vec::new();
    let mut side1 = Vec::new();
    let mut side2 = Vec::new();
    for line in input.lines() {
        let mut each = line.split_whitespace().map(|it| it.parse::<u32>().unwrap());
        side1.push(each.next().unwrap());
        side2.push(each.next().unwrap());
    }
    side1
        .iter()
        .fold(0, |acc, x| {
            acc + if let Some(found) = cache.iter().position(|seen| seen.number == *x) {
                x * cache[found].times
            } else {
                let times = side2
                    .clone()
                    .into_iter()
                    .filter(|it| it == x)
                    .count()
                    .try_into()
                    .unwrap();
                cache.push(Seen { number: *x, times });
                x * times
            }
        })
        .to_string()
        .to_owned()
}

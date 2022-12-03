use std::collections::HashSet;

pub fn solve() {
    let content = include_str!("../inputs/day03.txt");
    let worker: Vec<&str> = content.lines().collect();

    let process = worker
        .iter()
        .map(|line| {
            let mut char_vec = line.chars().collect::<Vec<char>>();
            let char_vec2: Vec<char> = char_vec.split_off(char_vec.len() / 2);
            let char_set2: HashSet<char> = char_vec2.into_iter().collect();
            let char_set1: HashSet<char> = char_vec.into_iter().collect();
            let inter = char_set1.intersection(&char_set2);
            inter.map(char_to_u32).sum::<u32>()
            // inter
        })
        .sum::<u32>();
    dbg!(process);

    let mut sum: u32 = 0;
    for i in (0..worker.len()).step_by(3) {
        let item1: HashSet<char> = worker[i].chars().collect();
        let item2: HashSet<char> = worker[i + 1].chars().collect();
        let item3: HashSet<char> = worker[i + 2].chars().collect();
        let inter_temp = item1
            .intersection(&item2)
            .copied()
            .collect::<HashSet<char>>();
        let inter = inter_temp.intersection(&item3);

        sum += inter.map(char_to_u32).sum::<u32>();
    }

    dbg!(sum);
}

// #[inline]
fn char_to_u32(val: &char) -> u32 {
    match val.is_ascii_lowercase() {
        true => *val as u32 - 'a' as u32 + 1,
        false => *val as u32 - 'A' as u32 + 27,
    }
}

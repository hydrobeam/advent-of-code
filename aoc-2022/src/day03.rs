use std::collections::HashSet;

pub fn solve() {
    let content = include_str!("../inputs/day03.txt");

    let process = content.lines().map(|line| {
        let mut char_vec = line.chars().collect::<Vec<char>>();
        let char_vec2: Vec<char> = char_vec.split_off(char_vec.len() / 2);
        let char_set2: HashSet<char> = char_vec2.into_iter().collect();
        let char_set1: HashSet<char> = char_vec.into_iter().collect();
        let inter = char_set1
            .intersection(&char_set2)
            .copied()
            .collect::<Vec<char>>();
        inter.iter().map(char_to_u32).sum::<u32>()
        // inter
    }).sum::<u32>();
    dbg!(process);

    let mut thing = content.lines();
    let mut sum: u32 = 0;
    loop {
        let item1: HashSet<char>;
        if let Some(val) = thing.next() {
            item1 = val.chars().collect();
        } else {
            break;
        }
        let item2: HashSet<char> = thing.next().unwrap().chars().collect();
        let item3: HashSet<char> = thing.next().unwrap().chars().collect();
        let inter_temp = item1
            .intersection(&item2)
            .copied()
            .collect::<HashSet<char>>();

        let inter = inter_temp.intersection(&item3);

        sum += inter.map(char_to_u32).sum::<u32>();
    }

    dbg!(sum);
}

fn char_to_u32(val: &char) -> u32 {
    let ten: u32 = val.to_owned().into();
    match val.is_ascii_lowercase() {
        true => ten - 'a' as u32 + 1,
        false => ten - 'A' as u32 + 27,
    }
}

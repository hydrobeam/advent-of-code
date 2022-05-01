use std::collections::{HashSet, HashMap};

pub fn solve_day06() {
    let content: &str = include_str!("../inputs/day06_input.txt");

    let mut total_counter: usize = 0;
    for group in content.split("\n\n") {
        let mut local_set: HashSet<char> = HashSet::new();

        for line in group.split('\n') {
            for chr in line.chars() {
                local_set.insert(chr);
            }
        }


        total_counter += local_set.len();
    }

    println!("Part 1 Solution: {}", total_counter);

    total_counter = 0;

    for group in content.split("\n\n") {
        //let mut local_set: HashSet<char> = HashSet::new();
        let mut local_map: HashMap<char, u32> = HashMap::new();

        let mut group_len: u32 = 0;
        for line in group.split('\n') {
            for chr in line.chars() {
                *local_map.entry(chr).or_insert(0) +=1;
            }
            group_len += 1;
        }

        local_map.drain_filter(|_k, v| *v != group_len);
        total_counter += local_map.len();
    }

    println!("Part 2 Solution: {}", total_counter);
}

use std::collections::HashSet;

pub fn solve_day09() {
    let content: Vec<u64> = include_str!("../inputs/day09_input.txt")
        .trim()
        .split('\n')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut encryption_error: u64 = 0;

    for slice in content.windows(26) {
        let mut possible_sums: HashSet<u64> = HashSet::new();

        let sumo = slice[25];

        for preamble_val in slice[0..=24].iter() {
            if possible_sums.is_empty() {
                possible_sums.insert(*preamble_val);
            } else {
                slice.iter().for_each(|x| {
                    possible_sums.insert(*x + *preamble_val);
                })
            }
        }

        match possible_sums.get(&sumo) {
            Some(_) => {}
            None => {
                encryption_error = sumo;
                break;
            }
        }
        //dbg!(&possible_sums);
    }

    println!("Solution to part1: {}", encryption_error);

    let mut begin_ind = 0;
    let mut end_ind = 1;
    // place holder
    let encryption_weakness;

    loop {
        let ret: u64 = content[begin_ind..end_ind].iter().sum();

        if ret < encryption_error {
            end_ind += 1;
        } else if ret > encryption_error {
            begin_ind += 1;
        } else {
            encryption_weakness = content[begin_ind..end_ind].iter().min().unwrap()
                + content[begin_ind..end_ind].iter().max().unwrap();
            break;
        }
    }

    println!("Solution to part 2: {}", encryption_weakness);
}

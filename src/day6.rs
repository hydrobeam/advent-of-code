use std::collections::HashMap;

#[allow(clippy::module_name_repetitions)]
pub fn solve_day6() {
    let contents = include_str!("../inputs/day6input.txt");
    let fish_timers = contents
        .split_terminator(',')
        .map(|x| x.parse::<i64>().expect(x))
        .collect::<Vec<i64>>();
    let days = 0;

    let mut hs: HashMap<i64, i64> = HashMap::new();

    // janky ass solution but hey

    // brute force the first bit up to a certain point where it's fast enough
    for elem in crack(fish_timers, days, 155) {
        *hs.entry(elem).or_insert(0) += 1;
    }

    let mut sum: i64 = 0;
    // the fishies grow predictably, so just simulate one growing
    // for the remaining days
    // then multiply however many it produces by how many times it appears
    // in the original days.

    for swag in hs.keys() {
        let ve = vec![*swag];
        sum += crack(ve, 0, 126).len() as i64 * hs.get(swag).unwrap();
    }

    dbg!(hs);
    dbg!(sum);
}

fn crack(mut vec: Vec<i64>, mut curr_days: i64, lim_days: i64) -> Vec<i64> {
    // brute forces and runs the simulation of fishy reproduction

    while curr_days < lim_days {
        let mut cnt: u32 = 0;
        for item in &mut vec {
            *item -= 1;
            if item.is_negative() {
                *item = 6;
                cnt += 1;
            }
        }
        vec.append(&mut vec![8; cnt as usize]);
        curr_days += 1;
    }
    vec
}

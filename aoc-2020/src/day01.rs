use std::fs;
use std::str::FromStr;

fn main() {
    let filename = "inputs/day12020inputs.txt";
    let wa = fs::read_to_string(filename).unwrap();
    let looper = wa
        .lines()
        .map(|x| u32::from_str(x).unwrap())
        .collect::<Vec<u32>>();

    for hm in 0..looper.len() {
        for ah in 0..looper.len() {
            for op in 0..looper.len() {
                if looper[hm] + looper[ah] + looper[op] == 1000 {
                    dbg!(looper[hm] * looper[ah] * looper[op]);
                }
            }
        }
    }
}

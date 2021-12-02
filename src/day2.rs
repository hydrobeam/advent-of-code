use std::fs;
use std::str::FromStr;

pub fn solve_day2p1() -> i32 {
    let filename = "/home/aquabeam/Documents/Github/advent2021/day2input.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut pos = [0, 0];

    let vecgang = contents.split("\n").collect::<Vec<&str>>();

    for thing in 0..(vecgang.len()) {
        let hm: Vec<&str> = vecgang[thing].split(" ").collect();
        
        match hm.as_slice() {
            ["forward", a]  => pos[0] = pos[0] + i32::from_str(a).unwrap(),
            ["up", a]  => pos[1] = pos[1] - i32::from_str(a).unwrap(),
            ["down", a]  => pos[1] = pos[1] + i32::from_str(a).unwrap(),
            _ => ()
        };
    };

    pos[0]*pos[1]
}

pub fn solve_day2p2() -> i32 {
    let filename = "/home/aquabeam/Documents/Github/advent2021/day2input.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut pos = [0, 0, 0]; // hori, depth, aim

    let vecgang = contents.split("\n").collect::<Vec<&str>>();

    for thing in 0..(vecgang.len()) {
        let hm: Vec<&str> = vecgang[thing].split(" ").collect();
        
        match hm.as_slice() {
            ["forward", a]  => {
                let val: i32 = i32::from_str(a).unwrap(); 
                pos[0] = pos[0] +val;
                pos[1] = pos[1] + val*pos[2];
            }
            ["up", a]  => pos[2] = pos[2] - i32::from_str(a).unwrap(),
            ["down", a]  => pos[2] = pos[2] + i32::from_str(a).unwrap(),
            _ => ()
        };
    };

    pos[0]*pos[1]
}
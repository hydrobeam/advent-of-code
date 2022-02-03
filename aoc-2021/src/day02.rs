use std::fs;
use std::str::FromStr;

pub fn solve_day2p1() -> i32 {
    let filename = "inputs/day2input.txt";

    let contents = fs::read_to_string(filename).unwrap();
    let vecgang = contents.split('\n');
    let mut pos = [0, 0];

    for thing in vecgang {
        let hm = thing.split_once(" ");
        // first elem: direction, second elem: value
        match hm {
            Some(("forward", a)) => pos[0] += i32::from_str(a).unwrap(),
            Some(("up", a)) => pos[1] -= i32::from_str(a).unwrap(),
            Some(("down", a)) => pos[1] += i32::from_str(a).unwrap(),
            _ => (),
        };
    }

    pos[0] * pos[1]
}

pub fn solve_day2p2() -> i32 {
    let filename = "inputs/day2input.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut pos = [0, 0, 0]; // hori, depth, aim

    let vecgang = contents.split('\n');

    for thing in vecgang {
        let hm = thing.split_once(" ");
        // println!("{:#?}", hm);

        match hm {
            Some(("forward", a)) => {
                let val: i32 = i32::from_str(a).unwrap();
                pos[0] += val;
                pos[1] += val * pos[2];
            }
            Some(("up", a)) => pos[2] -= i32::from_str(a).unwrap(),
            Some(("down", a)) => pos[2] += i32::from_str(a).unwrap(),
            _ => (),
        };
    }

    pos[0] * pos[1]
}

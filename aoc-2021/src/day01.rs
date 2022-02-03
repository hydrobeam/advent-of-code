use std::fs;
use std::str::FromStr;
// use itertools::Itertools;

pub fn solve_day1p1() -> u32 {
    let filename = "inputs/day1input.txt";
    let contents = fs::read_to_string(filename).expect("they makin me do this");
    let file = contents.lines();

    let sol = file
        .map(|x| u32::from_str(x).unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |acc, i| acc + (i[1] > i[0]) as u32);
    sol
}

pub fn solve_day1p2() {
    let filename: &str = "inputs/day1input.txt";

    let contents: String = fs::read_to_string(filename).expect("they makin me do this");
    // let vecsquad = contents
    //     .split("\n")
    //     .map(|x| u32::from_str(x).unwrap())
    //     .collect::<Vec<u32>>();

    let sol = contents
        .lines()
        .map(|x| u32::from_str(x).unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .collect::<Vec<&[u32]>>()
        .windows(2)
        .fold(0, |acc, x| {
            acc + ((x[0][0] + x[0][1] + x[0][2]) < (x[1][0] + x[1][1] + x[1][2])) as u32
        });

    dbg!(sol);
    // for thing in 3..vecsquad.len() {
    //     // println!("{}", thing);
    //     //let mruh = u32::from_str(thing).unwrap();
    //     // dbg!(vecsquad[thing]);

    //     // probs should use an array, not a vector

    //     let v1: Vec<u32> = vec![
    //         vecsquad[thing - 3],
    //         vecsquad[thing - 2],
    //         vecsquad[thing - 1],
    //     ];
    //     let v2: Vec<u32> = vec![vecsquad[thing - 2], vecsquad[thing - 1], vecsquad[thing]];

    //     if v1.iter().sum::<u32>() < v2.into_iter().sum::<u32>() {
    //         var += 1;
    //     }
    // }
    // dbg!(var);

    // hash attempt :(

    // let mut hash: HashMap<String, u32> = HashMap::new();
    // for i in 0..6 {
    //     hash.insert(format!("v{}", i), u32::from_str(vecsquad[thing-i]).unwrap());
    // }

    // dbg!(&hash);

    // if (hash.get("v0").unwrap() + hash.get("v1").unwrap() +hash.get("v2").unwrap()) > (hash.get("v3").unwrap() + hash.get("v4").unwrap() + hash.get("v5").unwrap()) {
    //     var = var + 1;
    // }
}

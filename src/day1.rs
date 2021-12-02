use std::fs;
use std::str::FromStr;

pub fn solve_day1p1() -> u32 {
    let filename = "inputs/day1input.txt";

    let contents = fs::read_to_string(filename).expect("they makin me do this");

    let vecsquad = contents.split('\n').collect::<Vec<&str>>();
    //println!("With text:\n{:#?}", vecsquad);
    let mut var: u32 = 0;

    for thing in 0..vecsquad.len() - 1 {
        // println!("{}", thing);
        //let mruh = u32::from_str(thing).unwrap();
        // dbg!(vecsquad[thing]);

        if u32::from_str(vecsquad[thing + 1]).unwrap() > u32::from_str(vecsquad[thing]).unwrap() {
            var += 1;
        }
    }
    var
}

pub fn solve_day1p2() -> u32 {
    let filename = "inputs/day1input.txt";

    let contents = fs::read_to_string(filename).expect("they makin me do this");

    let mut var: u32 = 0;

    let vecsquad = contents
        .split('\n') // input file with newlines
        .collect::<Vec<&str>>() // is a string, make it a
        .iter() // turn into a iterator so i can make everything an int
        .map(|x| u32::from_str(x).unwrap()) // make int
        .collect::<Vec<u32>>(); // return to vec

    for thing in 3..vecsquad.len() {
        // println!("{}", thing);
        //let mruh = u32::from_str(thing).unwrap();
        // dbg!(vecsquad[thing]);

        // probs should use an array, not a vector

        let v1: Vec<u32> = vec![
            vecsquad[thing - 3],
            vecsquad[thing - 2],
            vecsquad[thing - 1],
        ];
        let v2: Vec<u32> = vec![vecsquad[thing - 2], vecsquad[thing - 1], vecsquad[thing]];

        if v1.iter().sum::<u32>() < v2.into_iter().sum::<u32>() {
            var += 1;
        }

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
    var
}

use std::fs;
use std::str::FromStr;

fn main() {
    // solve_day3p2();
    let filename = "inputs/day12020inputs.txt";
    let wa = fs::read_to_string(filename).unwrap();
    // let file = wa.lines().map(|x| u32::from_str(x).unwrap()).collect::<Vec<u32>>();
    let file = wa.lines().map(|x| x.split(" "));

    let mut i = 0;
    for temp in file {
        let bemp = temp.collect::<Vec<&str>>();
        let range: Vec<u16> = bemp[0]
            .split("-")
            .map(|x| u16::from_str(x).unwrap())
            .collect();
        let chr = bemp[1].chars().next().unwrap();
        let mut pass = bemp[2].chars();

        let t1 = pass.nth((range[0] - 1) as usize).unwrap();

        let t2 = pass.nth((range[1] - range[0] - 1) as usize).unwrap();

        if t1 == chr && t2 != chr {
            i += 1;
        } else if t2 == chr && t1 != chr {
            i += 1
        }

        // let count = pass.filter(|&x| x==chr).count() as u16;
        // if count >= range[0]  && count <= range[1] {
        //     i+=1;
        // }
    }
    dbg!(i);
}

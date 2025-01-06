use std::{env, error::Error, fs::read_to_string};

use crate::solutions::*;

mod days;
mod solutions;

use days::*;

macro_rules! output_sol {
    ($day:ident, $v1:ident, $v2:ident) => {
        println!(
            r#"
Day {}
    Part 1: {}
    Part 2: {}
"#,
            $day, $v1, $v2
        )
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().expect("invalid day number passed");

    let input = read_to_string(format!("inputs/day{day}.txt"))
        .expect("Input file does not exist for corresponding day: Day{day}");

    let (p1_sol, p2_sol) = match day {
        1 => (Day01::solve_p1(&input), Day01::solve_p2(&input)),
        2 => (Day02::solve_p1(&input), Day02::solve_p2(&input)),
        3 => (Day03::solve_p1(&input), Day03::solve_p2(&input)),
        4 => (Day04::solve_p1(&input), Day04::solve_p2(&input)),
        5 => (Day05::solve_p1(&input), Day05::solve_p2(&input)),
        6 => (Day06::solve_p1(&input), Day06::solve_p2(&input)),
        7 => (Day07::solve_p1(&input), Day07::solve_p2(&input)),
        8 => (Day08::solve_p1(&input), Day08::solve_p2(&input)),
        9 => (Day09::solve_p1(&input), Day09::solve_p2(&input)),
        10 => (Day10::solve_p1(&input), Day10::solve_p2(&input)),
        11 => (Day11::solve_p1(&input), Day11::solve_p2(&input)),
        12 => (Day12::solve_p1(&input), Day12::solve_p2(&input)),
        13 => (Day13::solve_p1(&input), Day13::solve_p2(&input)),
        14 => (Day14::solve_p1(&input), Day14::solve_p2(&input)),
        15 => (Day15::solve_p1(&input), Day15::solve_p2(&input)),
        16 => (Day16::solve_p1(&input), Day16::solve_p2(&input)),
        17 => (Day17::solve_p1(&input), Day17::solve_p2(&input)),
        18 => (Day18::solve_p1(&input), Day18::solve_p2(&input)),
        19 => (Day19::solve_p1(&input), Day19::solve_p2(&input)),
        20 => (Day20::solve_p1(&input), Day20::solve_p2(&input)),
        21 => (Day21::solve_p1(&input), Day21::solve_p2(&input)),
        22 => (Day22::solve_p1(&input), Day22::solve_p2(&input)),
        23 => (Day23::solve_p1(&input), Day23::solve_p2(&input)),
        24 => (Day24::solve_p1(&input), Day24::solve_p2(&input)),
        25 => (Day25::solve_p1(&input), Day25::solve_p2(&input)),
        _ => panic!("day dne"),
    };

    match (p1_sol, p2_sol) {
        (Ok(s1), Ok(s2)) => output_sol!(day, s1, s2),
        (Ok(s1), Err(e)) => {
            output_sol!(day, s1, e)
        }
        (Err(e), Ok(s2)) => {
            output_sol!(day, e, s2)
        }
        (Err(e1), Err(e2)) => {
            output_sol!(day, e1, e2)
        }
    };

    Ok(())
}

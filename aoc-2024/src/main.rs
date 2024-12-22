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

    let input = read_to_string(format!("inputs/day{day}.txt\0"))?;

    let (p1_sol, p2_sol) = match day {
        1 => (Day01::solve_p1(&input), Day01::solve_p2(&input)),
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

mod day1;
mod day1_2020;
mod day2;
mod day2_2020;
mod day3;
mod day4;

use std::collections::binary_heap::Iter;
use std::mem::replace;
use std::str::FromStr;
use std::{fs, os::unix::prelude::OsStringExt};

// use day1_2020::solve2020_day1p1;

use crate::day1::{solve_day1p1, solve_day1p2};
// use crate::day2::{solve_day2p1, solve_day2p2};
fn main() {
    let filename = "inputs/day4input.txt";
    let contents = fs::read_to_string(filename).expect("file not found");
    let mut file = contents.lines();
}

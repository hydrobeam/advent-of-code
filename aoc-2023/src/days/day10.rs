use alloc::vec::Vec;

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day10;

#[derive(Debug)]
enum Terrain {
    North,
    West,
    NE,
    NW,
    SW,
    SE,
    Ground,
}

impl From<u8> for Terrain {
    fn from(value: u8) -> Self {
        use Terrain::*;
        match value {
            b'|' => North,
            b'-' => West,
            b'L' => NE,
            b'J' => NW,
            b'7' => SW,
            b'F' => SE,
            b'.' => Ground,
            _ => unreachable!(), // b'S' => {},
        }
    }
}

impl Solution for Day10 {
    fn solve_p1(input: &str) -> AocResult {
        let mut ret = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();

        let num_rows = ret.len();
        let num_cols = ret[0].len();

        // skip newlines
        let s_loc = input.rfind('S').unwrap();
        let s = [s_loc / (num_cols + 1), s_loc % (num_cols + 1)];

        dbg!(ret[s[0]][s[1]] as char);

        Err(AocError::Incomplete)
    }

    fn solve_p2(_input: &str) -> AocResult {
        Err(AocError::Incomplete)
    }
}

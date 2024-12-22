use std::collections::HashMap;

use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn solve_p1(input: &str) -> AocResult {
        let mut l1 = Vec::new();
        let mut l2 = Vec::new();
        input.lines().for_each(|x| {
            let mut l = x.split_ascii_whitespace();
            let i1 = l.next().unwrap().parse::<u32>().unwrap();
            let i2 = l.next().unwrap().parse::<u32>().unwrap();

            l1.push(i1);
            l2.push(i2);
        });

        l1.sort_unstable();
        l2.sort_unstable();

        let sum = l1
            .iter()
            .zip(l2)
            .fold(0, |acc, x| acc + (x.0.abs_diff(x.1)));

        Ok(AocSol::Unsigned(sum as usize))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut l1 = Vec::new();
        let mut l2 = HashMap::new();
        input.lines().for_each(|x| {
            let mut l = x.split_ascii_whitespace();
            let i1 = l.next().unwrap().parse::<u32>().unwrap();
            let i2 = l.next().unwrap().parse::<u32>().unwrap();

            l1.push(i1);

            let a = l2.entry(i2).or_insert(0);
            *a += 1;
        });

        let sum = l1.iter().fold(0, |acc, v| {
            acc + (v * l2.get(v).unwrap_or(&0))
        });

        Ok(AocSol::Unsigned(sum as usize))
    }
}

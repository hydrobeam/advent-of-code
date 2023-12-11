use alloc::vec::Vec;

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn solve_p1(input: &str) -> AocResult {
        Ok(AocSol::Unsigned(
            input
                .lines()
                .map(|line| {
                    let i = line.find(':').unwrap();

                    let (winnings, havings) = line[(i + 2)..].split_once('|').unwrap();

                    let winnings = winnings
                        .trim()
                        .split_ascii_whitespace()
                        .map(|num| u32::from_str_radix(num, 10).unwrap() as u16)
                        .collect::<Vec<u16>>();

                    let havings = havings
                        .trim()
                        .split_ascii_whitespace()
                        .map(|num| u32::from_str_radix(num, 10).unwrap() as u16);

                    let mut winners: u32 = 0;
                    for value in havings {
                        if winnings.contains(&value) {
                            winners += 1
                        }
                    }
                    if winners > 0 {
                        2_usize.pow(winners - 1)
                    } else {
                        0
                    }
                })
                .sum::<usize>(),
        ))
    }

    fn solve_p2(input: &str) -> AocResult {
        let cards = input
            .lines()
            .map(|line| {
                let i = line.find(':').unwrap();

                let (winnings, havings) = line[(i + 2)..].split_once('|').unwrap();

                let winnings = winnings
                    .trim()
                    .split_ascii_whitespace()
                    .map(|num| u32::from_str_radix(num, 10).unwrap() as u16)
                    .collect::<Vec<u16>>();

                let havings = havings
                    .trim()
                    .split_ascii_whitespace()
                    .map(|num| u32::from_str_radix(num, 10).unwrap() as u16);

                let mut winners: u32 = 0;
                for value in havings {
                    if winnings.contains(&value) {
                        winners += 1
                    }
                }
                winners
            })
            .collect::<Vec<u32>>();

        // dbg!(&cards);
        let mut tot_scratches = 0;

        for i in 0..cards.len() {
            mew(&mut tot_scratches, &cards, i);
        }

        Ok(AocSol::Unsigned(tot_scratches as usize))
    }
}

fn mew(val: &mut u32, cards: &[u32], mut i: usize) {
    let curr = cards[i];
    i += 1;
    *val += 1;

    for i in i..(i + (curr as usize)) {
        mew(val, cards, i);
    }
}

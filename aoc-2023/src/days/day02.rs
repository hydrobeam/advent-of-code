use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day02;

enum Colour {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl From<(u8, usize)> for Colour {
    fn from(value: (u8, usize)) -> Self {
        match value {
            (b'r', s) => Colour::Red(s),
            (b'g', s) => Colour::Green(s),
            (b'b', s) => Colour::Blue(s),
            _ => unreachable!(),
        }
    }
}

impl Solution for Day02 {
    fn solve_p1(input: &str) -> AocResult {
        const RED_LIMIT: usize = 12;
        const GREEN_LIMIT: usize = 13;
        const BLUE_LIMIT: usize = 14;

        Ok(input
            .lines()
            .map(|line| {
                let ind = line.find(':').unwrap();
                let id = line[5..ind].parse::<usize>().unwrap();

                let mut rolls = line[ind + 1..].split(';').map(|roll| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    roll.split(',')
                        .map(|item| {
                            let mut a = item.trim().split(' ');
                            let num = a.next().unwrap().parse::<usize>().unwrap();
                            let colour = a.next().unwrap();
                            Colour::from((colour.as_bytes()[0], num))
                        })
                        .for_each(|col| match col {
                            Colour::Red(s) => r = r.max(s),
                            Colour::Green(s) => g = g.max(s),
                            Colour::Blue(s) => b = b.max(s),
                        });
                    r > RED_LIMIT || b > BLUE_LIMIT || g > GREEN_LIMIT
                });

                if rolls.any(|a| a) {
                    0
                } else {
                    id
                }
            })
            .sum::<usize>()
            .into())
    }

    fn solve_p2(input: &str) -> AocResult {
        Ok(input
            .lines()
            .map(|line| {
                let ind = line.find(':').unwrap();
                let _id = line[5..ind].parse::<usize>().unwrap();

                line[ind + 1..]
                    .split(';')
                    .map(|roll| {
                        let mut r = 0;
                        let mut g = 0;
                        let mut b = 0;
                        roll.split(',')
                            .map(|item| {
                                let mut a = item.trim().split(' ');
                                let num = a.next().unwrap().parse::<usize>().unwrap();
                                let colour = a.next().unwrap();
                                Colour::from((colour.as_bytes()[0], num))
                            })
                            .for_each(|col| match col {
                                Colour::Red(s) => r = r.max(s),
                                Colour::Green(s) => g = g.max(s),
                                Colour::Blue(s) => b = b.max(s),
                            });
                        [r, g, b]
                    })
                    .fold([0, 0, 0], |curr_max, x| {
                        [
                            curr_max[0].max(x[0]),
                            curr_max[1].max(x[1]),
                            curr_max[2].max(x[2]),
                        ]
                    })
                    .iter()
                    .product::<usize>()
            })
            .sum::<usize>()
            .into())
    }
}

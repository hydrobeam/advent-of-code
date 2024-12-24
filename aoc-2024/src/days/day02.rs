use crate::solutions::{AocResult, AocSol, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn solve_p1(input: &str) -> AocResult {
        let count = input
            .lines()
            .filter(|x| {
                let mut f = x
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<u32>().unwrap())
                    .peekable();

                let mut prev = f.next().unwrap();
                let next = f.peek().unwrap();
                let increasing = *next > prev;

                for item in f {
                    if !is_valid(prev, item, increasing) {
                        return false;
                    }
                    prev = item;
                }

                true
            })
            .count();

        Ok(AocSol::Unsigned(count))
    }

    fn solve_p2(input: &str) -> AocResult {
        let count = input
            .lines()
            .filter(|x| {
                let mut f = x
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<u32>().unwrap())
                    .peekable();

                let mut prev = f.next().unwrap();
                let next = f.peek().unwrap();
                let increasing = *next > prev;

                let mut inv_count = 0;

                for item in f {
                    if !is_valid(prev, item, increasing) {
                        inv_count += 1;
                    } else {
                        prev = item;
                    }
                }

                if inv_count <= 1 {
                    return true;
                }

                let mut f = x
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<u32>().unwrap())
                    .peekable();

                f.next();
                let mut prev = f.next().unwrap();
                let next = f.peek().unwrap();
                let increasing = *next > prev;

                for item in f {
                    if !is_valid(prev, item, increasing) {
                        return false;
                    }
                    prev = item;
                }

                true
            })
            .count();

        Ok(AocSol::Unsigned(count))
    }
}

fn is_valid(prev: u32, item: u32, increasing: bool) -> bool {
    let diff = prev.abs_diff(item);

    (diff >= 1 && diff <= 3) && (((item > prev) && increasing) || ((item < prev) && !increasing))
}

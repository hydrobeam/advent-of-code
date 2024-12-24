use alloc::vec::Vec;

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn solve_p1(input: &str) -> AocResult {
        let mut histories = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|num| num.parse::<isize>().unwrap())
                    .peekable()
            })
            .peekable();

        let LIMIT = histories.peek().unwrap().clone().count() - 1;

        // cascading arrays
        let mut working_space: Vec<Vec<isize>> = Vec::with_capacity(LIMIT);
        for i in 0..LIMIT {
            working_space.push(alloc::vec![0;LIMIT - i]);
        }

        let mut total = 0;

        for mut hist in histories {
            // 1 2 3

            // prev: 1
            // 2 3
            // prev: 2
            // 2 3
            let mut last_num = hist.next().unwrap();
            for (i, num) in hist.enumerate() {
                working_space[0][i] = num - last_num;
                last_num = num;
            }

            let mut row_ind = 0;
            while row_ind < LIMIT - 1 {
                let (prev, post) = working_space.split_at_mut(row_ind + 1);
                let prev = &prev[row_ind];
                let post = &mut post[0];

                let mut grah = prev.iter();
                let mut prev_num = grah.next().unwrap();
                for (i, num) in grah.enumerate() {
                    post[i] = num - prev_num;
                    prev_num = num;
                }
                row_ind += 1;
                if post.iter().all(|&x| x == 0) {
                    break;
                }
            }

            let mut prev_num = 0;
            for curr in working_space[..row_ind].iter().rev() {
                let curr_num = curr[curr.len() - 1];
                let last_dupped = curr_num + prev_num;
                prev_num = last_dupped;
            }

            total += prev_num + last_num;
        }

        Ok(AocSol::Signed(total))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut histories = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|num| num.parse::<isize>().unwrap())
                    .peekable()
            })
            .peekable();

        let LIMIT = histories.peek().unwrap().clone().count() - 1;

        // cascading arrays
        let mut working_space: Vec<Vec<isize>> = Vec::with_capacity(LIMIT);
        for i in 0..LIMIT {
            working_space.push(alloc::vec![0;LIMIT - i]);
        }

        let mut total = 0;

        for mut hist in histories {
            let mut last_num = hist.next().unwrap();
            let first_num = last_num;
            for (i, num) in hist.enumerate() {
                working_space[0][i] = num - last_num;
                last_num = num;
            }

            let mut row_ind = 0;
            while row_ind < LIMIT - 1 {
                let (prev, post) = working_space.split_at_mut(row_ind + 1);
                let prev = &prev[row_ind];
                let post = &mut post[0];

                let mut grah = prev.iter();
                let mut prev_num = grah.next().unwrap();
                for (i, num) in grah.enumerate() {
                    post[i] = num - prev_num;
                    prev_num = num;
                }
                row_ind += 1;
                if post.iter().all(|&x| x == 0) {
                    break;
                }
            }

            let mut prev_num = 0;
            for curr in working_space[..row_ind].iter().rev() {
                let curr_num = curr[0];
                let last_dupped = curr_num - prev_num;
                prev_num = last_dupped;
            }

            total += first_num - prev_num;
        }

        Ok(AocSol::Signed(total))
    }
}

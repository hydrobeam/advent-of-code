use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_p1(input: &str) -> AocResult {
        let l = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();

        let rows = l.len();
        let cols = l[0].len();

        let mut part_num_sum = 0;
        for i in 0..rows {
            for j in 0..cols {
                let col = l[i][j];
                // we have a symbol
                if col != b'.' && !col.is_ascii_digit() {
                    let u_row = i.checked_sub(1).unwrap_or(0);

                    let l_col = j.checked_sub(1).unwrap_or(0);
                    let r_col = if j + 1 == cols { j } else { j + 1 };

                    let mut curr_row = u_row;
                    let mut curr_col = l_col;

                    loop {
                        // reset
                        if curr_col > r_col {
                            curr_col = l_col;
                            curr_row += 1;

                            if curr_row > i + 1 {
                                break;
                            }
                        }
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);
                        if let Some((left, right)) = nam {
                            part_num_sum += Day03::to_num_from_slice(&l[curr_row][left..right]);
                            curr_col = right;
                        } else {
                            curr_col += 1
                        }
                    }
                }
            }
        }

        Ok(AocSol::Unsigned(part_num_sum as usize))
    }

    fn solve_p2(input: &str) -> AocResult {
        let l = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();

        let rows = l.len();
        let cols = l[0].len();

        let mut total_gear_ratio = 0;
        for i in 0..rows {
            for j in 0..cols {
                let col = l[i][j];
                // we have a symbol
                if col == b'*' {
                    let u_row = i.checked_sub(1).unwrap_or(0);

                    let l_col = j.checked_sub(1).unwrap_or(0);
                    let r_col = if j + 1 == cols { j } else { j + 1 };

                    let mut curr_row = u_row;
                    let mut curr_col = l_col;

                    let mut num_gears = 0;
                    let mut curr_gear_ratio = 1;

                    loop {
                        // reset
                        if curr_col > r_col {
                            curr_col = l_col;
                            curr_row += 1;

                            if curr_row > i + 1 {
                                break;
                            }
                        }
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);
                        if let Some((left, right)) = nam {
                            curr_gear_ratio *= Day03::to_num_from_slice(&l[curr_row][left..right]);
                            num_gears += 1;
                            curr_col = right;
                        } else {
                            curr_col += 1
                        }
                    }

                    if num_gears == 2 {
                        total_gear_ratio += curr_gear_ratio;
                    }
                }
            }
        }

        Ok(AocSol::Unsigned(total_gear_ratio as usize))
    }
}

impl Day03 {
    fn to_num_from_slice(val: &[u8]) -> u32 {
        let src = unsafe { core::str::from_utf8_unchecked(val) };
        u32::from_str_radix(src, 10).unwrap()
    }

    fn nums_from_row(
        l: &[&[u8]],
        row_num: usize,
        col_num: usize,
        cols: usize,
    ) -> Option<(usize, usize)> {
        let mut left_num = col_num;
        let mut right_num = col_num;

        // top left
        let row = l[row_num];
        let tl = row[col_num];
        if tl.is_ascii_digit() {
            while row[left_num].is_ascii_digit() {
                if left_num == 0 {
                    break;
                }
                left_num -= 1;
            }
            while row[right_num].is_ascii_digit() {
                right_num += 1;
                if right_num == cols {
                    break;
                }
            }

            if !row[left_num].is_ascii_digit() {
                left_num += 1;
            }
            Some((left_num, right_num))
        } else {
            None
        }
    }
}

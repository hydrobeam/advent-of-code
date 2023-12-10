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
            let row = l[i];
            for j in 0..cols {
                let col = row[j];

                // we have a symbol
                if col != b'.' && !col.is_ascii_digit() {
                    let u_row = i.checked_sub(1).unwrap_or(0);

                    let l_col = j.checked_sub(1).unwrap_or(0);
                    let r_col = if j + 1 == cols { j } else { j + 1 };

                    let mut curr_row = u_row;
                    let mut curr_col = l_col;

                    // skip if u_row is 0
                    if i != 0 {
                        loop {
                            // topleft/top row
                            if curr_col > r_col {
                                // reset
                                curr_col = l_col;
                                curr_row += 1;
                                break;
                            }
                            let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                            match nam {
                                Some((left, right)) => {
                                    part_num_sum +=
                                        Day03::to_num_from_slice(&l[curr_row][left..right])?;
                                    curr_col = right;
                                }
                                None => curr_col += 1,
                            }
                        }
                    }

                    if j.checked_sub(1).is_some() {
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                        if let Some((left, right)) = nam {
                            part_num_sum += Day03::to_num_from_slice(&l[curr_row][left..right])?;
                        }

                        curr_col = r_col;
                    }

                    // middle right

                    // if not at max col index
                    if j != cols - 1 {
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                        if let Some((left, right)) = nam {
                            part_num_sum += Day03::to_num_from_slice(&l[curr_row][left..right])?;
                        }
                        curr_row += 1;
                        curr_col = l_col;
                    }

                    if i != rows - 1 {
                        loop {
                            if curr_col > j + 1 {
                                break;
                            }
                            let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                            match nam {
                                Some((left, right)) => {
                                    part_num_sum +=
                                        Day03::to_num_from_slice(&l[curr_row][left..right])?;
                                    curr_col = right;
                                }
                                None => curr_col += 1,
                            }
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
            let row = l[i];
            for j in 0..cols {
                let item = row[j];

                let mut gear_ratio = 1;
                let mut num_gears = 0;
                // we have a symbol
                if item == b'*' {
                    let u_row = i.checked_sub(1).unwrap_or(0);

                    let l_col = j.checked_sub(1).unwrap_or(0);
                    let r_col = if j + 1 == cols { j } else { j + 1 };

                    let mut curr_row = u_row;
                    let mut curr_col = l_col;

                    // skip if u_row is 0
                    if i != 0 {
                        loop {
                            // topleft/top row
                            if curr_col > r_col {
                                // reset
                                curr_col = l_col;
                                curr_row += 1;
                                break;
                            }
                            let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                            match nam {
                                Some((left, right)) => {
                                    let num = Day03::to_num_from_slice(&l[curr_row][left..right])?;
                                    gear_ratio *= num;
                                    num_gears += 1;
                                    curr_col = right;
                                }
                                None => curr_col += 1,
                            }
                        }
                    }

                    // middle left

                    // if not at min col index
                    if j.checked_sub(1).is_some() {
                        // left
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                        match nam {
                            Some((left, right)) => {
                                let num = Day03::to_num_from_slice(&l[curr_row][left..right])?;

                                gear_ratio *= num;
                                num_gears += 1;
                            }
                            None => {}
                        }

                        curr_col = r_col;
                    }

                    // middle right

                    // if not at max col index
                    if j != cols - 1 {
                        let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                        match nam {
                            Some((left, right)) => {
                                let num = Day03::to_num_from_slice(&l[curr_row][left..right])?;
                                gear_ratio *= num;
                                num_gears += 1;
                            }
                            None => {}
                        }
                        curr_row += 1;
                        curr_col = l_col;
                    }

                    if i != rows - 1 {
                        loop {
                            if curr_col > j + 1 {
                                break;
                            }
                            let nam = Day03::nums_from_row(&l, curr_row, curr_col, cols);

                            match nam {
                                Some((left, right)) => {
                                    let num = Day03::to_num_from_slice(&l[curr_row][left..right])?;
                                    if gear_ratio == 0 {
                                        gear_ratio = num;
                                    } else {
                                        gear_ratio *= num;
                                    }
                                    num_gears += 1;
                                    curr_col = right;
                                }
                                None => curr_col += 1,
                            }
                        }
                    }

                    // final
                    if num_gears == 2 {
                        total_gear_ratio += gear_ratio;
                    }
                }
            }
        }

        Ok(AocSol::Unsigned(total_gear_ratio as usize))
    }
}

impl Day03 {
    fn to_num_from_slice(val: &[u8]) -> Result<u32, AocError> {
        let src = unsafe { core::str::from_utf8_unchecked(val) };
        u32::from_str_radix(src, 10).map_err(|op| AocError::Any(Box::new(op)))
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

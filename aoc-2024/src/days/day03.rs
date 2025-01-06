use core::str;

use crate::solutions::{AocResult, AocSol, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_p1(input: &str) -> AocResult {
        let mut ind = 0;
        let arr = input.as_bytes();

        let mut sum = 0;
        while ind < arr.len() {
            if arr[ind] == b'm'
                && arr[ind + 1] == b'u'
                && arr[ind + 2] == b'l'
                && arr[ind + 3] == b'('
            {
                let mut ind_1 = ind + 4;
                while arr[ind_1].is_ascii_digit() {
                    ind_1 += 1;
                }
                // if no progress was made
                if ind_1 == ind + 4 {
                    ind += 1;
                    continue;
                }
                let num_1 = str::from_utf8(&arr[(ind + 4)..ind_1])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                if arr[ind_1] != b',' {
                    ind += 1;
                    continue;
                }

                let mut ind_2 = ind_1 + 1;
                while arr[ind_2].is_ascii_digit() {
                    ind_2 += 1;
                }
                // if no progress was made
                if ind_2 == ind_1 + 1 {
                    ind += 1;
                    continue;
                }

                let num_2 = str::from_utf8(&arr[(ind_1 + 1)..ind_2])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                if arr[ind_2] != b')' {
                    ind += 1;
                    continue;
                }

                sum += num_1 * num_2;
                ind = ind_2 + 1;
            } else {
                ind += 1;
            }
        }

        Ok(AocSol::Unsigned(sum))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut ind = 0;
        let arr = input.as_bytes();

        let mut sum = 0;
        let mut enabled: bool = true;
        while ind < arr.len() {
            if enabled
                && arr[ind] == b'm'
                && arr[ind + 1] == b'u'
                && arr[ind + 2] == b'l'
                && arr[ind + 3] == b'('
            {
                let mut ind_1 = ind + 4;
                while arr[ind_1].is_ascii_digit() {
                    ind_1 += 1;
                }
                // if no progress was made
                if ind_1 == ind + 4 {
                    ind += 1;
                    continue;
                }
                let num_1 = str::from_utf8(&arr[(ind + 4)..ind_1])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                if arr[ind_1] != b',' {
                    ind += 1;
                    continue;
                }

                let mut ind_2 = ind_1 + 1;
                while arr[ind_2].is_ascii_digit() {
                    ind_2 += 1;
                }
                // if no progress was made
                if ind_2 == ind_1 + 1 {
                    ind += 1;
                    continue;
                }

                let num_2 = str::from_utf8(&arr[(ind_1 + 1)..ind_2])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                if arr[ind_2] != b')' {
                    ind += 1;
                    continue;
                }

                sum += num_1 * num_2;
                ind = ind_2 + 1;
            } else if arr[ind] == b'd'
                && arr[ind + 1] == b'o'
                && arr[ind + 2] == b'n'
                && arr[ind + 3] == b'\''
                && arr[ind + 4] == b't'
                && arr[ind + 5] == b'('
                && arr[ind + 6] == b')'
            {
                enabled = false;
                ind = ind + 7;
            } else if arr[ind] == b'd'
                && arr[ind + 1] == b'o'
                && arr[ind + 2] == b'('
                && arr[ind + 3] == b')'
            {
                enabled = true;
                ind = ind + 4;
            } else {
                ind += 1;
            }
        }

        Ok(AocSol::Unsigned(sum))
    }
}

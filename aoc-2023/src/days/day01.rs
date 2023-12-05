use core::fmt::Debug;

use crate::{dbg, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn solve_p1(input: &str) -> impl core::fmt::Debug + core::fmt::Display {
        input
            .lines()
            .map(|line| {
                let l = line.as_bytes();
                let mut i1 = 0;
                let mut i2 = l.len() - 1;
                while !u8::is_ascii_digit(&l[i1]) {
                    i1 += 1;
                }
                while !u8::is_ascii_digit(&l[i2]) {
                    i2 -= 1;
                }
                ((l[i1] - 48) as usize) * 10 + ((l[i2] - 48) as usize)
            })
            .sum::<usize>()
    }

    fn solve_p2(input: &str) -> impl core::fmt::Debug + core::fmt::Display {
        input
            .lines()
            .map(|line| {
                let l = line.as_bytes();
                let mut i1 = 0;
                let mut i2 = l.len() - 1;
                let first_val;
                let second_val;
                loop {
                    let item = &l[i1];
                    if u8::is_ascii_digit(item) {
                        first_val = item - 48;
                        break;
                    }

                    let arr = &l[i1..];
                    match item {
                        b'o' => {
                            if arr.starts_with(b"one") {
                                first_val = 1;
                                break;
                            };
                        }
                        b't' => {
                            if arr.starts_with(b"two") {
                                first_val = 2;
                                break;
                            };
                            if arr.starts_with(b"three") {
                                first_val = 3;
                                break;
                            };
                        }
                        b'f' => {
                            if arr.starts_with(b"four") {
                                first_val = 4;
                                break;
                            };
                            if arr.starts_with(b"five") {
                                first_val = 5;
                                break;
                            };
                        }
                        b's' => {
                            if arr.starts_with(b"six") {
                                first_val = 6;
                                break;
                            };
                            if arr.starts_with(b"seven") {
                                first_val = 7;
                                break;
                            };
                        }
                        b'e' => {
                            if arr.starts_with(b"eight") {
                                first_val = 8;
                                break;
                            };
                        }
                        b'n' => {
                            if arr.starts_with(b"nine") {
                                first_val = 9;
                                break;
                            };
                        }
                        _ => {}
                    }
                    i1 += 1
                }

                loop {
                    let item = &l[i2];
                    if u8::is_ascii_digit(item) {
                        second_val = item - 48;
                        break;
                    }

                    let arr = &l[..=i2];
                    match item {
                        b'e' => {
                            if arr.ends_with(b"one") {
                                second_val = 1;
                                break;
                            };
                            if arr.ends_with(b"three") {
                                second_val = 3;
                                break;
                            };
                            if arr.ends_with(b"five") {
                                second_val = 5;
                                break;
                            };
                            if arr.ends_with(b"nine") {
                                second_val = 9;
                                break;
                            };
                        }
                        b'o' => {
                            if arr.ends_with(b"two") {
                                second_val = 2;
                                break;
                            };
                        }
                        b'r' => {
                            if arr.ends_with(b"four") {
                                second_val = 4;
                                break;
                            };
                        }
                        b'x' => {
                            if arr.ends_with(b"six") {
                                second_val = 6;
                                break;
                            };
                        }
                        b't' => {
                            if arr.ends_with(b"eight") {
                                second_val = 8;
                                break;
                            };
                        }
                        b'n' => {
                            if arr.ends_with(b"seven") {
                                second_val = 7;
                                break;
                            };
                        }
                        _ => {}
                    }

                    i2 -= 1;
                }

                ((first_val * 10) + second_val) as usize
            })
            .sum::<usize>()
    }
}

// mod test {
//     use crate::{dbg, Solution};

//     use super::Day01;

//     #[test]
//     fn basic() {
//         let ret = Day01::solve_p1(
//             r#"1abc2
// "#,
//         );

//         dbg!(ret);
//     }
// }

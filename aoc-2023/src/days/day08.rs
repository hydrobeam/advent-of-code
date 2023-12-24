use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::solutions::{AocError, AocResult, AocSol, Solution};
use crate::{dbg, println};

pub struct Day08;

fn word_to_u16(val: &[u8]) -> u16 {
    // ABC is like
    // 123 in base 10
    // handle 3 first, so rev
    val.iter()
        .rev()
        .enumerate()
        .map(|(i, val)| 26_u16.pow(i as u32) * (val - CHAR_DIFF) as u16)
        .sum()
}

/// for debugging purposes
fn u16_to_word(mut val: u16) -> String {
    let mut abc = [0; 3];

    abc[2] = ((val % 26) + CHAR_DIFF as u16) as u8;
    val /= 26;
    abc[1] = ((val % 26) + CHAR_DIFF as u16) as u8;
    val /= 26;
    abc[0] = ((val % 26) + CHAR_DIFF as u16) as u8;

    unsafe { String::from_utf8_unchecked((&abc).to_vec()) }
}

const CHAR_DIFF: u8 = 65; // value of 'A'
impl Solution for Day08 {
    fn solve_p1(input: &str) -> AocResult {
        // return Err(AocError::Incomplete);
        let mut inp = input.lines();

        let instructions = inp
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|&x| x == b'R') // true -> right, false -> left
            .cycle(); // loop ad infinitum

        // consider each path as a base 26 number, create an array 26^3 in length, where each cell
        // stores a tuple of path

        // 26^3 = 17576;
        // a u16 is big enough for all our values. saves a lot of bits over a u32
        let mut path_arr: [(u16, u16); 17576] = [(0, 0); 17576];

        inp.next(); // skip
        inp.for_each(|x| {
            // 0123456789012345
            // SGR = (JLL, VRV)
            let bytes = x.as_bytes();
            let first = word_to_u16(&bytes[0..3]);
            let p1 = word_to_u16(&bytes[7..10]);
            let p2 = word_to_u16(&bytes[12..15]);
            path_arr[first as usize] = (p1, p2);
        });
        let end = word_to_u16("ZZZ".as_bytes()) as usize;
        let mut count = 0;
        let mut start = word_to_u16("AAA".as_bytes()) as usize;
        for go_right in instructions {
            let paths = path_arr[start];
            if go_right {
                start = paths.1 as usize;
            } else {
                start = paths.0 as usize;
            }
            count += 1;
            if start == end {
                break;
            }
        }
        Ok(AocSol::Unsigned(count))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut inp = input.lines();

        let instructions = inp
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|&x| x == b'R') // true -> right, false -> left
            .cycle(); // loop ad infinitum

        // consider each path as a base 26 number, create an array 26^3 in length, where each cell
        // stores a tuple of path

        // 26^3 = 17576;
        // a u16 is big enough for all our values. saves a lot of bits over a u32
        let mut path_arr: [(u16, u16); 17576] = [(0, 0); 17576];

        inp.next(); // skip
        let mut curr_nodes = Vec::new();
        let mut count = 0;

        inp.for_each(|x| {
            // 0123456789012345
            // SGR = (JLL, VRV)
            let bytes = x.as_bytes();
            let first = word_to_u16(&bytes[0..3]);
            if bytes[2] == b'A' {
                curr_nodes.push(first);
            }
            let p1 = word_to_u16(&bytes[7..10]);
            let p2 = word_to_u16(&bytes[12..15]);

            path_arr[first as usize] = (p1, p2);
        });

        let mut appearance_arr: [usize; 6] = [0; 6];
        for go_right in instructions {
            for node in curr_nodes.iter_mut() {
                // let paths = path_arr[node];
                let paths = path_arr[*node as usize];
                if go_right {
                    *node = paths.1;
                } else {
                    *node = paths.0;
                }
            }

            count += 1;

            for (i, node) in curr_nodes.iter().enumerate() {
                if (node % 26) as u8 == b'Z' - CHAR_DIFF {
                    appearance_arr[i] = count;
                }
            }
            if appearance_arr.iter().all(|&x| x != 0) {
                break;
            }
        }

        let lcmed = appearance_arr
            .iter()
            .copied()
            .reduce(|acc, x| lcm(acc, x))
            .unwrap();

        Ok(AocSol::Unsigned(lcmed))
    }
}

// lcm 7 4
// 7 8
// 14 8
// 14 12
// 14 16
// 21 20
// 21 24
// 28 24
// 28 28
fn lcm(a: usize, b: usize) -> usize {
    let mut n = a;
    let mut m = b;
    while n != m {
        if m > n {
            n += a;
        } else {
            m += b;
        }
    }
    n
}

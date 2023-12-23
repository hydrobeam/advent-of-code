use alloc::string::{String, ToString};

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn solve_p1(input: &str) -> AocResult {
        let mut lines = input.lines();
        let times = lines.next().unwrap();
        let times = unsafe { core::str::from_utf8_unchecked(&times.as_bytes()[6..]) }
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap());
        let distances = lines.next().unwrap();
        let distances = unsafe { core::str::from_utf8_unchecked(&distances.as_bytes()[9..]) }
            .split_whitespace()
            .map(|abc| abc.parse::<f32>().unwrap());

        let mut tot_ways: f32 = 1.0;

        for (time, distance) in times.zip(distances) {
            // build_up = b
            // distance = d
            // time = t
            // record = r
            //
            // d = (t-b)*b
            // d = tb - b^2
            //
            // need b s.t. d > r
            // tb - b^2 > r
            //
            // (goal) solve:
            // -b^2 + tb - r > 0
            //
            // quadratic formula:
            // x = (-b +/- sqrt(b^2 - 4ac))/2a
            let t = time;
            let r = distance;
            let det = t * t - 4_f32 * (-1_f32) * (-r);

            if det <= 0_f32 {
                // < 0 means no roots i.e. no ways to beat record
                // = 0 implies one way to beat record, but n * 1 = n so we don't care
                continue;
            }

            let det_sqrt: f32;
            unsafe {
                core::arch::asm!(
                    "sqrtss {0}, {0}",
                    inlateout(xmm_reg) det => det_sqrt,
                    options(nostack)
                )
            };

            let max_build_up = (-t - det_sqrt) / -2_f32;
            let min_build_up = (-t + det_sqrt) / -2_f32;

            let top: f32;
            // round down
            unsafe {
                core::arch::asm!(
                    "roundss {0}, {0}, 01b",
                    inlateout(xmm_reg) max_build_up => top,
                    options(nostack)
                )
            };

            let bot: f32;
            // round down
            unsafe {
                core::arch::asm!(
                    "roundss {0}, {0}, 10b",
                    inlateout(xmm_reg) min_build_up => bot,
                    options(nostack)
                )
            };

            let low: f32;
            // round down
            unsafe {
                core::arch::asm!(
                    "roundss {0}, {0}, 01b",
                    inlateout(xmm_reg) det_sqrt => low,
                    options(nostack)
                )
            };

            tot_ways *= top - bot + 1.0;

        }
        Ok(AocSol::String(tot_ways.to_string()))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut lines = input.lines();
        let time = lines.next().unwrap().as_bytes()[6..]
            .iter()
            .filter(|&&a| a != b' ')
            .copied()
            .collect::<alloc::vec::Vec<u8>>();
        let time = String::from_utf8_lossy(&time).parse::<u64>().unwrap() as f64;

        let distance = lines.next().unwrap().as_bytes()[9..]
            .iter()
            .filter(|&&a| a != b' ')
            .copied()
            .collect::<alloc::vec::Vec<u8>>();

        let distance = String::from_utf8_lossy(&distance).parse::<u64>().unwrap() as f64;

        let t = time;
        let r = distance;
        let det = t * t - 4_f64 * (-1_f64) * (-r);

        let det_sqrt: f64;
        unsafe {
            core::arch::asm!(
                "sqrtsd {0}, {0}",
                inlateout(xmm_reg) det => det_sqrt,
                options(nostack)
            )
        };

        let max_build_up = (-t - det_sqrt) / -2_f64;
        let min_build_up = (-t + det_sqrt) / -2_f64;

        let top: f64;
        // round down
        unsafe {
            core::arch::asm!(
                "roundsd {0}, {0}, 01b",
                inlateout(xmm_reg) max_build_up => top,
                options(nostack)
            )
        };

        let bot: f64;
        // round up
        unsafe {
            core::arch::asm!(
                "roundsd {0}, {0}, 10b",
                inlateout(xmm_reg) min_build_up => bot,
                options(nostack)
            )
        };

        Ok(AocSol::String((top - bot + 1.0).to_string()))
    }
}

// #[derive(Default, Debug, Clone, Copy)]
// struct vec3 {
//     e: [f64;3]
// }

// impl Abc {
//     fn new() -> Self {
//         todo!()

//     }
// }

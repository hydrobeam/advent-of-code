use alloc::vec::Vec;

use crate::dbg;
use crate::solutions::{AocError, AocResult, AocSol, Solution};

pub struct Day05;

#[derive(Debug)]
struct Range {
    src: usize,
    dest: usize,
    len: usize,
}

impl Range {
    fn map(&self, num: usize) -> usize {
        self.dest + (num - self.src)
    }
    fn contains(&self, num: usize) -> bool {
        num >= self.src && num < self.src + self.len
    }
}

impl Solution for Day05 {
    fn solve_p1(input: &str) -> AocResult {
        let mut src = input.split("\n\n");
        let mut seeds = src
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        while let Some(mapping) = src.next() {
            let veced_map = mapping
                .trim()
                .split('\n')
                .skip(1)
                .map(|line| {
                    let mut splat = line.split(' ');
                    let dest = splat.next().unwrap().parse::<usize>().unwrap();
                    let src = splat.next().unwrap().parse::<usize>().unwrap();
                    let len = splat.next().unwrap().parse::<usize>().unwrap();
                    Range { src, dest, len }
                })
                .collect::<Vec<Range>>();

            for seed in seeds.iter_mut() {
                for map in &veced_map {
                    if map.contains(*seed) {
                        *seed = map.map(*seed);
                        break;
                    }
                }
            }
        }

        Ok(AocSol::Unsigned(*seeds.iter().min().unwrap() as usize))
    }

    fn solve_p2(input: &str) -> AocResult {
        let mut src = input.split("\n\n");
        let seeds = src
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .array_chunks::<2>()
            .map(|num| {
                (
                    num[0].parse::<usize>().unwrap(),
                    (num[1].parse::<usize>().unwrap()),
                )
            })
            .collect::<Vec<(usize, usize)>>();

        let mut mappings = Vec::with_capacity(8);
        while let Some(mapping) = src.next() {
            let mut veced_map = mapping
                .trim()
                .split('\n')
                .skip(1)
                .map(|line| {
                    let mut splat = line.split(' ');
                    let dest = splat.next().unwrap().parse::<usize>().unwrap();
                    let src = splat.next().unwrap().parse::<usize>().unwrap();
                    let len = splat.next().unwrap().parse::<usize>().unwrap();
                    Range { src, dest, len }
                })
                .collect::<Vec<Range>>();
            veced_map.sort_unstable_by(|a, b| a.src.cmp(&b.src));
            mappings.push(veced_map);
        }

        let mut min_loc = usize::MAX;
        let mut prog = 0;
        for &(seed_start, len) in &seeds {
            dbg!(prog, min_loc);
            prog += 1;

            for mut seed in seed_start..seed_start + len {
                for mapping in &mappings {
                    let i = mapping.partition_point(|x| x.src <= seed);
                    // let i = 0;
                    if i > 0 && mapping[i - 1].contains(seed) {
                        seed = mapping[i - 1].map(seed);
                    }

                    // old solution, slower
                    // for map in mapping {
                    //     if map.contains(seed) {
                    //         seed = map.map(seed);
                    //         break;
                    //     }
                    // }
                }
                min_loc = min_loc.min(seed);
            }
        }
        Ok(AocSol::Unsigned(min_loc as usize))
    }
}

use alloc::vec::Vec;

use crate::solutions::{AocError, AocResult, AocSol, Solution};
use crate::{dbg, println};

pub struct Day07;

pub mod p1 {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum Card {
        N2,
        N3,
        N4,
        N5,
        N6,
        N7,
        N8,
        N9,
        T,
        J,
        Q,
        K,
        A,
    }

    impl From<u8> for Card {
        fn from(value: u8) -> Self {
            use Card::*;
            match value {
                b'A' => A,
                b'K' => K,
                b'Q' => Q,
                b'J' => J,
                b'T' => T,
                b'9' => N9,
                b'8' => N8,
                b'7' => N7,
                b'6' => N6,
                b'5' => N5,
                b'4' => N4,
                b'3' => N3,
                b'2' => N2,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    #[derive(Debug)]
    pub struct Hand {
        pub ty: HandType,
        pub val: [Card; 5],
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.ty == other.ty && self.val == other.val
        }
    }

    impl<'a> PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            if self.ty == other.ty {
                // equality clause
                Some(self.val.cmp(&other.val))
            } else {
                Some(self.ty.cmp(&other.ty))
            }
        }
    }

    #[derive(Debug)]
    pub struct Round {
        pub hand: Hand,
        pub bid: usize,
    }
}

mod p2 {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum Card {
        J,
        N2,
        N3,
        N4,
        N5,
        N6,
        N7,
        N8,
        N9,
        T,
        Q,
        K,
        A,
    }

    impl From<u8> for Card {
        fn from(value: u8) -> Self {
            use Card::*;
            match value {
                b'A' => A,
                b'K' => K,
                b'Q' => Q,
                b'J' => J,
                b'T' => T,
                b'9' => N9,
                b'8' => N8,
                b'7' => N7,
                b'6' => N6,
                b'5' => N5,
                b'4' => N4,
                b'3' => N3,
                b'2' => N2,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    #[derive(Debug)]
    pub struct Hand {
        pub ty: HandType,
        pub val: [Card; 5],
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.ty == other.ty && self.val == other.val
        }
    }

    impl<'a> PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            if self.ty == other.ty {
                // equality clause
                Some(self.val.cmp(&other.val))
            } else {
                Some(self.ty.cmp(&other.ty))
            }
        }
    }

    #[derive(Debug)]
    pub struct Round {
        pub hand: Hand,
        pub bid: usize,
    }
}

impl Solution for Day07 {
    fn solve_p1(input: &str) -> AocResult {
        use p1::*;
        let mut obj_arr: [Card; 5] = [Card::A; 5];
        let mut sorted_arr: [Card; 5] = [Card::A; 5];
        let mut round_vec: Vec<Round> = input
            .lines()
            .map(|x| {
                let val = x.split_once(' ').unwrap();
                let bid = val.1.parse::<usize>().unwrap();

                for (i, &item) in val.0.as_bytes().iter().enumerate() {
                    obj_arr[i] = Card::from(item);
                }
                let given_hand = obj_arr;
                obj_arr.sort_unstable();
                sorted_arr.copy_from_slice(&obj_arr);

                let ty = match obj_arr.partition_dedup().0.len() {
                    1 => HandType::FiveKind,
                    2 => {
                        // when deduped => 12 in both cases
                        // so examine the sorted array for patterns
                        if sorted_arr[3] == sorted_arr[4] && sorted_arr[0] == sorted_arr[1] {
                            // 11122 || 22211
                            HandType::FullHouse
                        } else {
                            // 11112
                            HandType::FourKind
                        }
                    }
                    3 => {
                        // when deduped => 123 in both cases
                        // so examine the sorted array for patterns
                        if (sorted_arr[0] == sorted_arr[1] && sorted_arr[1] == sorted_arr[2])
                            || (sorted_arr[1] == sorted_arr[2] && sorted_arr[2] == sorted_arr[3])
                            || (sorted_arr[2] == sorted_arr[3] && sorted_arr[3] == sorted_arr[4])
                        {
                            // 11123 || 12223 | 12333
                            HandType::ThreeKind
                        } else {
                            //
                            HandType::TwoPair
                        }
                    }
                    4 => HandType::OnePair,
                    5 => HandType::HighCard,
                    _ => unreachable!(),
                };

                let hand = Hand {
                    ty,
                    val: given_hand,
                };
                Round { hand, bid }
            })
            .collect();

        round_vec.sort_unstable_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

        // println!("{:?}", &round_vec);
        let sum: usize = round_vec
            .iter()
            .enumerate()
            .map(|(pos, round)| (pos + 1) * round.bid) // 0-indexed
            .sum();

        Ok(AocSol::Unsigned(sum))
    }

    fn solve_p2(input: &str) -> AocResult {
        use p2::*;
        let mut obj_arr: [Card; 5] = [Card::A; 5];
        let mut sorted_arr: [Card; 5] = [Card::A; 5];
        let mut round_vec: Vec<Round> = input
            .lines()
            .map(|x| {
                let val = x.split_once(' ').unwrap();
                let bid = val.1.parse::<usize>().unwrap();

                for (i, &item) in val.0.as_bytes().iter().enumerate() {
                    obj_arr[i] = Card::from(item);
                }
                let given_hand = obj_arr;
                obj_arr.sort_unstable();
                sorted_arr.copy_from_slice(&obj_arr);

                // jokers are the least when sorting, so they're first
                let mut num_jokers = 0;
                let mut ind = 0;
                while ind < 5 && sorted_arr[ind] == Card::J {
                    num_jokers += 1;
                    ind += 1;
                }

                let ty = match obj_arr.partition_dedup().0.len() {
                    1 => HandType::FiveKind,
                    2 => {
                        if num_jokers > 0 {
                            HandType::FiveKind
                        }
                        // when deduped => 12 in both cases
                        // so examine the sorted array for patterns
                        else if sorted_arr[3] == sorted_arr[4] && sorted_arr[0] == sorted_arr[1] {
                            // 11122 || 22211
                            HandType::FullHouse
                        } else {
                            // 11112
                            HandType::FourKind
                        }
                    }
                    3 => {
                        if num_jokers == 3 || num_jokers == 2 {
                            // 11JJ3 => 11113
                            // JJJ12 => 11112
                            HandType::FourKind
                        }
                        // when deduped => 123 in both cases
                        // so examine the sorted array for patterns
                        else if (sorted_arr[0] == sorted_arr[1] && sorted_arr[1] == sorted_arr[2])
                            || (sorted_arr[1] == sorted_arr[2] && sorted_arr[2] == sorted_arr[3])
                            || (sorted_arr[2] == sorted_arr[3] && sorted_arr[3] == sorted_arr[4])
                        {
                            // 11123 || 12223 | 12333
                            if num_jokers == 1 {
                                HandType::FourKind
                            } else {
                                HandType::ThreeKind
                            }
                        } else {
                            // 11223
                            if num_jokers == 1 {
                                HandType::FullHouse
                            } else {
                                HandType::TwoPair
                            }
                        }
                    }
                    4 => {
                        if num_jokers > 0 {
                            HandType::ThreeKind
                        } else {
                            HandType::OnePair
                        }
                    }
                    5 => {
                        if num_jokers > 0 {
                            HandType::OnePair
                        } else {
                            HandType::HighCard
                        }
                    }
                    _ => unreachable!(),
                };

                let hand = Hand {
                    ty,
                    val: given_hand,
                };
                Round { hand, bid }
            })
            .collect();

        round_vec.sort_unstable_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

        // println!("{:?}", &round_vec);
        let sum: usize = round_vec
            .iter()
            .enumerate()
            .map(|(pos, round)| (pos + 1) * round.bid) // 0-indexed
            .sum();

        Ok(AocSol::Unsigned(sum))
    }
}

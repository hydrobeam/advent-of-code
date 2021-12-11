use std::collections::{HashMap, HashSet};
use std::vec::IntoIter;

pub fn solve_day8p1() {
    let contents = include_str!("../inputs/day8input.txt").lines();
    let output: i64 = contents
        .map(|x| {
            x.split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .map(|st| st.chars().count() as i64)
                .filter(|&val| val == 3 || val == 4 || val == 7 || val == 2)
                .count() as i64
        })
        .sum();
    dbg!(output);
}

pub fn solve_day8p2() {
    let mut total: i64 = 0;
    let contents = include_str!("../inputs/day8input.txt").lines();

    let vals = contents.map(|x| {
        x.split('|')
            .map(|v| v.trim().split(' ').collect::<Vec<&str>>())
    });

    for mut line in vals {
        // will store the value for the sorted inputs, e.g. (acde -> 3)
        let mut hashie: HashMap<&str, i64> = HashMap::new();

        let inputs = &mut line.next().unwrap();
        inputs.sort_by(|a, b| (a.len()).cmp(&b.len()));
        // sort by length to get vals that we know
        let mut inputs = inputs
            .iter()
            .map(|&x| x.chars().sorted().collect::<String>());

        // this kinda sucks
        let one = inputs.next().unwrap();
        let seven = inputs.next().unwrap();
        let four = inputs.next().unwrap();
        let five_lengths = vec![
            inputs.next().unwrap(),
            inputs.next().unwrap(),
            inputs.next().unwrap(),
        ];

        let mut d = inputs.skip(3);
        let eight = d.next().unwrap();

        hashie.insert(&one, 1);
        hashie.insert(&seven, 7);
        hashie.insert(&four, 4);
        hashie.insert(&eight, 8);

        let one_hash: HashSet<char> = one.chars().collect();
        let seven_hash: HashSet<char> = seven.chars().collect();
        let four_hash: HashSet<char> = four.chars().collect();
        let eight_hash: HashSet<char> = eight.chars().collect();

        let top: HashSet<char> = seven_hash.difference(&one_hash).copied().collect();
        let mut not_two: Vec<&str> = Vec::new();

        let mut two_hash: HashSet<char> = HashSet::new();
        let mut three_hash: HashSet<char> = HashSet::new();
        let mut five_hash: HashSet<char> = HashSet::new();

        // pretty sure the logic here is ineffecient but it's scool
        for val in &five_lengths {
            let val_hash: HashSet<char> = val.chars().collect();
            let bottom_maybe: HashSet<char> = val_hash
                .difference(&four_hash)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&top)
                .copied()
                .collect();

            if bottom_maybe.len() == 2 {
                hashie.insert(val, 2);
                two_hash = val.chars().collect();
            } else {
                not_two.push(val);
            }
        }

        let mut bottom_right_hash: HashSet<char> = HashSet::new();
        let mut top_left_hash: HashSet<char> = HashSet::new();

        for val in not_two {
            let val_hash: HashSet<char> = val.chars().collect();
            let nw_se = val_hash
                .difference(&two_hash)
                .copied()
                .collect::<HashSet<char>>();

            // difference between 2 and 5 or 3 leaves top_left / bottom_right

            let temper = nw_se
                .difference(&one_hash)
                .copied()
                .collect::<HashSet<char>>();

            if temper.is_empty() {
                bottom_right_hash.insert(nw_se.iter().copied().next().unwrap());
                hashie.insert(val, 3);
                three_hash = val.chars().collect();
            } else {
                // if item still exists, then we have five
                top_left_hash.insert(temper.iter().copied().next().unwrap());
                hashie.insert(val, 5);
                five_hash = val.chars().collect();
            }
        }

        // two minu three just gives you bottom_left, so we don't need
        let bottom_left = string_sub_string(&two_hash, &three_hash).chars().collect();
        let middle = single_char_hash(&four_hash, &one_hash, &top_left_hash);
        let top_right = string_sub_string(&four_hash, &five_hash).chars().collect();

        let nine = string_sub_string(&eight_hash, &bottom_left);
        let six = string_sub_string(&eight_hash, &top_right);
        let zero = string_sub_string(&eight_hash, &middle);

        hashie.insert(&nine, 9);
        hashie.insert(&six, 6);
        hashie.insert(&zero, 0);

        let outputs = &line.next().unwrap();

        let mut num_vec: Vec<i64> = Vec::new();
        for outie in outputs {
            let sortie = outie.chars().sorted().collect::<String>();
            num_vec.push(hashie.get(&*sortie).copied().unwrap());
        }
        total += num_vec.into_iter().fold(0, |mut total, x| {
            total *= 10;
            total + x
        });
    }
    dbg!(total);
}

fn string_sub_string(s1: &HashSet<char>, s2: &HashSet<char>) -> String {
    s1.difference(s2)
        .collect::<String>()
        .chars()
        .sorted()
        .collect::<String>()
}

fn single_char_hash(s1: &HashSet<char>, s2: &HashSet<char>, s3: &HashSet<char>) -> HashSet<char> {
    s1.difference(s2)
        .copied()
        .collect::<HashSet<char>>()
        .difference(s3)
        .copied()
        .collect::<HashSet<char>>()
}

// home-made sorted implementation

trait Sort: Iterator {
    fn sorted(self) -> IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut hold_vec = self.collect::<Vec<Self::Item>>();
        hold_vec.sort();
        hold_vec.into_iter()
    }
}

impl<T: Sized> Sort for T where T: Iterator {}

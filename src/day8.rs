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
            .collect::<Vec<Vec<&str>>>()
    });

    for mut line in vals {
        let mut hashie: HashMap<&str, i64> = HashMap::new();

        let inputs = &mut line[0];
        inputs.sort_by(|a, b| (a.len()).cmp(&b.len()));
        // sort by length to get vals that we know
        let mut inputs = inputs
            .iter()
            .map(|&x| x.chars().sorted().collect::<String>());

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

        for val in &five_lengths {
            let val_hash: HashSet<char> = val.chars().collect();
            let bottom_wait = val_hash
                .difference(&four_hash)
                .copied()
                .collect::<HashSet<char>>();
            let bottom: HashSet<char> = bottom_wait.difference(&top).copied().collect();
            if bottom.len() == 2 {
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
                // nuke one, if it is empty, then we have removed three
                let bottom_right = nw_se.iter().copied().next().unwrap();
                bottom_right_hash.insert(bottom_right);
                hashie.insert(val, 3);
                three_hash = val.chars().collect();
            } else {
                // if item still exists, then we have five
                let top_left = temper.iter().copied().next().unwrap();
                top_left_hash.insert(top_left);
                hashie.insert(val, 5);
                five_hash = val.chars().collect();
            }
        }

        let bottom_left = single_char_hash(&two_hash, &three_hash, &bottom_right_hash);
        let middle = single_char_hash(&four_hash, &one_hash, &top_left_hash);
        let top_right = single_char_hash(&eight_hash, &five_hash, &bottom_left);

        let nine = string_sub_string(&eight_hash, &bottom_left);
        let six = string_sub_string(&eight_hash, &top_right);
        let zero = string_sub_string(&eight_hash, &middle);

        hashie.insert(&nine, 9);
        hashie.insert(&six, 6);
        hashie.insert(&zero, 0);

        let outputs = &line[1];

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
        hold_vec.sort_unstable();
        hold_vec.into_iter()
    }
}

impl<T: Sized> Sort for T where T: Iterator {}

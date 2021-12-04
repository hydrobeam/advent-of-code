use std::fs;
use std::str::FromStr;

fn solve_day4_p1() {
    // use day1_2020::solve2020_day1p1;

    // use crate::day2::{solve_day2p1, solve_day2p2};
    let filename = "inputs/day4input.txt";
    let contents = fs::read_to_string(filename).expect("file not found");
    let mut file = contents.lines();
    let moves = &file
        .next()
        .unwrap()
        .split(',')
        .map(|x| i32::from_str(x).unwrap())
        .collect::<Vec<i32>>();

    let temp_boards = file.filter(|x| !x.is_empty()).collect::<Vec<&str>>(); //nuke the whitespace

    let loards = temp_boards.chunks(5).map(|x| {
        x.iter()
            .map(|y| y.split_whitespace())
            .flatten()
            .map(|q| i32::from_str(q).expect(q))
            .collect::<Vec<i32>>()
    });

    let mut real_boards = loards.clone();

    let mut boards = loards.collect::<Vec<Vec<i32>>>();

    // dbg!(is_bingo(&mut (-1..23).collect::<Vec<i32>>()));
    // dbg!(boards.next().unwrap());
    let mut counter = 0;

    'outer: for bongo in moves {
        // dbg!(bongo);
        for item in boards.iter_mut() {
            for element in item.iter_mut() {
                if element == bongo {
                    *element = -1;
                }
            }

            if is_bingo(item) {
                let sum: i32 = item.iter().filter(|&x| x.is_positive()).sum();
                dbg!(item, bongo, counter, sum);
                println!("full job:  {}", sum * bongo);
                break 'outer;
            }
            counter += 1;
        }
        counter = 0;
    }
}

fn solve_day4p2() {
    let filename = "inputs/day4input.txt";
    let contents = fs::read_to_string(filename).expect("file not found");
    let mut file = contents.lines();
    let moves = file
        .next() // first line is the inputs
        .unwrap()
        .split(',') //they are separated by commas
        .map(|x| i32::from_str(x).unwrap())
        .collect::<Vec<i32>>();

    let temp_boards = file.filter(|x| !x.is_empty()).collect::<Vec<&str>>();

    let loards = temp_boards.chunks(5).map(|x| {
        let mut m = x
            .iter()
            .map(|y| y.split_whitespace())
            .flatten()
            .map(|q| i32::from_str(q).expect(q))
            .collect::<Vec<i32>>();
        m.push(-2);
        m // We add this on the end as a way of indicating whether a board has won or not.
          // if a board is a winner, then we flip this final digit to be 0.
          // it does not interfere with the bingo playing.
    });

    let mut boards = loards.collect::<Vec<Vec<i32>>>();

    let mut counter = 0;
    let len_boards = boards.len();
    'outer: for bongo in moves {
        // dbg!(bongo);
        for item in boards.iter_mut() {
            for element in item.iter_mut() {
                if *element == bongo {
                    *element = -1;
                }
            }

            if is_bingo(item) {
                if item.last().expect("shit went wrong").is_negative() {
                    counter += 1;
                    let item_len = item.len();
                    item[item_len - 1] = 0; // 0 wont be caught by is_neg or is_pos
                }

                if counter == len_boards {
                    let sum: i32 = item.iter().filter(|&x| x.is_positive()).sum();
                    dbg!(bongo, counter, sum * bongo);

                    break 'outer;
                }
            }
        }
    }
}

fn is_bingo(vec: &mut Vec<i32>) -> bool {
    //&mut vec beacuse rust yells when it's not that
    let hori_vec = &mut vec.chunks_exact(5);
    // use chunks_exact because the final element of the vec is the win-indicator.
    let mut veri_vec: Vec<Vec<i32>> = Vec::new();

    for a in 0..5 {
        let mut t_vec: Vec<i32> = Vec::new();
        for b in 0..vec.len() {
            if b % 5 == a {
                t_vec.push(vec[b]);
            }
        }
        veri_vec.push(t_vec)
    }

    // diag_vec not needed, oops!
    let mut diag_vec: Vec<Vec<i32>> = Vec::new();
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for i in 0..vec.len() {
        if i == 0 {
            v1.push(vec[i]);
        } else if i == vec.len() - 1 {
            v1.push(vec[i]);
        } else if i % 6 == 0 && i % 4 == 0 {
            v1.push(vec[i]);
            v2.push(vec[i]);
        } else if i % 6 == 0 {
            v1.push(vec[i]);
        } else if i % 4 == 0 {
            v2.push(vec[i])
        }
    }

    diag_vec.push(v1);
    diag_vec.push(v2);

    // let a = diag_vec.iter().any(|x| x.iter().all(|&b| b < 0)); // old diag code

    // if negative then it's marked, if posiive, then not marked.
    let b = hori_vec.any(|z| z.iter().all(|&b| b < 0));
    let c = veri_vec.iter().any(|x| x.iter().all(|&b| b < 0));

    b || c
}

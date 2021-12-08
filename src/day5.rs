use std::str::FromStr;
use std::{fs};

pub fn solve_day5p1() {
    let filename = "inputs/day5input.txt";
    let contents = fs::read_to_string(filename).expect("file not found");
    let file = contents.lines();

    let coords = file
        .map(|x| {
            x.split("->")
                .map(|y| {
                    y.split(',')
                        .map(|i| u32::from_str(i.trim()).expect(i))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .filter(|vec| vec[0][0] == vec[1][0] || vec[0][1] == vec[1][1]);

    // slow!!
    let flatty = coords.clone().flatten().flatten();
    let cols = flatty.clone().step_by(2).max().unwrap() + 1; //x vals
    let rows = flatty.skip(1).step_by(2).max().unwrap() + 1; // y-values

    let mut scan_vec = vec![0; cols as usize * rows as usize];
    // let vec =
    // let max = coords.iter();
    for position in coords {
        let (y_1, y_2) = (position[0][1].min(position[1][1]), position[0][1].max(position[1][1]));
        let (x_1, x_2) = (position[0][0].min(position[1][0]), position[0][0].max(position[1][0]));

        // horizontal
        if y_2 == y_1 {
            let index = y_1 * cols;
            let begin = index + x_1;
            let end = index + x_2;
            for i in begin..=end {
                scan_vec[i as usize] += 1;
            }
        } else if x_2 == x_1 {
            let index = y_1 * cols;
            let begin = index + x_1; // we at 7
            
            let end = (y_2 * cols) + x_2;

            for i in (begin..=end).step_by(cols as usize){
                scan_vec[i as usize] += 1;
            }
        }

    }

    let count = scan_vec.into_iter().filter(|&x| x >= 2).count();
    // for i in 0..scan_vec.len() {
    //     if i%rows as usize == 0 && i != 0 {
    //         println!();
    //     }
    //     print!("{}", scan_vec[i]);



    // }
    // println!("{:#?}", scan_vec.chunks(10));

    dbg!(count);
}

pub fn solve_day5p2() {
    let filename = "inputs/day5input.txt";
    let contents = fs::read_to_string(filename).expect("file not found");
    let file = contents.lines();

    let coords = file.map(|x| {
        x.split("->")
            .map(|y| {
                y.split(',')
                    .map(|i| u32::from_str(i.trim()).expect(i))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
    }).collect::<Vec<Vec<Vec<u32>>>>();

    // slow!!
    let flatty = coords.iter().flatten().flatten().copied().collect::<Vec<u32>>();
    let cols = flatty.iter().step_by(2).max().unwrap() + 1;
    let rows = flatty.iter().skip(1).step_by(2).max().unwrap() + 1;

    let mut scan_vec = vec![0; cols as usize * rows as usize];
    // let vec =
    // let max = coords.iter();
    for position in coords {
        let x_1 = position[0][0];
        let y_1 = position[0][1];
        let x_2 = position[1][0];
        let y_2 = position[1][1];

        if y_2 == y_1 {
            // horizontal
            let index = y_1 * cols;
            let begin = index + x_1.min(x_2);
            let end = index + x_1.max(x_2);

            for i in begin..=end  {
                scan_vec[i as usize] += 1;
            }

        } else if x_2 == x_1 {
            // vertical
            let index = y_1.min(y_2) * cols;
            let begin = index + x_1; 

            let end = (y_1.max(y_2) * cols) + x_2;

            for i in (begin..=end).step_by(cols as usize) {
                scan_vec[i as usize] += 1;
            }
        } else {
            // diagonal
            if (x_1 > x_2 && y_1 > y_2) || (y_2 > y_1 && x_2 > x_1) {
                // when the trajectory is SE/NW

                let min_x = x_1.min(x_2);
                let max_x = x_1.max(x_2);
                let min_y = y_1.min(y_2);
                let max_y = y_1.max(y_2);

                let begin = min_x + cols * min_y;
                let end = max_x + cols * max_y;
                for i in (begin..=end).step_by((cols + 1) as usize) {
                    scan_vec[i as usize] += 1;
                }
            } else {
                // when the trajectory is SW/NE
                let ind_1 = y_1 * cols + x_1;
                let ind_2 = y_2 * cols + x_2;

                for i in (ind_1.min(ind_2)..=ind_1.max(ind_2)).step_by((cols - 1) as usize) {
                    scan_vec[i as usize] += 1;
                }
            }
        }
    }

    let count = scan_vec.into_iter().filter(|&x| x >= 2).count();

    // for i in 0..scan_vec.len() { // func to view table
    //     if i % rows as usize == 0 && i != 0 {
    //         println!();
    //     }
    //     print!("{}", scan_vec[i]);
    // }
    // println!("{:#?}", scan_vec.chunks(10));

    dbg!(count);
}
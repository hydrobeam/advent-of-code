// use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell};
use std::rc::Rc;

// use std::vec;
// use std::io::Error;

pub fn solve_day9p1() {
    let contents = include_str!("../inputs/day9input.txt")
        .lines()
        .map(|x| {
            x.split_terminator("")
                .skip(1)
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    // println!("{:?}", &contents);
    let mut total: i64 = 0;

    let max_cols = contents[0].len() - 1;
    let max_rows = contents.len() - 1;

    let mut i = 0;
    for row_index in 0..contents.len() {
        for col_index in 0..contents[0].len() {
            // println!("{:?}", &contents[row_index]);
            i += 1;
            let val = contents[row_index][col_index];
            // dbg!(&val);

            if col_index == 0 {
                let right = contents[row_index][col_index + 1];
                if row_index == max_rows {
                    // bottom left
                    if val < contents[row_index - 1][col_index] && val < right {
                        // dbg!(&val);
                        total += val + 1;
                    }
                } else if row_index == 0 {
                    // top left
                    if val < contents[row_index + 1][col_index] && val < right {
                        dbg!(&val);
                        total += val + 1;
                    }
                } else {
                    // left
                    if val < contents[row_index + 1][col_index]
                        && val < contents[row_index - 1][col_index]
                        && val < right
                    {
                        dbg!(&val);
                        total += val + 1;
                    }
                }
            } else if col_index == max_cols {
                let left = contents[row_index][col_index - 1];
                if row_index == 0 {
                    // top right
                    if val < contents[row_index + 1][col_index] && val < left {
                        dbg!(&val);
                        total += val + 1;
                    }
                } else if row_index == max_rows {
                    // bottom right
                    if val < contents[row_index - 1][col_index] && val < left {
                        dbg!(&val);
                        total += val + 1;
                    }
                } else {
                    // right
                    if val < contents[row_index - 1][col_index]
                        && val < contents[row_index + 1][col_index]
                        && val < left
                    {
                        total += val + 1;
                    }
                }
            } else if row_index == 0 {
                // top
                let right = contents[row_index][col_index + 1];
                let left = contents[row_index][col_index - 1];
                let down = contents[row_index + 1][col_index];
                dbg!(right, left, down);

                if val < left && val < right && val < down {
                    total += val + 1;
                }
            } else if row_index == max_rows {
                let right = contents[row_index][col_index + 1];
                let left = contents[row_index][col_index - 1];
                let up = contents[row_index - 1][col_index];

                if val < left && val < right && val < up {
                    dbg!(&val);
                    total += val + 1;
                }
            } else {
                let right = contents[row_index][col_index + 1];
                let left = contents[row_index][col_index - 1];
                let up = contents[row_index - 1][col_index];
                let down = contents[row_index + 1][col_index];

                if val < right && val < left && val < up && val < down {
                    dbg!(&val);
                    total += val + 1;
                }
            }
        }
    }
    dbg!(total);
    dbg!(i);
}

pub fn solve_day9p2() {
    let default_basin: Rc<RefCell<u64>> = Rc::new(RefCell::new(0));
    // let swag: RefCell<i64> = RefCell::new(0);
    let txt = include_str!("../inputs/day9input.txt");
    let mut contents = include_str!("../inputs/day9input.txt")
        .lines()
        .map(|x| {
            x.split_terminator("")
                .skip(1)
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| Mountain {
                    height: x.parse::<u64>().unwrap(),
                    in_basin: false,
                    basin_num: Rc::clone(&default_basin),
                })
                .collect::<Vec<Mountain>>()
        })
        .collect::<Vec<Vec<Mountain>>>();

    let mut sum_pointer_vec: Vec<i64> = Vec::new();
    let mut basin_vec: Vec<Rc<RefCell<u64>>> = Vec::new();
    let mut basin_num: usize = 0;

    for row_index in 0..contents.len() {
        for col_index in 0..contents[0].len() {
            let val = &mut contents
                .get(row_index)
                .unwrap()
                .get(col_index)
                .unwrap()
                .clone();

            if val.height == 9 {
                continue;
            }

            let mut neighbours: Vec<Mountain> = Vec::new();
            if row_index > 0 {
                let up = contents[row_index - 1][col_index].clone();
                neighbours.push(up);
            }
            if col_index > 0 {
                let left = contents[row_index][col_index - 1].clone();
                neighbours.push(left);
            }

            let neighbours: Vec<Mountain> = neighbours.into_iter().filter(|x| x.in_basin).collect();

            match neighbours.len() {
                0 => {
                    basin_num += 1;

                    let basin: Rc<RefCell<u64>> = Rc::new(RefCell::new(basin_num as u64));
                    let cellie: i64 = 1;
                    sum_pointer_vec.push(cellie);
                    basin_vec.push(basin);

                    val.make_basin(Rc::clone(&basin_vec[basin_num - 1]));
                    contents[row_index][col_index] = val.clone();
                }
                1 => {
                    val.adapt_basin(&neighbours[0]);
                    contents[row_index][col_index] = val.clone();
                    let taker_num = *val.basin_num.borrow();
                    sum_pointer_vec[taker_num as usize - 1] += 1;
                }
                _ => {
                    let top_basin = *neighbours[0].basin_num.borrow(); // up
                    let left_basin = neighbours[1].clone().basin_num; // left
                    let left_num = *left_basin.clone().borrow();
                    // dbg!(top_basin, left_num);
                    // dbg!(&val);
                    // dbg!(row_index, col_index);
                    // dbg!(&sum_pointer_vec, &basin_vec);
                    // dbg!(&neighbours);
                    if top_basin != left_num {
                        let topper = left_num.max(top_basin);
                        let botter = left_num.min(top_basin);

                        sum_pointer_vec[botter as usize - 1] +=
                            sum_pointer_vec[topper as usize - 1];

                        // if left_num 
                        *left_basin.borrow_mut() = botter;
                        *neighbours[0].basin_num.borrow_mut() = botter;

                        for basin in &basin_vec[topper as usize .. ] {
                                *basin.borrow_mut() -= 1;
                            }
                        sum_pointer_vec.remove(topper as usize - 1);
                        basin_vec.remove(topper as usize - 1);

                        basin_num -= 1;
                    }

                    val.in_basin = true;
                    val.basin_num = Rc::clone(&left_basin);
                    contents[row_index][col_index] = val.clone();
                    sum_pointer_vec[*val.basin_num.borrow() as usize - 1] += 1;

                }
            };
        }
    }
    // print!("{:#?}", contents);
    sum_pointer_vec.sort_unstable_by(|a, b| a.cmp(b));
    dbg!(&sum_pointer_vec[0..3]);

}

#[derive(Debug, Clone)]
struct Mountain {
    height: u64,
    in_basin: bool,
    basin_num: Rc<RefCell<u64>>,
}

impl<'a> Mountain {
    fn make_basin(&mut self, basin_num: Rc<RefCell<u64>>) {
        self.in_basin = true;
        self.basin_num = basin_num;
    }

    fn adapt_basin(&mut self, mountain: &Mountain) {
        self.in_basin = true;
        self.basin_num = Rc::clone(&mountain.basin_num);
        // println!("{:#?}", self.num_members);
    }
}

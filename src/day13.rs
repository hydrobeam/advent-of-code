pub fn solve_day13() {
    let input = include_str!("../inputs/day13input.txt")
        .split_once("\n\n")
        .unwrap();

    let indices = input.0.lines().map(|x| {
        x.split(',')
            .map(|y| y.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    });

    let directions = input.1.lines().map(|x| {
        x.split(' ')
            .nth(2)
            .expect(x)
            .split('=')
            .collect::<Vec<&str>>()
    });

    let (max_x, max_y) = find_max(Box::new(indices.clone().flatten()));
    let mut dot_vec = init_dot_vec(Box::new(indices), max_x, max_y);

    for direction in directions {
        let axis = direction[0];
        let val = direction[1].parse::<usize>().unwrap();
        let y_len = dot_vec.len();
        let x_len = dot_vec[0].len();

        if axis == "x" {
            if val >= x_len / 2 {
                for item in &mut dot_vec {
                    for col in (val + 1)..x_len {
                        if item[col] == 1 {
                            item[(val as isize + (val as isize - col as isize)) as usize] = 1;
                        }
                    }
                }
                for row in &mut dot_vec {
                    *row = row[0..val].to_vec();
                }
            } else {
                for item in &mut dot_vec {
                    for col in 0..val {
                        if item[col] == 1 {
                            item[(val as isize + (val as isize - col as isize)) as usize] = 1;
                        }
                    }
                }

                for row in &mut dot_vec {
                    *row = row[(val + 1)..x_len].to_vec();
                }
            }
        } else {
            for row in (val + 1)..dot_vec.len() {
                for col in 0..dot_vec[0].len() {
                    if dot_vec[row][col] == 1 {
                        dot_vec[(val as isize + (val as isize - row as isize)) as usize][col] = 1;
                    }
                }
            }

            if val >= y_len / 2 {
                dot_vec = dot_vec[0..val].to_vec();
            } else {
                dot_vec = dot_vec[(val + 1)..y_len].to_vec();
            }
        }

        let visible_dots: usize = dot_vec.iter().flatten().sum();
        dbg!(visible_dots);
    }

    for row in dot_vec {
        println!();
        for col in row {
            if col == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
    }
}

fn find_max(indices: Box<dyn Iterator<Item = usize>>) -> (usize, usize) {
    let mut even = false;
    let mut max_x = 0;
    let mut max_y = 0;
    for val in indices {
        if even {
            if val > max_y {
                max_y = val;
            }
            even = false;
        } else {
            if val > max_x {
                max_x = val;
            }
            even = true;
        }
    }

    (max_x, max_y)
}

fn init_dot_vec(
    indices: Box<dyn Iterator<Item = Vec<usize>>>,
    max_x: usize,
    max_y: usize,
) -> Vec<Vec<usize>> {
    // makes the plane we're going to be working with

    let mut dot_vec: Vec<Vec<usize>> = Vec::new();
    for _ in 0..=max_y {
        let zeros = vec![0; max_x + 1];
        dot_vec.push(zeros);
    }

    for coord in indices {
        let x = coord[0];
        let y = coord[1];

        dot_vec[y][x] = 1;
    }
    dot_vec
}

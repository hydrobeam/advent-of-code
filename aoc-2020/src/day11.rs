use std::iter;

pub fn solve_day11() {
    let mut content = include_str!("../inputs/day11_input.txt")
        .trim()
        .split('\n')
        .map(|x| x.trim().chars().map(process_input).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let mut prev_number = 0;
    let mut count = content
        .iter()
        .flatten()
        .filter(|x| match x {
            Tile::Occupied => true,
            _ => false,
        })
        .count();

    let mut c2 = content.clone();
    let mut count2 = count;

    loop {
        let nu_content = hunt1(&content, &mut count);
        if count == prev_number {
            break;
        } else {
            prev_number = count;
            content = nu_content;
        }
    }

    println!("The solution to part 1 is: {}", prev_number);
    prev_number = 0;

    loop {
        let nu_content = hunt2(&c2, &mut count2);
        if count2 == prev_number {
            break;
        } else {
            prev_number = count2;
            c2 = nu_content;
        }
    }

    // debugging: printing out the board
    //
    // let target_coords = (1, 1);
    //
    //    for (y, v1) in c2.iter().enumerate() {
    //        for (x, v2) in v1.iter().enumerate() {
    //            if (x, y) == target_coords {
    //                print!("$");
    //            } else {
    //                print!("{}", v2.to_char());
    //            }
    //        }
    //        print!("\n");
    //    }
    //    print!("\n");
    //}

    println!("The solution to part 2 is: {}", prev_number);
}

/// pretty much just hunt1 except it uses in_vision to find the neighbours,
/// and the dup_count thresehold is 5 for the occupied case
fn hunt2(init_arr: &Vec<Vec<Tile>>, count: &mut usize) -> Vec<Vec<Tile>> {
    let y_max = init_arr.len();
    let x_max = init_arr[0].len();

    let mut nu_arr: Vec<Vec<Tile>> = init_arr.clone();

    for y in 0..y_max {
        for x in 0..x_max {
            match init_arr[y][x] {
                Tile::Occupied => {
                    let neighs = in_vision(&init_arr, x, x_max - 1, y, y_max - 1);
                    let dup_count = neighs
                        .iter()
                        .filter(|x| match x {
                            Tile::Occupied => true,
                            _ => false,
                        })
                        .count();

                    if dup_count >= 5 {
                        nu_arr[y][x] = Tile::Unoccupied;
                        *count -= 1;
                    }
                }
                Tile::Unoccupied => {
                    let neighs = in_vision(&init_arr, x, x_max - 1, y, y_max - 1);
                    let dup_count = neighs
                        .iter()
                        .filter(|x| match x {
                            Tile::Occupied => true,
                            _ => false,
                        })
                        .count();

                    if dup_count == 0 {
                        nu_arr[y][x] = Tile::Occupied;
                        *count += 1;
                    }
                }
                Tile::Floor => {}
            }
        }
    }

    nu_arr
}

/// gets passed
/// - two iterators corresponding
/// to the x and y values we want to check
/// - a mutable reference to the neigh_vec that we can add to
/// - and a reference to the initial 2D array.
/// then we loop through the iterators we are passed and check if each
/// of them are floors or not, i.e. in the line of sight
fn do_thing(
    init_arr: &Vec<Vec<Tile>>,
    counter_arr: &mut Vec<Tile>,
    y_iter: impl Iterator<Item = usize>,
    x_iter: impl Iterator<Item = usize>,
) {
    for (y_val, x_val) in y_iter.zip(x_iter) {
        match init_arr[y_val][x_val] {
            Tile::Floor => continue,
            tile @ Tile::Occupied | tile @ Tile::Unoccupied => {
                counter_arr.push(tile);
                break;
            }
        }
    }
}

/// returns a vector of tiles in the line of sight
/// works with do_thing
fn in_vision(
    init_arr: &Vec<Vec<Tile>>,
    x: usize,
    x_max: usize,
    y: usize,
    y_max: usize,
) -> Vec<Tile> {
    let mut neigh_vec: Vec<Tile> = Vec::new();

    // up
    if y > 0 {
        do_thing(init_arr, &mut neigh_vec, (0..y).rev(), iter::repeat(x));
    }

    //down
    if y < y_max {
        do_thing(init_arr, &mut neigh_vec, (y + 1)..=y_max, iter::repeat(x));
    }

    // left
    if x > 0 {
        do_thing(init_arr, &mut neigh_vec, iter::repeat(y), (0..x).rev());
    }

    //right
    if x < x_max {
        do_thing(init_arr, &mut neigh_vec, iter::repeat(y), (x + 1)..=x_max);
    }

    //UR
    if x < x_max && y > 0 {
        do_thing(init_arr, &mut neigh_vec, (0..y).rev(), (x + 1)..=x_max);
    }

    //UL
    if x > 0 && y > 0 {
        do_thing(init_arr, &mut neigh_vec, (0..y).rev(), (0..x).rev());
    }

    // DL
    if x > 0 && y < y_max {
        do_thing(init_arr, &mut neigh_vec, (y + 1)..=y_max, (0..x).rev());
    }

    // DR
    if x < x_max && y < y_max {
        do_thing(init_arr, &mut neigh_vec, (y + 1)..=y_max, (x + 1)..=x_max);
    }

    neigh_vec
}

/// the act of running one simulation
/// returns the number of occupied seats
/// plus the vector
fn hunt1(init_arr: &Vec<Vec<Tile>>, count: &mut usize) -> Vec<Vec<Tile>> {
    let y_max = init_arr.len();
    let x_max = init_arr[0].len();

    let mut nu_arr: Vec<Vec<Tile>> = init_arr.clone();

    for y in 0..y_max {
        for x in 0..x_max {
            match init_arr[y][x] {
                Tile::Occupied => {
                    let neighs = find_neighbours(x, x_max - 1, y, y_max - 1);
                    let dup_count = neighs
                        .iter()
                        .filter(|x| match init_arr[x.1 as usize][x.0 as usize] {
                            Tile::Occupied => true,
                            _ => false,
                        })
                        .count();

                    if dup_count >= 4 {
                        nu_arr[y][x] = Tile::Unoccupied;
                        *count -= 1;
                    }
                }
                Tile::Unoccupied => {
                    let neighs = find_neighbours(x, x_max - 1, y, y_max - 1);
                    let dup_count = neighs
                        .iter()
                        .filter(|x| match init_arr[x.1 as usize][x.0 as usize] {
                            Tile::Occupied => true,
                            _ => false,
                        })
                        .count();

                    if dup_count == 0 {
                        nu_arr[y][x] = Tile::Occupied;
                        *count += 1;
                    }
                }
                Tile::Floor => {}
            }
        }
    }

    nu_arr
}

fn process_input(val: char) -> Tile {
    match val {
        'L' => Tile::Unoccupied,
        '#' => Tile::Occupied,
        '.' => Tile::Floor,
        _ => panic!("oopsies"),
    }
}

/// finds all 8 possible adjacent neigbours
fn find_neighbours(x: usize, x_max: usize, y: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut neigh_vec: Vec<(usize, usize)> = Vec::new();

    if x > 0 && y > 0 {
        neigh_vec.push((x - 1, y - 1));
    }
    if x > 0 {
        neigh_vec.push((x - 1, y));
    }
    if x > 0 && y < y_max {
        neigh_vec.push((x - 1, y + 1));
    }
    if y < y_max {
        neigh_vec.push((x, y + 1));
    }
    if x < x_max && y < y_max {
        neigh_vec.push((x + 1, y + 1));
    }
    if x < x_max {
        neigh_vec.push((x + 1, y));
    }
    if x < x_max && y > 0 {
        neigh_vec.push((x + 1, y - 1));
    }
    if y > 0 {
        neigh_vec.push((x, y - 1))
    }

    neigh_vec
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Floor,
    Occupied,
    Unoccupied,
}

impl Tile {
    /// debugging tool, for printing out the board
    fn to_char(&self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Occupied => '#',
            Tile::Unoccupied => 'L',
        }
    }
}

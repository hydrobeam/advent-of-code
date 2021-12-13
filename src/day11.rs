use std::collections::HashMap;

pub fn solve_day11() {
    let input = include_str!("../inputs/day11input.txt").lines();

    // create 2d-array;
    let mut contents = input
        .map(|x| {
            x.split_terminator("")
                .skip(1)
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let max_rows = contents.len() - 1;
    let max_cols = contents[0].len() - 1;

    let mut hashie: HashMap<(usize, usize), Octo> = HashMap::new();

    // create hashmap of octopus indices -> octopus
    for row_index in 0..contents.len() {
        for col_index in 0..contents.len() {
            let curr_pos = (row_index, col_index);

            let mut curr_octo: Octo = Octo {
                flashed: false,
                index: curr_pos,
                neighbours: Vec::new(),
            };

            let neighbours = find_neighbours(row_index, col_index, max_rows, max_cols);
            for octopus in neighbours {
                curr_octo.neighbours.push(octopus);
            }

            hashie.insert(curr_pos, curr_octo);
        }
    }

    let mut flashes = 0;
    let mut days = 0;

    loop {
        loop {
            let nines = hashie
                .values_mut()
                .filter(|x| contents[x.index.0][x.index.1] > 9 && !x.flashed)
                .collect::<Vec<&mut Octo>>();
            let len = nines.len();

            if len != 0 {
                for octo in nines {
                    octo.flashed = true;
                    flashes += 1;

                    for index in &octo.neighbours {
                        contents[index.0][index.1] += 1;
                    }
                }
            } else {
                // no more nines and no more potential for butterfly effects
                break;
            }
        }
        contents = contents
            .iter()
            .map(|x| {
                x.iter()
                    .map(|&y| if y > 9 { 0 } else { y })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        // part 2 -> if all the numbers are 0, then we're done
        let sumo: usize = contents
            .iter()
            .map(|n| n.iter().filter(|&&x| x != 0).count())
            .sum();

        if sumo == 0 {
            break;
        }

        // make everyone unflashed, increment day, increment every energy level by 1.
        hashie = hashie.into_iter().map(|(x, y)| (x, y.unflash())).collect();
        days += 1;
        contents = next_day(contents);
    }
    dbg!(days);
    dbg!(flashes);
}

fn next_day(vec: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // increments all the energy numbers by 1
    vec.into_iter()
        .map(|x| x.iter().map(|&b| b + 1).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}

#[derive(Debug)]
struct Octo {
    flashed: bool,
    index: (usize, usize),
    neighbours: Vec<(usize, usize)>,
}

trait Flash {
    fn unflash(self) -> Self;
}

impl Flash for Octo {
    fn unflash(mut self) -> Self {
        self.flashed = false;
        self
    }
}

fn find_neighbours(
    row_index: usize,
    col_index: usize,
    max_rows: usize,
    max_cols: usize,
) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    if row_index > 0 && col_index > 0 {
        // top left
        neighbours.push(((row_index - 1), col_index - 1));
    }
    if row_index > 0 {
        //top
        // dbg!(row_index);
        neighbours.push(((row_index - 1), col_index));
    }
    if row_index > 0 && col_index < max_cols {
        //top right
        neighbours.push(((row_index - 1), col_index + 1));
    }
    if col_index < max_cols {
        //right
        neighbours.push(((row_index), (col_index + 1)));
    }
    if col_index < max_cols && row_index < max_rows {
        // bottom right
        neighbours.push(((row_index + 1), (col_index + 1)));
    }
    if row_index < max_rows {
        // bottom
        neighbours.push(((row_index + 1), (col_index)));
    }
    if row_index < max_rows && col_index > 0 {
        // bottom left
        neighbours.push(((row_index + 1), (col_index - 1)));
    }
    if col_index > 0 {
        // left
        neighbours.push(((row_index), (col_index - 1)));
    }

    neighbours
}

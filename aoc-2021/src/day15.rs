use std::cell::Cell;
use std::collections::HashMap;

fn dijstkra(
    index_mountain_map: &mut HashMap<(usize, usize), Stalagmite>,
    max_cols: usize,
    max_rows: usize,
) {
    // index_mountain_map is the index-> node hashmap

    let cor_node = index_mountain_map.get_mut(&(0, 0)).unwrap();
    cor_node.distance = Cell::new(0);
    cor_node.risk = 0;

    let mut elements_checked = 0;
    let mut index: (usize, usize) = (0, 0);

    while elements_checked < max_cols * max_rows {
        let curr_val = index_mountain_map.get(&index).unwrap();

        let parent_dist = curr_val.distance.get();

        // get valid neighbours (ones that exist in the map/ aren't visited)
        let neighbours = find_neighbours(index, max_cols - 1, max_rows - 1)
            .into_iter()
            .filter_map(|f| match index_mountain_map.get(&f) {
                Some(node) => Some(node),
                None => None,
            });

        for neigh in neighbours {
            let adopted_val = parent_dist + neigh.risk;
            let neigh_dist = &neigh.distance;
            // dbg!(adopted_val);
            if neigh_dist.get() > adopted_val {
                neigh_dist.set(adopted_val);
                neigh.parent_index.set(Some(index));
                // elements_touched += 1;
            }
        }

        curr_val.visited.set(true);
        elements_checked += 1;
        (*index_mountain_map).remove(&index);

        // slow!!
        index = index_mountain_map
            .values()
            .min_by(|x, y| x.distance.get().cmp(&y.distance.get()))
            .unwrap()
            .index;
        if elements_checked % 5000 == 0 {
            // debugging thing
            dbg!(elements_checked);
        }
    }
}

#[derive(Debug, Default)]
struct Stalagmite {
    risk: u64,
    index: (usize, usize),
    distance: Cell<u64>,
    visited: Cell<bool>,
    parent_index: Cell<Option<(usize, usize)>>,
}

fn find_neighbours(
    curr_index: (usize, usize),
    max_cols: usize,
    max_rows: usize,
) -> Vec<(usize, usize)> {
    // retreives valid neighbours in the 2d vector, order of the neighhbours is Up, Down, Left, Right (B A).
    // dbg!(max_rows, max_cols);
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    if curr_index.0 > 0 {
        // up
        neighbours.push((curr_index.0 - 1, curr_index.1));
    }
    if curr_index.0 < max_rows {
        // down
        neighbours.push((curr_index.0 + 1, curr_index.1));
    }
    if curr_index.1 > 0 {
        // left
        neighbours.push((curr_index.0, curr_index.1 - 1));
    }
    if curr_index.1 < max_cols {
        // right
        neighbours.push((curr_index.0, curr_index.1 + 1));
    }
    neighbours
}

fn mod_stuff(num: u64, value: u64) -> u64 {
    // func for expanding the plane
    let temp = (num + value) % 9;
    if temp == 0 {
        return 9;
    }
    temp
}

pub fn solve_day15p2() {
    let input = include_str!("../inputs/day15input.txt").lines();

    let mut grand_vec: Vec<Vec<Vec<u64>>> = Vec::new();
    let times_to_dupl: u64 = 9;
    for row in input {
        let mut holder_vec: Vec<Vec<u64>> = Vec::new();

        let vec_row = row
            .split_terminator("")
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap());
        holder_vec.push(vec_row.clone().collect::<Vec<u64>>());
        for value in 1..times_to_dupl {
            holder_vec.push(
                vec_row
                    .clone()
                    .map(|s| mod_stuff(s, value))
                    .collect::<Vec<u64>>(),
            )
        }
        grand_vec.push(holder_vec);
    }

    let mut contents: Vec<Vec<u64>> = Vec::new();
    let max_rows = grand_vec.len();
    // let max_cols = grand_vec[0][0].len();
    let row_iters = 5;
    let col_iters = 5;
    for row_iter in 0..row_iters {
        for row in 0..max_rows {
            let mut one_unit_vec: Vec<u64> = Vec::new();

            for col_iter in 0..col_iters {
                let mut temp = grand_vec[row][col_iter + row_iter].clone();
                one_unit_vec.append(&mut temp);
                // one_unit_vec.
            }
            contents.push(one_unit_vec);
        }
    }

    let max_rows = contents.len();
    let max_cols = contents[0].len();
    let mut index_mountain_map = create_map(&contents, max_rows, max_cols);

    dijstkra(&mut index_mountain_map, max_cols, max_rows);
    dbg!(&index_mountain_map.values().last().unwrap());
}

fn create_map(
    contents: &Vec<Vec<u64>>,
    max_rows: usize,
    max_cols: usize,
) -> HashMap<(usize, usize), Stalagmite> {
    let mut index_mountain_map: HashMap<(usize, usize), Stalagmite> = HashMap::new();
    // initialize the set of stalagmites with risk values.

    for row in 0..max_rows {
        for col in 0..max_cols {
            if (row, col) == (0, 125) {
                dbg!((row, col));
            }
            index_mountain_map.insert(
                (row, col),
                Stalagmite {
                    risk: contents[row][col],
                    index: (row, col),
                    distance: Cell::new(u64::MAX),
                    ..Default::default() // visited: false,
                },
            );
        }
    }
    // dbg!(index_mountain_map.values().nth(1));
    index_mountain_map
}

fn reverse_path(
    index_mountain_map: &HashMap<(usize, usize), Stalagmite>,
    max_rows: usize,
    max_cols: usize,
) {
    // debugging tool to see the path
    let mut start_node: &Stalagmite = index_mountain_map
        .get(&(max_rows - 1, max_cols - 1))
        .unwrap();
    loop {
        let next_node = start_node.parent_index.get().unwrap();
        println!(
            "{:?} {}",
            next_node,
            index_mountain_map.get(&next_node).unwrap().distance.get() // .unwrap()
        );
        if next_node == (0_usize, 0_usize) {
            break;
        } else {
            start_node = index_mountain_map.get(&next_node).unwrap();
        }
    }
}


pub fn solve_day15p1() {
    let input = include_str!("../inputs/day15input.txt");

    let contents: Vec<Vec<u64>> = input
        .lines()
        .map(|x| {
            x.split_terminator("")
                .skip(1)
                .map(|f| f.parse::<u64>().expect(f))
                .collect::<Vec<u64>>()
        })
        .collect();

    let max_rows: usize = contents.len();
    let max_cols: usize = contents[0].len();

    let mut index_mountain_map: HashMap<(usize, usize), Stalagmite> = create_map(&contents, max_rows, max_cols);

    dijstkra(&mut index_mountain_map, max_cols, max_rows);
    dbg!(&index_mountain_map
        .get(&(max_rows - 1, max_cols - 1))
        .unwrap()
        .distance
        .get());

    dbg!(&index_mountain_map.values().last().unwrap());
    println!("{:#?}", index_mountain_map);
}

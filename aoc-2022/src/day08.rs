pub fn solve() {
    let content = include_str!("../inputs/day08.txt");

    let mut forest: Vec<Vec<Tree>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| Tree::from(char::to_digit(x, 10).unwrap() as u8))
                .collect::<Vec<Tree>>()
        })
        .collect();

    let col_max = forest[0].len() - 1;
    let row_max = forest.len() - 1;

    for row in 0..=row_max {
        for col in 0..=col_max {
            det_visibility(row, col, &mut forest, col_max, row_max)
        }
    }

    let visible = forest.iter().flatten().filter(|tree| tree.visible).count();
    let max_scenic = forest
        .iter()
        .flatten()
        .map(|tree| tree.scenic.iter().product::<usize>())
        .max()
        .unwrap();

    println!("Part 1: {visible}");
    println!("Part 2: {max_scenic}");
}

fn det_visibility(
    row: usize,
    col: usize,
    forest: &mut Vec<Vec<Tree>>,
    col_max: usize,
    row_max: usize,
) {

    if row == 0 || col == 0 {
        // if on left or top
        forest[row][col].visible = true;
    } else if row == row_max || col == col_max {
        // if on bottom or right
        forest[row][col].visible = true;
    } else {
        let us_tree = forest[row][col];
        // check all cols to the left
        let mut truth_arr = [true; 4];

        for col_left in (0..col).rev() {
            if forest[row][col_left].height >= us_tree.height {
                truth_arr[0] = false;
                forest[row][col].scenic[0] = col - col_left;
                break;
            }
        }

        // if the path is clear
        if truth_arr[0] {
            forest[row][col].scenic[0] = col;
        }

        for col_right in (col + 1)..=col_max {
            if forest[row][col_right].height >= us_tree.height {
                truth_arr[1] = false;
                forest[row][col].scenic[1] = col_right - col;
                break;
            }
        }

        if truth_arr[1] {
            forest[row][col].scenic[1] = col_max - col;
        }

        for row_up in (0..row).rev() {
            if forest[row_up][col].height >= us_tree.height {
                truth_arr[2] = false;
                forest[row][col].scenic[2] = row - row_up;
                break;
            }
        }
        if truth_arr[2] {
            forest[row][col].scenic[2] = row;
        }

        for row_down in (row + 1)..=row_max {
            if forest[row_down][col].height >= us_tree.height {
                truth_arr[3] = false;
                forest[row][col].scenic[3] = row_down - row;
                break;
            }
        }
        if truth_arr[3] {
            forest[row][col].scenic[3] = row_max - row;
        }

        forest[row][col].visible = truth_arr.iter().any(|x| *x);
    }

}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u8,
    visible: bool,
    // L, R, D, U
    scenic: [usize; 4],
}

impl From<u8> for Tree {
    fn from(val: u8) -> Self {
        Tree {
            height: val,
            visible: false,
            scenic: [0; 4],
        }
    }
}

use crate::{
    solutions::{AocResult, AocSol, Solution},
    AocError,
};

pub struct Day04;

impl Solution for Day04 {
    fn solve_p1(input: &str) -> AocResult {
        let arr = input.as_bytes();
        let mut lined = input.lines();
        let col_count = lined.next().unwrap().len();
        let row_count = lined.count() + 1;

        let mut row = 0;
        let mut col = 0;
        let mut ind = 0;
        let mut sum = 0;

        // col_count+1 throughout because input has newlines
        let index = |row, col| row * (col_count + 1) + col;
        while ind < arr.len() {
            let chr = arr[index(row, col)];

            if chr == b'X' {
                // up
                sum += (row >= 3
                    && arr[index(row - 1, col)] == b'M'
                    && arr[index(row - 2, col)] == b'A'
                    && arr[index(row - 3, col)] == b'S') as usize;
                // down
                sum += (row <= row_count - 4
                    && arr[index(row + 1, col)] == b'M'
                    && arr[index(row + 2, col)] == b'A'
                    && arr[index(row + 3, col)] == b'S') as usize;
                // left
                sum += (col >= 3
                    && arr[index(row, col - 1)] == b'M'
                    && arr[index(row, col - 2)] == b'A'
                    && arr[index(row, col - 3)] == b'S') as usize;
                // right
                sum += (col <= col_count - 4
                    && arr[index(row, col + 1)] == b'M'
                    && arr[index(row, col + 2)] == b'A'
                    && arr[index(row, col + 3)] == b'S') as usize;
                // NW
                sum += (col >= 3
                    && row >= 3
                    && arr[index(row - 1, col - 1)] == b'M'
                    && arr[index(row - 2, col - 2)] == b'A'
                    && arr[index(row - 3, col - 3)] == b'S') as usize;
                // NE
                sum += (col <= col_count - 4
                    && row >= 3
                    && arr[index(row - 1, col + 1)] == b'M'
                    && arr[index(row - 2, col + 2)] == b'A'
                    && arr[index(row - 3, col + 3)] == b'S') as usize;
                // SE
                sum += (col <= col_count - 4
                    && row <= col_count - 4
                    && arr[index(row + 1, col + 1)] == b'M'
                    && arr[index(row + 2, col + 2)] == b'A'
                    && arr[index(row + 3, col + 3)] == b'S') as usize;
                // SW
                sum += (col >= 3
                    && row <= col_count - 4
                    && arr[index(row + 1, col - 1)] == b'M'
                    && arr[index(row + 2, col - 2)] == b'A'
                    && arr[index(row + 3, col - 3)] == b'S') as usize;
            }
            ind += 1;
            col = ind % (col_count + 1);
            row = ind / (col_count + 1);
        }

        Ok(AocSol::Unsigned(sum))
    }

    fn solve_p2(input: &str) -> AocResult {
        let arr = input.as_bytes();
        let mut lined = input.lines();
        let col_count = lined.next().unwrap().len();
        let row_count = lined.count() + 1;

        let mut row = 0;
        let mut col = 0;
        let mut ind = 0;
        let mut sum = 0;

        // col_count+1 throughout because input has newlines
        let index = |row, col| row * (col_count + 1) + col;

        while ind < arr.len() {
            let chr = arr[index(row, col)];

            // we only look for X-MAS from the topleft
            // pardon the disgusting boolean, it's cool.
            // essentially we verify:
            // - necessary preconditions
            // - A in the middle,
            // - the corners have a correct S/M pair
            sum += (col <= col_count - 3
                && row <= row_count - 3
                && arr[index(row + 1, col + 1)] == b'A'
                && ((arr[index(row + 2, col)] == b'S' && arr[index(row, col + 2)] == b'M')
                    || (arr[index(row + 2, col)] == b'M' && arr[index(row, col + 2)] == b'S'))
                && (
                    // m at NW
                    chr == b'M' && arr[index(row + 2, col + 2)] == b'S'
                    // s at NW
                    || chr == b'S' && arr[index(row + 2, col + 2)] == b'M'
                )) as usize;

            ind += 1;
            col = ind % (col_count + 1);
            row = ind / (col_count + 1);
        }

        Ok(AocSol::Unsigned(sum))
    }
}

// fn index(row: usize, col: usize, row_count: usize, col_count: usize) -> usize {
//     todo!()
// }

pub fn solve_day05() {
    let content: &str = include_str!("../inputs/day05_input.txt");
    let mut highest: u32 = 0;

    let mut boarding_pass_vec: Vec<u32> = Vec::new();

    for line in content.split('\n') {
        let mut counter: u32 = 0;

        let mut row_coord: [u32; 2] = [0, 127];
        let mut col_coord: [u32; 2] = [0, 7];

        for (counter, chr) in line.chars().enumerate() {
            if counter <= 6 {
                match chr {
                    'F' => row_coord[1] = (row_coord[1] + row_coord[0]) / 2, // lower half
                    'B' => row_coord[0] = ((row_coord[1] + row_coord[0]) / 2) + 1, // upper half
                    // basically, [0, 127] => [0, 63], because 127/2 = 63.5 ,and we know it's in the
                    // lower half (and we're dealing with integers), so we let the rounding down happen
                    // what if [0, 128], (128 + 0)/2 = 64, it's in the lower half, so we touch nothing
                    //
                    // however, [0, 127] => [64, 127], because 127/2 = 63.5, and we know it's in the upper half
                    // it's never going to be 63, and the result of the rounding (after finding the mean)
                    // so we round up by adding 1
                    // what if [0, 128]
                    //
                    _ => unreachable!(),
                }
            } else {
                match chr {
                    'L' => col_coord[1] = (col_coord[1] + col_coord[0]) / 2,
                    'R' => col_coord[0] = ((col_coord[1] + col_coord[0]) / 2) + 1,
                    _ => unreachable!(),
                }
            }
        }

        let seat_id: u32 = row_coord[0] * 8 + col_coord[0];
        if seat_id > highest {
            highest = seat_id;
        }

        boarding_pass_vec.push(seat_id) // both [0] and [1] will be the same
    }

    boarding_pass_vec.sort_unstable();

    println!("Part 1 Solution: {}", highest);

    for thing in 0..(boarding_pass_vec.len() - 1) {
        if boarding_pass_vec[thing + 1] - boarding_pass_vec[thing] > 1 {
            println!("Part 2 Solution: {}", boarding_pass_vec[thing] + 1);
        }
    }
}

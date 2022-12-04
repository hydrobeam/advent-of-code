pub fn solve() {
    let content = include_str!("../inputs/day04.txt");

    let sum_pair: (u64, u64) = content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .fold((0, 0), |sum, val| {
            (
                sum.0
                    + ((val[0][0] >= val[1][0] && val[0][1] <= val[1][1])
                        || val[1][0] >= val[0][0] && val[1][1] <= val[0][1])
                        as u64,

                // 2-5,4-8 ✓
                // 4-8,2-3 ⛧
                sum.1
                    + ((val[0][1] >= val[1][0] && val[0][0] <= val[1][1])
                        || (val[1][1] >= val[0][0] && val[1][0] <= val[0][1]))
                        as u64,
            )
        });


    dbg!(sum_pair.0);
    dbg!(sum_pair.1);
}

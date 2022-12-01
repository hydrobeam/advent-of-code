pub fn solve() {
    let content = include_str!("../inputs/day01.txt");

    let elve_calorie_top_3: [u64; 3] = content
        .split("\n\n")
        .map(|var| {
            var.split_terminator('\n')
                .map(|inner| inner.parse::<u64>().expect(inner))
                .sum::<u64>()
        })
        .fold([0, 0, 0], |mut lst, val| {
            if val > lst[0] {
                lst[2] = lst[1];
                lst[1] = lst[0];
                lst[0] = val;
            } else if val > lst[1] {
                lst[2] = lst[1];
                lst[1] = val;
            } else if val > lst[2] {
                lst[2] = val;
            }

            lst
        });

    dbg!(elve_calorie_top_3[0]);
    dbg!(elve_calorie_top_3.iter().sum::<u64>());
}

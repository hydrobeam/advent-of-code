pub fn solve2020_day3p1() {
    let inputs = include_str!("../inputs/day32020inputs.txt").lines();

    let contents = inputs
        .map(|x| x.split_terminator("").skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut rows = 0;
    let mut cols: usize = 0;
    let num_cols = contents[0].len();

    let slope_collections: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tree_total: Vec<i64> = Vec::new();

    for slope in slope_collections {
        let mut trees = 0;
        while rows < contents.len() {
            if contents[rows][cols] == "#" {
                trees += 1;
            }
            rows += slope.1;
            cols += slope.0;
            cols &= num_cols;
        }
        rows = 0;
        cols = 0;
        tree_total.push(trees);
    }

    dbg!(tree_total.iter().product::<i64>());
}

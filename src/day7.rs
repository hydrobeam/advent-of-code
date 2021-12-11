pub fn solve_day7p1() {
    let contents = include_str!("../inputs/day7input.txt");
    let mut hori_pos = contents
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    hori_pos.sort_unstable();
    let goat_key = hori_pos[hori_pos.len() / 2]; // find median from sorted lst
    let ans: i64 = hori_pos.iter().map(|x| (x - goat_key).abs()).sum(); // map and sum
    dbg!(ans, goat_key);

    // first_attempt
    // let mut hashies: HashMap<i64,i64> = HashMap::new();
    // for val in hori_pos.clone() {
    //     *hashies.entry(val).or_insert(0) +=1;
    // };

    // let mut min = i64::MAX;
    // let mut goat_key = 0;
    // for key in hashies.keys() {
    //     let temp:i64 = hori_pos.clone().map(|x| (x-key).abs()).sum();
    //     if temp < min {
    //         min = temp;
    //         goat_key= *key;
    //     }
    // };
    // dbg!(goat_key);
    // dbg!(min);
}

pub fn solve_day7p2() {
    // brute force
    let contents = include_str!("../inputs/day7input.txt");
    let hori_pos = contents.split(',').map(|x| x.parse::<i64>().unwrap());

    let mut min = i64::MAX;
    let mut goat_key = 0;
    for key in 0..hori_pos.clone().max().unwrap() {
        let temp: i64 = hori_pos
            .clone()
            .map(|x| ((x - key).abs()) * ((x - key).abs() + 1) / 2)
            .sum();
        if temp < min {
            min = temp;
            goat_key = key;
        }
    }
    dbg!(goat_key);
    dbg!(min);
}

pub fn solve_day01_p1() {
    let content = include_str!("../inputs/day01_input.txt");
    let operated = content
        .lines()
        .map(|x| (x.parse::<i32>().unwrap() / 3) -2);

    dbg!(operated.sum::<i32>());

}

pub fn solve_day01_p2() {
    let content = include_str!("../inputs/day01_input.txt");
    let operated = content
        .lines()
        .map(|x| fuel_counter(x.parse::<i32>().unwrap()));

    dbg!(operated.sum::<i32>());

}


fn fuel_counter(mut mass: i32) -> i32 {
    let mut sumo: i32 = 0;

    // computes the first fuel
    mass = (mass / 3) - 2;

    while mass > 0 {
        sumo += mass;
        mass = (mass / 3) - 2;
    }
    sumo
}

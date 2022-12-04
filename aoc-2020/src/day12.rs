pub fn solve_day12() {
    let content: Vec<Instruction> = include_str!("../inputs/day12_input.txt")
        .trim()
        .split('\n')
        .map(process_input)
        .collect();

    let mut shp = Ship::new();

    for line in &content {
        // movement
        dbg!(&line);
        match line.direction {
            Direction::N => shp.north += line.value,
            Direction::S => shp.north -= line.value,
            Direction::E => shp.east += line.value,
            Direction::W => shp.east -= line.value,
            Direction::R => shp.facing.increment(line.value),
            Direction::L => shp.facing.increment(-line.value),
            Direction::F => match shp.facing.val {
                0 => shp.east += line.value,
                90 => shp.north -= line.value,
                180 => shp.east -= line.value,
                270 => shp.north += line.value,
                _ => {
                    dbg!(shp.facing.val);
                    panic!("Invalid Degree")
                }
            },
        }
    }

    println!(
        "The solution to part 1 is {}",
        i64::abs(shp.east) + i64::abs(shp.north)
    );

    let mut ship = Ship::new();
    let mut waypoint = Ship::new();
    waypoint.east = 10;
    waypoint.north = 1;

    for line in content {
        match line.direction {
            Direction::N => waypoint.north += line.value,
            Direction::S => waypoint.north -= line.value,
            Direction::E => waypoint.east += line.value,
            Direction::W => waypoint.east -= line.value,
            Direction::R => match line.value {
                90 => {
                    let temp_east = waypoint.east;
                    waypoint.east = waypoint.north;
                    waypoint.north = -temp_east;
                }
                270 => {
                    let temp_east = waypoint.east;
                    waypoint.east = -waypoint.north;
                    waypoint.north = temp_east;
                }
                180 => {
                    waypoint.east = -waypoint.east;
                    waypoint.north = -waypoint.north
                }
                _ => panic!("Invalid rotation"),
            },
            Direction::L => match line.value {
                270 => {
                    let temp_east = waypoint.east;
                    waypoint.east = waypoint.north;
                    waypoint.north = -temp_east;
                }
                90 => {
                    let temp_east = waypoint.east;
                    waypoint.east = -waypoint.north;
                    waypoint.north = temp_east;
                }
                180 => {
                    waypoint.east = -waypoint.east;
                    waypoint.north = -waypoint.north
                }
                _ => panic!("Invalid rotation"),
            },
            Direction::F => {
                ship.east += line.value * waypoint.east;
                ship.north += line.value * waypoint.north;
            }
        }
    }

    println!(
        "The solution to part 2 is {}",
        i64::abs(ship.east) + i64::abs(ship.north)
    );
}

fn process_input(val: &str) -> Instruction {
    let mut iter = val.chars();

    let dir = handle_direction(iter.next().unwrap());
    let val: i64 = iter.collect::<String>().parse::<i64>().unwrap();

    Instruction {
        direction: dir,
        value: val,
    }
}

fn handle_direction(val: char) -> Direction {
    match val {
        'N' => Direction::N,
        'S' => Direction::S,
        'E' => Direction::E,
        'W' => Direction::W,
        'L' => Direction::L,
        'R' => Direction::R,
        'F' => Direction::F,
        _ => panic!("Invalid input"),
    }
}

#[derive(Debug)]
struct Ship {
    facing: Degree, // degrees
    north: i64,
    east: i64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: Degree { val: 0 },
            north: 0,
            east: 0,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i64,
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

#[derive(Debug)]
struct Degree {
    val: i64,
}

impl Degree {
    fn increment(&mut self, increment: i64) {
        self.val = (self.val + increment).rem_euclid(360);
    }
}

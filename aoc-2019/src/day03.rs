use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn solve_day03_p1() {
    let content = include_str!("../inputs/day03_input.txt");
    //let content = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83";
    let wires = content
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(process_input)
                .collect::<Vec<Movement>>()
        })
        .collect::<Vec<Vec<Movement>>>();

    let mut intersection_vec: Vec<u64> = Vec::new();
    let mut big_hamh: HashSet<Position> = HashSet::new();

    let mut wire_choice: u64 = 0;
    for wire_vec in wires {
        let mut curr_pos: Position = Position::new(wire_choice);

        for command in wire_vec {
            let mut counter = 1;

            while counter <= command.magnitude {
                curr_pos.increment_in_direction(&command.direction);
                match big_hamh.get(&curr_pos) {
                    Some(val) => {
                        if val.activated_wire != wire_choice {
                            intersection_vec.push(val.compute_manhattan_distance());
                        }
                    }
                    None => {
                        big_hamh.insert(curr_pos);
                    }
                }
                counter += 1;
            }
        }

        wire_choice += 1;
    }

    dbg!(intersection_vec.iter().min().unwrap());
}

fn process_input(input: &str) -> Movement {
    let mut cars = input.chars();
    let first = cars.next().unwrap();

    Movement {
        direction: match first {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        },
        // convert string of nums to number
        magnitude: cars
            .map(|x| x.to_digit(10).unwrap() as u64)
            .fold(0, |mut acc, x| {
                acc *= 10;
                acc + x
            }),
    }
}

pub fn solve_day03_p2() {

    let content = include_str!("../inputs/day03_input.txt");
    //let content = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83";
    let wires = content
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|y| process_input(y))
                .collect::<Vec<Movement>>()
        })
        .collect::<Vec<Vec<Movement>>>();

    let mut intersection_vec: Vec<u64> = Vec::new();
    let mut big_hamh: HashSet<Position> = HashSet::new();

    let mut wire_choice: u64 = 0;
    for wire_vec in wires {
        let mut curr_pos: Position = Position::new(wire_choice);

        for command in wire_vec {
            let mut counter = 1;

            while counter <= command.magnitude {
                curr_pos.increment_in_direction(&command.direction);
                curr_pos.increment_steps();
                match big_hamh.get(&curr_pos) {
                    Some(val) => {
                        if val.activated_wire != wire_choice {
                            let comb_val = val.steps_hitherto + curr_pos.steps_hitherto;
                            intersection_vec.push(comb_val);
                        }
                    }
                    None => {
                        big_hamh.insert(curr_pos);
                    }
                }
                counter += 1;
            }
        }

        wire_choice += 1;
    }

    dbg!(intersection_vec.iter().min().unwrap());
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    magnitude: u64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
    activated_wire: u64,
    steps_hitherto: u64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Position {
    fn new(wire_choice: u64) -> Self {
        Position {
            x: 0,
            y: 0,
            activated_wire: wire_choice,
            steps_hitherto: 0,
        }
    }

    fn get_pos(self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn increment_in_direction(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn increment_steps(&mut self) {
        self.steps_hitherto += 1;
    }

    fn add_movemenet(&mut self, vector: &Movement) {
        match vector.direction {
            Direction::Up => self.y += vector.magnitude as i64,
            Direction::Down => self.y -= vector.magnitude as i64,
            Direction::Left => self.x -= vector.magnitude as i64,
            Direction::Right => self.x += vector.magnitude as i64,
        }
    }

    fn compute_manhattan_distance(&self) -> u64 {
        (i64::abs(self.x) + i64::abs(self.y)) as u64
    }
}

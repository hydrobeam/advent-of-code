use std::collections::HashSet;

const ROPE_LEN: usize = 10;

pub fn solve() {
    let content = include_str!("../inputs/day09.txt");

    let instructions = content
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();
    let mut visited_1: HashSet<Posn> = HashSet::new();
    let mut visited_2: HashSet<Posn> = HashSet::new();

    // 9 tails, 1 head
    let mut rope = [Posn::new(); ROPE_LEN];

    for instr in instructions {
        match instr.dir {
            Direction::Up => {
                for _ in 1..=instr.val {
                    rope[0].y += 1;
                    for ind in 0..(rope.len() - 1) {
                        let temp_head: Posn = rope[ind];
                        update_tail(&mut rope[ind + 1], temp_head);
                    }
                    visited_1.insert(rope[1]);
                    visited_2.insert(rope[ROPE_LEN - 1]);
                }
            }
            Direction::Down => {
                for _ in 1..=instr.val {
                    rope[0].y -= 1;
                    for ind in 0..(rope.len() - 1) {
                        let temp_head: Posn = rope[ind];
                        update_tail(&mut rope[ind + 1], temp_head);
                    }
                    visited_1.insert(rope[1]);
                    visited_2.insert(rope[ROPE_LEN - 1]);
                }
            }
            Direction::Left => {
                for _ in 1..=instr.val {
                    rope[0].x -= 1;
                    for ind in 0..(rope.len() - 1) {
                        let temp_head: Posn = rope[ind];
                        update_tail(&mut rope[ind + 1], temp_head);
                    }
                    visited_1.insert(rope[1]);
                    visited_2.insert(rope[ROPE_LEN - 1]);
                }
            }
            Direction::Right => {
                for _ in 1..=instr.val {
                    rope[0].x += 1;
                    for ind in 0..(rope.len() - 1) {
                        let temp_head: Posn = rope[ind];
                        update_tail(&mut rope[ind + 1], temp_head);
                    }
                    visited_1.insert(rope[1]);
                    visited_2.insert(rope[ROPE_LEN - 1]);
                }
            }
        }
    }

    println!("Part 1: {}", visited_1.len());
    println!("Part 2: {}", visited_2.len());
}

/// copy head
fn update_tail(tail: &mut Posn, head: Posn) {
    // tail and head can be on the same spot
    // check diagonals
    // New for p2. p2 allows it to be possible that the head can be two diagonals away
    // becase tails can move diagonally (the head can't),so we add this clause
    // to account for that case and move the sub-tail where it needs to go.
    // tried consequently checking with two if statements, but the current logic makes
    // that not very sound. we'd need to remove the abs clause and add a check to see if
    // movement is required (?) since if we move 1, there's a chance it's in the diagonal posiiton
    // which is valid
    if (head.y - tail.y).abs() > 1 && (head.x - tail.x).abs() > 1 {
        tail.x = head.x - (if head.x > tail.x { 1 } else { -1 });
        tail.y = head.y - (if head.y > tail.y { 1 } else { -1 });
    } else if (head.x - tail.x).abs() > 1 {
        // if the x is two units away

        // if head.x is greater, it needs to be left
        // otheriwse, it needs to be right (head.x + 1)

        tail.x = head.x - (if head.x > tail.x { 1 } else { -1 });
        tail.y = head.y;
    } else if (head.y - tail.y).abs() > 1 {
        // if the y is two units away
        tail.y = head.y - (if head.y > tail.y { 1 } else { -1 });
        tail.x = head.x;
    }
    // otherwise, do nothing because they are diagonal
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    val: u32,
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let mut chr = val.split(' ');
        Instruction {
            dir: match chr.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            val: chr.last().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Posn {
    x: isize,
    y: isize,
}

impl Posn {
    fn new() -> Self {
        Posn { x: 0, y: 0 }
    }
}

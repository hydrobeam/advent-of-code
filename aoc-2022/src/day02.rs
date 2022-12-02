pub fn solve() {
    let content = include_str!("../inputs/day02.txt");

    // let content = "A Y\nB X\nC Z\n";

    let ret = content
        .split_terminator('\n')
        .map(|line| {
            let mut chars = line.chars();
            (
                Move::from(chars.next().unwrap()),
                Move::from(chars.last().unwrap()),
            )
        })
        .fold(0, |acc, play| acc + play.1.to_score() + play.1.play(play.0));

    dbg!(ret);

    let ret2 = content
        .split_terminator('\n')
        .map(|line| {
            let mut chars = line.chars();
            let game_move = Move::from(chars.next().unwrap());
            (
                game_move,
                Instruction::from(chars.last().unwrap()).become_move(game_move),
            )
        })
        .fold(0, |acc, play| acc + play.1.to_score() + play.1.play(play.0));

    dbg!(ret2);
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from(var: char) -> Self {
        match var {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => unreachable!(),
        }
    }
    fn to_score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn play(self, other: Move) -> u64 {
        if other == self {
            3
        } else if (self == Move::Rock && other == Move::Scissors)
            || (self == Move::Paper && other == Move::Rock)
            || (self == Move::Scissors && other == Move::Paper)
        {
            6
        } else {
            0
        }
    }
}

enum Instruction {
    Lose,
    Draw,
    Win,
}

impl Instruction {
    fn from(var: char) -> Self {
        match var {
            'X' => Instruction::Lose,
            'Y' => Instruction::Draw,
            'Z' => Instruction::Win,
            _ => unreachable!(),
        }
    }

    fn become_move(self, other: Move) -> Move {
        match other {
            Move::Rock => match self {
                Instruction::Lose => Move::Scissors,
                Instruction::Draw => Move::Rock,
                Instruction::Win => Move::Paper,
            },
            Move::Paper => match self {
                Instruction::Lose => Move::Rock,
                Instruction::Draw => Move::Paper,
                Instruction::Win => Move::Scissors,
            },
            Move::Scissors => match self {
                Instruction::Lose => Move::Paper,
                Instruction::Draw => Move::Scissors,
                Instruction::Win => Move::Rock,
            },
        }
    }
}

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
        if self.draw() == other {
            3
        } else if self.you_beat() == other {
            6
        } else {
            0
        }
    }

    /// returns the item that you lose to
    fn beats_you(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    fn draw(self) -> Move {
        self
    }
    fn you_beat(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
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
        match self {
            Instruction::Lose => other.you_beat(),
            Instruction::Draw => other.draw(),
            Instruction::Win => other.beats_you(),
        }
    }
}

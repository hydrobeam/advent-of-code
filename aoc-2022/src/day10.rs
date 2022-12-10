const TARGET_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const CRT_CYCLES: [usize; 6] = [40, 80, 120, 160, 200, 240];

pub fn solve() {
    let content: &'static str = include_str!("../inputs/day10.txt");

    let instructions = content.lines().map(process);
    let mut tot_signal_strength: isize = 0;
    let mut register: isize = 1;
    let mut cycle: usize = 0;

    let mut crt = [['.'; 40]; 6];

    for instr in instructions {
        draw_crt(&mut crt, cycle, register);
        match instr {
            Instruction::Add(num) => {
                cycle += 1;
                calc_signal_strength(&mut tot_signal_strength, register, cycle);
                draw_crt(&mut crt, cycle, register);
                cycle += 1;
                register += num;
            }
            Instruction::Noop => {
                cycle += 1;
            }
        }
        calc_signal_strength(&mut tot_signal_strength, register, cycle);
    }

    let mut output_str: String = String::with_capacity(CRT_CYCLES[0] * CRT_CYCLES.len());
    for row in crt {
        for item in row {
            output_str.push(item);
        }
        output_str.push('\n');
    }

    println!("Part 1: {tot_signal_strength}");
    println!("Part 2: \n{output_str}");
}

#[inline]
fn calc_signal_strength(tot_signal_strength: &mut isize, register: isize, cycle: usize) {
    if TARGET_CYCLES.iter().any(|val| *val == cycle) {
        *tot_signal_strength += register * cycle as isize;
    }
}

#[inline]
fn draw_crt(crt: &mut [[char; 40]; 6], cycle: usize, register: isize) {
    let col_ind = cycle % CRT_CYCLES[0];
    let row_ind = cycle / CRT_CYCLES[0];
    let pixel = col_ind as isize;

    if register == pixel || register - 1 == pixel || register + 1 == pixel {
        crt[row_ind][col_ind] = '@';
    }
}

fn process(line: &str) -> Instruction {
    // parsing approach adapted from https://stackoverflow.com/a/50510152

    let mut split = line.split(' ');
    let first = split.next().expect("Instruction must exist");
    let second = split.next();

    if let "noop" = first {
        Instruction::Noop
    } else {
        Instruction::Add(second.unwrap().parse().unwrap())
    }
}

#[derive(Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

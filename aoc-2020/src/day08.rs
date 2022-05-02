use std::str::FromStr;

pub fn solve_day08() {
    let content: &str = include_str!("../inputs/day08_input.txt");
    let mut vals = content
        .split_terminator('\n')
        .map(process_input)
        .collect::<Vec<Op>>();

    let copy = vals.clone().to_owned();
    let res1 = run_until_first_dup(&mut vals, 0, 0);
    let res2 = run_until_term(copy);

    dbg!(res1);
    dbg!(res2);
}

fn run_until_term(og_op_list: Vec<Op>) -> i32 {
    let mut instruction_counter = 0;
    let mut accumulator = 0;
    let mut visited_vec: Vec<(i32, Op)> = Vec::new();

    let mut copy = og_op_list.clone().to_owned();
    let mut copy2 = og_op_list.clone().to_owned();

    loop {
        if instruction_counter < 0 {
            // can't index hte ist with a negative
            panic!("invalid_program: instruction counter index < 0");
        }

        let curr_op = &mut copy[instruction_counter as usize];
        //println!("times_hit: {}, instruction_counter: {}",curr_op.times_hit, instruction_counter);
        if curr_op.times_hit == 1 {
            visited_vec.push((instruction_counter, *(curr_op.clone().reset_visits())));
        } else if curr_op.times_hit >= 2 {
            break;
        }

        match curr_op.op_type {
            Operation::Nop => instruction_counter += 1,
            Operation::Acc => {
                accumulator += curr_op.value;
                instruction_counter += 1
            }
            Operation::Jmp => instruction_counter += curr_op.value,
        }
        (*curr_op).visit();
    }

    let desired_length = og_op_list.len();
    'outer: for mut op in visited_vec {
        match op.1.op_type {
            Operation::Acc => {
                continue;
            }
            _ => {
                op.1.flip_type();
            }
        }

        copy2[op.0 as usize] = op.1;
        dbg!(op.1);

        'inner: loop {
            if instruction_counter < 0 {
                // can't index hte ist with a negative
                panic!("invalid_program: instruction counter index < 0");
            }

            if instruction_counter as usize >= desired_length {
                break 'outer;
            }
            let curr_op = &mut copy2[instruction_counter as usize];
            //println!("times_hit: {}, instruction_counter: {}",curr_op.times_hit, instruction_counter);
            if curr_op.times_hit == 2 {
                break 'inner;
            }

            match curr_op.op_type {
                Operation::Nop => instruction_counter += 1,
                Operation::Acc => {
                    accumulator += curr_op.value;
                    instruction_counter += 1
                }
                Operation::Jmp => instruction_counter += curr_op.value,
            }
            (*curr_op).visit();
        }

        copy2 = og_op_list.clone().to_owned();
        accumulator = 0;
        instruction_counter = 0;
    }

    accumulator
}

/// pretty much what the name says
/// vector will have its elements' "times_hit" parameter modified
fn run_until_first_dup(
    op_list: &mut Vec<Op>,
    mut accumulator: i32,
    mut instruction_counter: i32,
) -> i32 {
    loop {
        if instruction_counter < 0 {
            // can't index hte ist with a negative
            panic!("invalid_program: instruction counter index < 0");
        }

        let curr_op = &mut op_list[instruction_counter as usize];
        //println!("times_hit: {}, instruction_counter: {}",curr_op.times_hit, instruction_counter);
        if curr_op.times_hit > 0 {
            break;
        }

        match curr_op.op_type {
            Operation::Nop => instruction_counter += 1,
            Operation::Acc => {
                accumulator += curr_op.value;
                instruction_counter += 1
            }
            Operation::Jmp => instruction_counter += curr_op.value,
        }
        (*curr_op).visit();
    }

    accumulator
}

fn process_input(val: &str) -> Op {
    let group = val.split_once(' ').unwrap();

    let ret_op: Operation;
    match group.0 {
        "nop" => ret_op = Operation::Nop,
        "acc" => ret_op = Operation::Acc,
        "jmp" => ret_op = Operation::Jmp,
        _ => unreachable!(),
    }
    let value: i32 = i32::from_str(group.1).expect("Integer conversion failed");
    Op::new(ret_op, value)
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy)]
struct Op {
    op_type: Operation,
    value: i32,
    times_hit: usize,
}

impl Op {
    fn new(op_type: Operation, value: i32) -> Op {
        Op {
            op_type,
            value,
            times_hit: 0,
        }
    }

    fn visit(&mut self) {
        self.times_hit += 1;
    }

    fn reset_visits(&mut self) -> &mut Self {
        self.times_hit = 0;
        self
    }

    fn flip_type(&mut self) {
        match self.op_type {
            Operation::Jmp => {
                self.op_type = Operation::Nop;
            }
            Operation::Nop => self.op_type = Operation::Jmp,
            _ => {}
        }
    }
}

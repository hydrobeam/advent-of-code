pub fn solve() {
    let content = include_str!("../inputs/day05.txt");

    // {stack_info}
    // 1   2   3
    //
    // instructions_start...

    // so we split on double newline to get our stack info and instruction
    let index = content.find("\n\n").unwrap();
    let (string_stacks, instructions) = content.split_at(index);

    // find the number of stacks
    // reverse the stack_string because the number of stacks is the last line
    // also helps when populating since we can just push if we're working bottom up
    let mut iter_stacks = string_stacks.lines().rev();
    let num_stacks = iter_stacks.next().unwrap().split("   ").count();

    let mut stack_vecs_1: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    // populate the stacks
    for line in iter_stacks {
        let mut count = 0;
        let char_vec: Vec<char> = line.chars().collect();
        for ind in (0..line.len()).step_by(4) {
            if char_vec[ind] == '[' {
                stack_vecs_1[count].push(char_vec[ind + 1]);
            }
            count += 1;
        }
    }

    let mut stack_vecs_2 = stack_vecs_1.clone();

    let instrs = instructions.trim().lines().map(Instruction::from);
    for instruction in instrs {
        // part1
        // flip order of crates
        for _ in 0..instruction.num_crates {
            let temp = stack_vecs_1[instruction.left - 1].pop().unwrap();
            stack_vecs_1[instruction.right - 1].push(temp);
        }

        // part 2
        // move without changing order
        let lstack = &mut stack_vecs_2[instruction.left - 1];
        let mut moving_vec = lstack.split_off(lstack.len() - instruction.num_crates);
        stack_vecs_2[instruction.right - 1].append(&mut moving_vec);
    }

    println!(
        "Part 1: {}",
        stack_vecs_1
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    );
    println!(
        "Part 2: {}",
        stack_vecs_2
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    );

}

#[derive(Debug)]
struct Instruction {
    num_crates: usize,
    left: usize,
    right: usize,
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let thin = val.split(' ').collect::<Vec<&str>>();

        let num_crates = thin[1].parse().unwrap();
        let left = thin[3].parse().unwrap();
        let right = thin[5].parse().unwrap();

        Instruction {
            num_crates,
            left,
            right,
        }
    }
}

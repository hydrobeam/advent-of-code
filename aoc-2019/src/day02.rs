pub fn solve_day02_p1() {
    let tmp = include_str!("../inputs/day02_input.txt");
    //let tmp = "1,9,10,3,2,3,11,0,99,30,40,50";

   let mut content = tmp
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    content[1] = 12;
    content[2] = 2;

    let mut instruction_counter: usize = 0;
    let mut curr_instruction: usize = content[instruction_counter];

    loop {

        if curr_instruction == 99 {
            break;
        }
        else if curr_instruction == 1 || curr_instruction == 2 {

            let addr1 = content[instruction_counter + 1];
            let addr2 = content[instruction_counter + 2];
            let addr3 = content[instruction_counter + 3];

            let val1 = content[addr1];
            let val2 = content[addr2];

            if curr_instruction == 1 {
                content[addr3] = val1 + val2;
            } else {
                content[addr3] = val1 * val2;
            }


        } else {
            panic!(
                "something went wrong: instruction {} at addr: {}",
                curr_instruction, instruction_counter
            );
        }

        instruction_counter += 4;
        curr_instruction = content[instruction_counter];
    }


    dbg!(&content[0]);
}

pub fn solve_day02_p2() {
    let tmp = include_str!("../inputs/day02_input.txt");
    //let tmp = "1,9,10,3,2,3,11,0,99,30,40,50";

    let mut main_content = tmp
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for noun in (0..=99) {
        for verb in (0..=99) {
            let mut content = main_content.clone();
            content[1] = noun;
            content[2] = verb;

            let mut instruction_counter: usize = 0;
            let mut curr_instruction: usize = content[instruction_counter];

            loop {
                if curr_instruction == 99 {
                    break;
                } else if curr_instruction == 1 || curr_instruction == 2 {
                    let addr1 = content[instruction_counter + 1];
                    let addr2 = content[instruction_counter + 2];
                    let addr3 = content[instruction_counter + 3];

                    let val1 = content[addr1];
                    let val2 = content[addr2];

                    if curr_instruction == 1 {
                        content[addr3] = val1 + val2;
                    } else {
                        content[addr3] = val1 * val2;
                    }
                } else {
                    panic!(
                        "something went wrong: instruction {} at addr: {}",
                        curr_instruction, instruction_counter
                    );
                }

                instruction_counter += 4;
                curr_instruction = content[instruction_counter];
            }
            if content[0] == 19690720 {
                dbg!(100 * noun + verb);
            }
        }
    }
}

pub fn solve_day10() {
    let mut content = include_str!("../inputs/day10_input.txt")
        .trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    content.push(0);
    content.sort_unstable();
    content.push(content[content.len() -1] + 3);

    let mut three_count = 0;
    let mut one_count = 0;
    let mut dist_vec: Vec<u64> = Vec::with_capacity(content.len());

    for group in content.windows(2) {
        match group[1] - group[0] {
            3 => {
                three_count += 1;
                dist_vec.push(3)
            }
            1 => {
                one_count += 1;
                dist_vec.push(1)
            }
            _ => {}
        }
    }

    println!("The solution to part 1: {}", three_count * one_count);

    let mut solutions: u64 = 1;

    // tracks how many consecutuive ones we've run into within dist_vec
    let mut running_sum = 0;

    for val in dist_vec {
        if val == 1 {
            running_sum += 1;
        } else {
            if running_sum > 0 {
                // find the sum of the numbers from 1 to running sum:
                // (n)(n+1) / 2
                // then subtract (n - 1)
                //
                // as for why this works, i have no idea, found through trial and error. 
                solutions *= (running_sum * (running_sum + 1) / 2) - (running_sum - 1);
                running_sum = 0;
            }
        }
    }

    println!("The solution to part 2 is: {}", solutions);

    // worked throgh in ../day10_thinking/test.txt
    // then i was looking for a formula that would produce this mapping
    //
    // the table
    // 1-> 1
    // 2 -> 2
    // 3 -> 4
    // 4 -> 7
    // 5 -> 11
    // 6 -> 16
    // 7 -> 22
}

// thinking process
// 
//fn rec_func(num: u64, acc: u64) -> u64 {
//    if num == 1 {
//        acc
//    } else {
//        return acc + rec_func(num - 1, acc);
//    }
//
//    // 1 + 6 + 5 + 4 + 3 + 2 + 1
//    // 4 + 3 + 4 + 5 + 6
//    // 11 + 5  +6
//    // 22
//    //
//    // n(n +1)
//    // 1 + 2 + 3  + 4 + 5 + 6 + 7
//    // 6 + 4 + 5 + 6 + 7
//    // 16 + 5 + 7
//    // 28
//    // ((n-1))n  + 1
//    // o
//    //
//    //
//    // 6
//    // 1 + 2 + 3 + 4 + 5 + 6
//    // 6 + 4 + 5 + 6
//    // 12 + 4 + 5
//    // 21
//    //
//    // n(n+1) - (n - 1)
//}

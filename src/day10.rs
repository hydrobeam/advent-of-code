pub fn solve_day10p1() {
    // syntax checker highscore breaker
    let input = include_str!("../inputs/day10input.txt");

    let contents = input.lines().map(|x| x.split_terminator("").skip(1));
    let mut total: u64 = 0;
    let beginners = ["<", "[", "(", "{"];
    let closers = [">", "]", ")", "}"];
    let vals = [25137, 57, 3, 1197];

    for line in contents {
        let mut str_vec: Vec<&str> = Vec::new();

        'outer: for character in line {
            if closers.contains(&character) {
                let ind = closers.iter().position(|&x| x == character).unwrap();
                if str_vec[str_vec.len() - 1] != beginners[ind] {
                    total += vals[ind];
                    break 'outer;
                } else {
                    str_vec.pop();
                }
            } else {
                str_vec.push(character);
            }
        }
    }

    dbg!(total);
}

pub fn solve_day10p2() {
    // syntax checker highscore breaker
    let input = include_str!("../inputs/day10input.txt");
    let contents = input.lines().map(|x| x.split_terminator("").skip(1));

    let beginners = ["<", "[", "(", "{"];
    let closers = [">", "]", ")", "}"];
    let vals = [4, 2, 1, 3];

    let mut scores: Vec<u64> = Vec::new();

    for line in contents {
        let mut str_vec: Vec<&str> = Vec::new();
        let mut incomplete: bool = true;
        'outer: for character in line.clone() {
            if closers.contains(&character) {
                let ind = closers.iter().position(|&x| x == character).unwrap();
                if str_vec[str_vec.len() - 1] != beginners[ind] {
                    incomplete = false;
                    break 'outer;
                } else {
                    str_vec.pop();
                }
            } else {
                str_vec.push(character);
            }
        }
        if incomplete {
            let mut partial_total: u64 = 0;

            for character in str_vec.into_iter().rev() {
                let ind = beginners.iter().position(|&x| x == character).unwrap();
                partial_total *= 5;
                partial_total += vals[ind];
            }
            scores.push(partial_total);
        }
    }

    scores.sort_unstable();
    dbg!(scores[scores.len() / 2]);
}

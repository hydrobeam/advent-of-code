use std::collections::HashSet;

pub fn solve() {
    let content = include_str!("../inputs/day06.txt");

    let content_vec: Vec<char> = content.chars().collect();

    let marker_count_1 = 4;
    let marker_count_2 = 14;
    // det_1 determines whether 1 has been found
    // only det_1 is needed because 4 non duplicate elements must occur before we get 14
    let mut det_1 = false;


    let mut sol_1: usize = 0;
    let mut sol_2: usize = 0;

    for count in marker_count_1..content_vec.len() {
        if !det_1 {
            if detect_dup(
                &content_vec[(count - (marker_count_1 - 1))..=count],
                marker_count_1,
            ) {
                det_1 = true;
                sol_1 = count + 1;
            }
        }

        if count > marker_count_2 - 1 {
            if detect_dup(
                &content_vec[(count - (marker_count_2 - 1))..=count],
                marker_count_2,
            ) {
                sol_2 = count + 1;
                break;
            }
        }
    }

    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
}

#[inline]
fn detect_dup(list: &[char], past: usize) -> bool {
    let temp_set: HashSet<char> = list.iter().copied().collect();
    temp_set.len() == past
}

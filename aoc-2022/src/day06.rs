pub fn solve() {
    let content = include_str!("../inputs/day06.txt");

    let content_vec: Vec<char> = content.chars().collect();

    let marker_count_1 = 4;
    let marker_count_2 = 14;

    for count in 0..content_vec.len() {
        if count > marker_count_1 - 1 {
            let mut temp_vec = content_vec[(count - (marker_count_1 - 1))..=count].to_owned();
            temp_vec.sort_unstable();
            temp_vec.dedup();
            if temp_vec.len() == marker_count_1 {
                dbg!(count + 1);
                break;
            }
        }
    }

    for count in 0..content_vec.len() {
        if count > marker_count_2 - 1 {
            let mut temp_vec = content_vec[(count - (marker_count_2 - 1))..=count].to_owned();
            temp_vec.sort_unstable();
            temp_vec.dedup();
            if temp_vec.len() == marker_count_2 {
                dbg!(count + 1);
                break;
            }
        }
    }
}

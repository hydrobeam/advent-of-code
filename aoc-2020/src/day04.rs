pub fn solve2020_day4p1() {
    let input = include_str!("../inputs/day42020inputs.txt");

    let contents = input
        .split("\n\n")
        .map(|x| {
            x.lines() // split by newline
                .map(|y| {
                    y.split(' ') // separate by spaces (indicates each property)
                        .map(|l| {
                            l.split(':') // split left and right, left is field, right is val;
                                .collect::<Vec<&str>>()
                        })
                        .filter(|a| {
                            if a[0] == "byr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => (1920..=2002).contains(&val),
                                    Err(_) => false,
                                }
                            } else if a[0] == "iyr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => (2010..=2020).contains(&val),
                                    Err(_) => false,
                                }
                            } else if a[0] == "eyr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => (2020..=2030).contains(&val),
                                    Err(_) => false,
                                }
                            } else if a[0] == "hgt" {
                                let val = a[1].split_once("i");

                                match val {
                                    Some(val) => match (val.0).parse::<i64>() {
                                        Ok(value) => (59..=79).contains(&value),
                                        Err(_) => false,
                                    },
                                    None => match a[1].split_once("c") {
                                        Some(value) => match (value.0).parse::<i64>() {
                                            Ok(value_3) => (150..=193).contains(&value_3),
                                            Err(_) => false,
                                        },
                                        None => false,
                                    },
                                }
                            } else if a[0] == "hcl" {
                                let mut valid = true;
                                let mut text = a[1].chars();
                                if text.next().unwrap() == '#' {
                                    // let besto = text.collect::<Vec<char>>();
                                    for character in text {
                                        let hot = character.to_string().parse::<i64>();
                                        let chorley = match hot {
                                            Ok(val) => (0..=9).contains(&val),
                                            Err(_) => ('a'..='f').contains(&character)
                                        };

                                        if !chorley {
                                            valid = false;
                                        }
                                    }
                                } else {
                                    valid = false;
                                }

                                valid
                            } else if a[0] == "ecl" {
                                matches!(a[1], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                            } else if a[0] == "pid" {
                                let mut valid = true;
                                let vector = a[1].chars().collect::<Vec<char>>();

                                if vector.len() == 9 {
                                    for character in vector {
                                        let chorley = character.to_string().parse::<i64>().is_ok();

                                        if !chorley {
                                            valid = false;
                                        }
                                    }
                                } else {
                                    valid = false;
                                }

                                valid
                            } else {
                                false
                            }
                        })
                        .collect::<Vec<Vec<&str>>>()
                })
                .flatten()
                .count()
            // .collect::<Vec<Vec<&str>>>()
        })
        .filter(|&val| val == 7);
    // .collect::<Vec<Vec<Vec<&str>>>>();

    dbg!(contents.count());
}

pub fn solve2020_day4p1() {
    let input = include_str!("../inputs/day42020inputs.txt");

    let contents = input
        .split("\n\n")
        .map(|x| {
            x.lines() // split by newline
                .map(|y| {
                    y.split(' ') // separate by spaces (indicates each property)
                        .map(|l| {
                            l.split(":") // split left and right, left is field, right is val;
                                .collect::<Vec<&str>>()
                        })
                        .filter(|a| {
                            if a[0] == "byr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => val >= 1920 && val <= 2002,
                                    Err(_) => false,
                                }
                            } else if a[0] == "iyr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => val >= 2010 && val <= 2020,
                                    Err(_) => false,
                                }
                            } else if a[0] == "eyr" {
                                let val = a[1].parse::<i64>();
                                match val {
                                    Ok(val) => val >= 2020 && val <= 2030,
                                    Err(_) => false,
                                }
                            } else if a[0] == "hgt" {
                                let val = a[1].split_once("i");

                                match val {
                                    Some(val) => match (val.0).parse::<i64>() {
                                        Ok(value) => value >= 59 && value <= 79,
                                        Err(_) => false,
                                    },
                                    None => match a[1].split_once("c") {
                                        Some(value) => match (value.0).parse::<i64>() {
                                            Ok(value_3) => value_3 >= 150 && value_3 <= 193,
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
                                            Ok(val) => val >= 0 && val <= 9,
                                            Err(_) => character >= 'a' && character <= 'f',
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
                                match a[1] {
                                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                                    _ => false,
                                }
                            } else if a[0] == "pid" {
                                let mut valid = true;
                                let vector = a[1].chars().collect::<Vec<char>>();

                                if vector.len() == 9 {
                                    for character in vector {
                                        let chorley = match character.to_string().parse::<i64>() {
                                            Ok(_) => true,
                                            Err(_) => false,
                                        };

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

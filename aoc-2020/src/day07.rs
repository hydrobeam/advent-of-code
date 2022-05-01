use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str;
use std::str::FromStr;

pub fn solve_day07() {
    let content: &str = include_str!("../inputs/day07_input.txt");
    let mut finale = content
        .split('\n')
        .map(process_input)
        .collect::<HashMap<&str, Line>>();

    let mut good_set: HashSet<&str> = HashSet::with_capacity(finale.len());
    let mut bad_set: HashSet<&str> = HashSet::with_capacity(finale.len());

    for lin in finale.keys() {
        hunt(lin, &mut bad_set, &mut good_set, &finale);
    }

    dbg!(good_set.len());
    dbg!(hunt2("shiny gold", &finale,0));
}

fn hunt<'a>(
    the_name: &'a str,
    bad_set: &mut HashSet<&'a str>,
    good_set: &mut HashSet<&'a str>,
    total_set: &'a HashMap<&'a str, Line>,
) -> bool {
    match bad_set.contains(the_name) {
        true => false,
        false => {
            match good_set.contains(the_name) {
                true => true,
                // start checking
                false => {
                    let the_line = total_set.get(the_name).unwrap();
                    let mut loop_ret: bool = false;
                    for val in &the_line.bag_holdings {
                        match val.bag_name {
                            Some("shiny gold") => {
                                loop_ret = true;
                                break;
                            }
                            _ => match val.quantity {
                                0 => {}
                                _ => {
                                    match hunt(val.bag_name.unwrap(), bad_set, good_set, total_set)
                                    {
                                        true => {
                                            (*good_set).insert(val.bag_name.unwrap());
                                            loop_ret = true;
                                            break;
                                        }
                                        false => {
                                            (*bad_set).insert(val.bag_name.unwrap());
                                            loop_ret = false;
                                        }
                                    }
                                }
                            },
                        }
                    }
                    if loop_ret == true {
                        (*good_set).insert(the_name);
                    } else {
                        (*bad_set).insert(the_name);
                    }
                    return loop_ret;
                }
            }
        }
    }
}

fn hunt2<'a>(the_name: &str, total_set: &'a HashMap<&'a str, Line>, mut increment: usize) -> usize {

    let the_line = total_set.get(the_name).unwrap();
    for val in &the_line.bag_holdings {
        match val.quantity {
            0 => {},
            _ => {
                // it's 1 because we're starting with at least one bag that must be added
                increment += val.quantity * hunt2(val.bag_name.unwrap(), total_set, 1);
            }
        }
    }

    increment

}

fn process_input<'a>(var: &str) -> (&str, Line) {
    let temp = var.split(" contain ").collect::<Vec<&str>>();
    //let bag_type = temp[0].split(' ').collect::<Vec<&str>>();
    let bag_holdings = temp[1]
        .split(',')
        .map(|x| make_quant(x.trim()))
        .collect::<Vec<QuantLine>>();

    (
        drop_bag(temp[0]),
        Line {
            bag_name: drop_bag(temp[0]), // will never panic
            bag_holdings,
        },
    )
}

fn drop_bag(var: &str) -> &str {
    let byt: &[u8] = var.as_bytes();

    let mut to_chop: usize = 0;

    match byt[0] as char {
        '1' => to_chop = 4,
        'n' => to_chop = 0,
        _ => to_chop = 5,
    }

    str::from_utf8(&byt[0..(var.len() - to_chop)]).unwrap()
}

/// insert trimmed stuff in here
/// drops the bag/bags off the end of a word line
/// option in case there is "no bag"
fn make_quant(var: &str) -> QuantLine {
    let byt: &[u8] = var.as_bytes();

    let mut to_chop: usize = 0;

    match byt[0] as char {
        '1' => to_chop = 4,
        'n' => to_chop = 0,
        _ => to_chop = 5,
    }

    match to_chop {
        0 => QuantLine {
            bag_name: None,
            quantity: 0,
        },
        _ => {
            let bag_dropped: (&str, &str) = str::from_utf8(&byt[0..(var.len() - to_chop)])
                .unwrap()
                .split_once(' ')
                .unwrap();

            let bag_num = usize::from_str(bag_dropped.0).unwrap();
            let bag_name = bag_dropped.1;

            QuantLine {
                bag_name: Some(bag_name.trim()),
                quantity: bag_num,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Line<'a> {
    bag_name: &'a str,
    bag_holdings: Vec<QuantLine<'a>>,
}

#[derive(Debug, Clone)]
struct QuantLine<'a> {
    bag_name: Option<&'a str>, // Option in case of none
    quantity: usize,
}

impl Hash for Line<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bag_name.hash(state);
    }
}

impl Eq for Line<'_> {}

impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.bag_name == other.bag_name
    }
}

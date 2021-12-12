use std::collections::HashMap;

static mut TOTAL: u64 = 0;

pub fn solve_day12() {
    let input = include_str!("../inputs/day12input.txt").lines();

    let mut hashie: HashMap<&str, Junction> = HashMap::new();

    for line in input {
        let mut line = line.split('-');

        let before = line.next().unwrap();
        let after = line.next().unwrap();

        let a = hashie.entry(before).or_insert(Junction {
            name: before,
            connections: Vec::new(),
            size: {
                if before == before.to_uppercase() {
                    Size::Big
                } else {
                    Size::Small
                }
            },
            visits: 0,
        });
        a.connections.push(after);
        let dos = hashie.entry(after).or_insert(Junction {
            name: after,
            connections: Vec::new(),
            size: {
                if after == after.to_uppercase() {
                    Size::Big
                } else {
                    Size::Small
                }
            },
            visits: 0,
        });
        dos.connections.push(before);
    }

    let start = hashie.get_mut("start").unwrap();
    start.visits += 1;

    hunt(start.clone(), hashie, true);
    unsafe { dbg!(TOTAL) };
}

fn hunt(node: Junction, hoshie: HashMap<&str, Junction>, second_junc_active: bool) {
    for noodle in &node.connections {
        let mut new_hash = hoshie.clone();
        let nu_node = new_hash.get_mut(noodle).unwrap();
        if nu_node.name == "start" {
            continue;
        }
        if nu_node.name == "end" {
            // unsafe :flushed:
            unsafe { TOTAL += 1 };
            continue;
        }

        nu_node.visits += 1;
        let size = nu_node.size;

        let mut new_bool: bool = second_junc_active;
        if let Size::Small = size {
            if nu_node.visits == 2 && second_junc_active {
                new_bool = false;
            } else if nu_node.visits > 1 {
                continue;
            };
        }

        hunt(nu_node.clone(), new_hash, new_bool);
    }
}

#[derive(Debug, Clone, Copy)]
enum Size {
    Big,
    Small,
}

#[derive(Debug, Clone)]
struct Junction<'a> {
    name: &'a str,
    connections: Vec<&'a str>,
    size: Size,
    visits: u64,
}
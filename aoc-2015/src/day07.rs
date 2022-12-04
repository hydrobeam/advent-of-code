use std::collections::HashMap;

pub fn solve_day07() {
    let content = include_str!("../inputs/day07_input.txt");
    let mut input_hash: HashMap<&str, Line> = content.split('\n').map(process_input).collect();
    let goal_len: usize = input_hash.len();


    // will contain completed values
    let mut output_hash_map: HashMap<&str, u16> = HashMap::with_capacity(goal_len);

    while output_hash_map.len() < goal_len {
        for key in input_hash.keys() {
            hunt(key, &input_hash, &mut output_hash_map);
        }
    }

    let a_val = *output_hash_map.get("a").unwrap();
    output_hash_map.clear();

    // part B:

    //overwrite the value of "b" with that of a
    *input_hash.get_mut("b").unwrap() = Line {
        op: None,
        val1: ValType::Immediate(a_val),
        val2: None,
    };
    while output_hash_map.len() < goal_len {
        for key in input_hash.keys() {
            hunt(key, &input_hash, &mut output_hash_map);
        }
    }

    dbg!(a_val);
    dbg!(output_hash_map.get("a").unwrap());
}

fn hunt<'a>(
    input_str: &'a str,
    input_hash: &HashMap<&'a str, Line<'a>>,
    output_hash: &mut HashMap<&'a str, u16>,
) -> u16 {
    // if it exists in the output hash, get it.
    // otherwise, start resolving to find the value
    match output_hash.get(input_str) {
        Some(val) => *val,
        None => {
            let Line {
                op: operation,
                val1: v1,
                val2: v2,
            } = input_hash.get(input_str).unwrap();
            {
                let ret1: u16;
                let ret2: u16;
                let processed_val: u16;

                match v1 {
                    ValType::Indirect(val) => {
                        ret1 = hunt(val, input_hash, output_hash);
                    }
                    ValType::Immediate(val) => {
                        ret1 = *val;
                    }
                };

                match v2 {
                    Some(ValType::Indirect(val)) => {
                        ret2 = hunt(val, input_hash, output_hash);
                    }
                    Some(ValType::Immediate(val)) => {
                        ret2 = *val;
                    }
                    None => ret2 = 0, // have to initialize or else the compiler whines
                };

                match operation {
                    Some(OpType::AND) => {
                        processed_val = ret1 & ret2;
                    }
                    Some(OpType::LSHIFT) => {
                        processed_val = ret1 << ret2;
                    }
                    Some(OpType::RSHIFT) => {
                        processed_val = ret1 >> ret2;
                    }
                    Some(OpType::OR) => {
                        processed_val = ret1 ^ ret2;
                    }
                    Some(OpType::NOT) => {
                        processed_val = !ret1;
                    }
                    None => {
                        processed_val = ret1;
                    }
                };

                output_hash.insert(input_str, processed_val);
                processed_val
            }
        }
    }
}

/// construct the hash map of names to instructions
fn process_input(input_str: &str) -> (&str, Line) {
    //  af AND ah -> ai
    //  l_and_r = ["af AND ah", "ai"]
    //  left_vec = ["af", "AND", "ah"]
    let l_and_r: Vec<&str> = input_str.split("->").map(|x| x.trim()).collect();
    let left_vec: Vec<&str> = l_and_r[0].split(' ').collect();

    let r_val: Line = match left_vec.len() {
        1 => Line {
            op: None,
            val1: { make_val_type(left_vec[0]) },
            val2: None,
        },

        2 => Line {
            op: Some(make_op_type(left_vec[0])),
            val1: make_val_type(left_vec[1]),
            val2: None,
        },

        3 => Line {
            op: Some(make_op_type(left_vec[1])),
            val1: make_val_type(left_vec[0]),
            val2: Some(make_val_type(left_vec[2])),
        },
        _ => unreachable!(),
    };

    (l_and_r[1], r_val)
}

fn make_val_type(input: &str) -> ValType {
    let temp = input.parse::<u16>();

    if let Ok(value) = temp {
        ValType::Immediate(value)
    } else {
        ValType::Indirect(input)
    }
}

fn make_op_type(input: &str) -> OpType {
    match input {
        "AND" => OpType::AND,
        "NOT" => OpType::NOT,
        "RSHIFT" => OpType::RSHIFT,
        "LSHIFT" => OpType::LSHIFT,
        "OR" => OpType::OR,
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone)]
enum OpType {
    AND,
    LSHIFT,
    RSHIFT,
    OR,
    NOT,
}

#[derive(Debug)]
enum ValType<'a> {
    Indirect(&'a str),
    Immediate(u16),
}

#[derive(Debug)]
struct Line<'a> {
    op: Option<OpType>,
    val1: ValType<'a>,
    val2: Option<ValType<'a>>,
}

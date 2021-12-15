use std::collections::BTreeMap;

pub fn solve_day14p1(vmalll:u32) {
    let mut input = include_str!("../inputs/day14input.txt").split("\n\n");

    let code = input
        .next()
        .unwrap()
        .split_terminator("")
        .skip(1)
        .collect::<Vec<&str>>();

    // dbg!(&code);
    let mut hashie: BTreeMap<&str, &str> = BTreeMap::new();
    let contents = input.next().unwrap().lines();

    for line in contents {
        let mut mphg = line.split(" -> ");

        let elem_1 = mphg.next().unwrap();
        let elem_2 = mphg.next().unwrap();

        hashie.insert(elem_1, elem_2);
    }

    let done = hunt(&code, &hashie, vmalll);

    // dbg!(done.len());

    let mut count_hash: BTreeMap<&str, u64> = BTreeMap::new();
    for character in done {
        *count_hash.entry(character).or_insert(0) += 1;
    }

    let mut sortie = count_hash.values().copied().collect::<Vec<u64>>();
   
    sortie.sort();
    dbg!(&sortie);


    dbg!(sortie[sortie.len() - 1] - sortie[0]);

    // dbg!(hashie);
}

fn hunt<'a>(
    current_string: &'a [&'a str],
    ledger: &'a BTreeMap<&str, &'a str>,
    mut step_limit: u32,
) -> Vec<&'a str> {
    let mut main_vec: Vec<&str> = current_string.to_owned();

    while step_limit > 0 {
        let mut curr_index = 1;
        let mut building_vec: Vec<&str> = main_vec.clone();
        for window in main_vec.windows(2) {
            // dbg!();
            match ledger.get(&window.iter().cloned().collect::<String>() as &str) {
                Some(val) => {
                    building_vec.insert(curr_index, val);
                    curr_index += 1;
                }
                None => println!("swag"),
            }
            curr_index += 1;
        }

        main_vec = building_vec;
        step_limit -= 1;
        // println!("{:?}", main_vec);
    }

    main_vec
}

pub fn solve_day14p2(vmall:u32) {
    let mut input = include_str!("../inputs/day14input.txt").split("\n\n");

    let code = input
        .next()
        .unwrap()
        .split_terminator("")
        .skip(1)
        .collect::<Vec<&str>>();

    let mut hashie: BTreeMap<&str, &str> = BTreeMap::new();
    let mut count_hashie: BTreeMap<&str, u128> = BTreeMap::new();
    let contents = input.next().unwrap().lines();

    for line in contents {
        let mut mphg = line.split(" -> ");

        let elem_1 = mphg.next().unwrap();
        let elem_2 = mphg.next().unwrap();

        hashie.insert(elem_1, elem_2);
        count_hashie.insert(elem_1, 0);
    }

    let mut bt_code: BTreeMap<String, u128> = BTreeMap::new();
    for hm in code.windows(2) {
        let a = hm.iter().cloned().collect::<String>();
        *bt_code.entry(a).or_insert(0) += 1;
    }

    let done = hunt_2(bt_code, &hashie, vmall);

    let mut letter_map_1:BTreeMap<char, u128> = BTreeMap::new();


    // BB: 2131321
    for key in done.keys() {
        let let_1 = key.as_bytes()[0] as char;
        let let_2 = key.as_bytes()[1] as char; 
        let occurences = done.get(key).unwrap().to_owned();

        if let_1 == let_2 {
            *letter_map_1.entry(let_1).or_insert(0) += occurences+1;
        } else {
            *letter_map_1.entry(let_1).or_insert(0) += (occurences)/2 as u128 ;
            *letter_map_1.entry(let_2).or_insert(0) += (occurences+1)/2 as u128;
        }

    }

    let mut a = letter_map_1.values().copied().collect::<Vec<u128>>();
    a.sort();

    dbg!(&a);
    dbg!(a[a.len()-1] - a[0] );

}

fn hunt_2<'a>(
    current_string: BTreeMap<String, u128>,
    ledger: &'a BTreeMap<&str, &'a str>,
    mut step_limit: u32,
) -> BTreeMap<String, u128> {
    let mut main_bt = current_string;

    while step_limit > 0 {
        let mut building_bt = main_bt.clone();
        for window in main_bt.keys() {
            match ledger.get(window as &str) {
                Some(val) => {
                    let mut holder = window.clone();
                    let occurences = main_bt.get(window).unwrap().to_owned();
                    holder.insert(1, val.chars().next().unwrap());
                    *building_bt
                        .entry(format!(
                            "{}{}",
                            holder.as_bytes()[0] as char,
                            holder.as_bytes()[1] as char
                        ))
                        .or_insert(0) += occurences;

                    *building_bt
                        .entry(format!(
                            "{}{}",
                            holder.as_bytes()[1] as char,
                            holder.as_bytes()[2] as char
                        ))
                        .or_insert(0) += occurences;

                    *building_bt.get_mut(window).unwrap() -= occurences;
                }
                None => (),
            }
        }

        main_bt = building_bt;
        // dbg!(&main_bt);
        step_limit -= 1;
    }

    main_bt
}

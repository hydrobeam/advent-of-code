const MONKEY_TURNS: u128 = 15;
pub fn solve() {
    let content = include_str!("../inputs/day11.txt");

    let mut monkey_list: Vec<Monkey> = content.split("\n\n").map(Monkey::from).collect();
    let divisibility_values: Vec<u128> = monkey_list.iter().map(|x| x.cond).collect();

    for turn in 0..MONKEY_TURNS {
        for (i, mon) in monkey_list.iter().enumerate() {
            print!("{i}: [");
            for item in &mon.items {
                print!("{item},");
            }
            print!("]\n");
        }
        println!();
        for ind in 0..monkey_list.len() {
            let monkey = &mut monkey_list[ind];
            let mut throws: Vec<(u128, u128)> = Vec::new();

            monkey.num_inspections += monkey.items.len() as u128;
            for item_ind in (0..monkey.items.len()).rev() {
                let item = monkey.items[item_ind];
                let new_worry = monkey.op.process(item);
                let monkey_target = monkey.test(new_worry);
                throws.push((monkey_target, new_worry));
            }
            // monkey is about to throw all these items, so nuke em
            monkey.items.clear();

            // handle operations after to avoid mutating while holding a borrow
            for (ind, item) in throws {
                monkey_list[ind as usize].items.push(item);
            }
        }
    }

    let monkey_business: u128 = monkey_list
        .iter()
        .fold([0, 0], |mut acc, monkey| {
            if monkey.num_inspections > acc[0] {
                acc[1] = acc[0];
                acc[0] = monkey.num_inspections;
            } else if monkey.num_inspections > acc[1] {
                acc[1] = monkey.num_inspections;
            }
            acc
        })
        .iter()
        .product();

    // dbg!(monkey_list);

    println!("Part 1: {monkey_business}");
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    op: Operation,
    // if divisible by input, return monkey to toss to
    cond: u128,
    true_case: u128,
    false_case: u128,
    num_inspections: u128,
}

impl Monkey {
    fn test(&self, worry_level: u128) -> u128 {
        if (worry_level % self.cond) == 0 {
            self.true_case
        } else {
            self.false_case
        }
    }
}

impl From<&str> for Monkey {
    /// Format:
    /// Monkey 0:
    /// Starting items: 58, 93, 88, 81, 72, 73, 65
    ///   Operation: new = old * 13
    ///   Test: divisible by 11
    ///     If true: throw to monkey 3
    ///     If false: throw to monkey 2
    ///
    fn from(val: &str) -> Self {
        let mut temp = val.lines();
        temp.next(); // skip monkey number line
        let items = temp
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u128>().unwrap())
            .collect();

        let op = Operation::from(temp.next().unwrap().split("= ").nth(1).unwrap());
        let cond: u128 = temp
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let true_case: u128 = temp
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let false_case: u128 = temp
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items,
            op,
            cond,
            true_case,
            false_case,
            num_inspections: 0,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
    Num(u128),
    Old,
}

impl Operation {
    // return new worry level
    fn process(&self, val: u128) -> u128 {
        match self {
            Operation::Add(one, two) => one.process(val) + two.process(val),
            Operation::Mul(one, two) => one.process(val) * two.process(val),
            Operation::Num(num) => *num,
            Operation::Old => val,
        }
    }
}
impl From<&str> for Operation {
    ///  Format:
    ///  old * 13
    ///  13 * 13
    ///  old * old
    fn from(value: &str) -> Self {
        let mut val_iter = value.split(' ');
        let first = match val_iter.next().unwrap() {
            "old" => Operation::Old,
            num => Operation::Num(num.parse().unwrap()),
        };
        // hold off onthis
        let op = val_iter.next().unwrap();

        let last = match val_iter.next().unwrap() {
            "old" => Operation::Old,
            num => Operation::Num(num.parse().unwrap()),
        };

        if let "*" = op {
            Operation::Mul(Box::new(first), Box::new(last))
        } else {
            Operation::Add(Box::new(first), Box::new(last))
        }
    }
}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve_day13() {
    let mut content = include_str!("../inputs/day13_input.txt").split('\n');
    let estimate: i64 = content.next().unwrap().parse().unwrap();
    let temp = content.next().unwrap().split(',');

    let mut dep2 = temp
        .clone()
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| Bus::new(x.1.parse::<i64>().unwrap(), x.0 as i64))
        .collect::<Vec<Bus>>();

    dep2.sort_unstable_by(|a, b| a.index.cmp(&b.index));

    let departs = temp
        .filter(|&x| x != "x")
        .map(|x| x.parse::<i64>().expect(x));
    // div up
    let mut res = departs
        // 939 / 59 = 15.9something, round up = 16
        // 59 * 16 = 944
        // 944 - 939
        // 5
        //.map(|x| (x * dbg!((estimate + (x - 1)) / x) - estimate, x))
        // cant simplify
        .map(|x| (x * ((estimate + (x - 1)) / x) - estimate, x))
        .collect::<Vec<(i64, i64)>>();

    res.sort_unstable_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());
    println!("The solution to part 1 is {}", res[0].0 * res[0].1);


    //let mut heap: BinaryHeap<i64> = BinaryHeap::with_capacity(res.len());
    //dep2.for_each(|x| heap.push(x.1));
    //dbg!(dep2);

    'outer: loop {
        let mut counting_index = 0;

        'inner: loop {
            let our_element = dep2[counting_index];

            match dep2.get(counting_index + 1) {
                None => break 'outer,
                Some(future_element) => {
                    if our_element.index < future_element.index
                        && (future_element.timestamp - our_element.timestamp)
                            == (future_element.index - our_element.index)
                    {
                        // do nothing, keep going
                    } else {
                        dep2[counting_index].timestamp += our_element.id;
                        break 'inner;
                    }
                }
            }
            counting_index += 1;
        }

        // put the smallest on top
        dep2.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
        // if printer % 50000000 == 0 {
        //     dbg!(&dep2);
        // }
        // printer += 1;
    }
    dbg!(dep2);
}

#[derive(Debug, Clone, Copy, Eq)]
struct Bus {
    id: i64,
    timestamp: i64,
    index: i64,
}

impl Bus {
    fn new(id: i64, index: i64) -> Self {
        Self {
            id,
            timestamp: 0,
            index,
        }
    }
}

impl Ord for Bus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialEq for Bus {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl PartialOrd for Bus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

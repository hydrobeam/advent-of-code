use std::fs;
use std::str::FromStr;

#[allow(clippy::cast_possible_truncation)]
pub fn solve_day3p1() {
    let filename = "inputs/day3input.txt";
    let contents = fs::read_to_string(filename).expect("they makin me do this");
    let vecsquad = contents.lines();

    let mut i: u16 = 0;
    let mut hs: Vec<u16> = vec![0; 12];

    for bit in vecsquad {
        i += 1;
        let lst = bit.split_terminator("").into_iter().skip(1);

        for (index, item) in lst.enumerate() {
            hs[index] += u16::from_str(item).unwrap();
        }
    }

    let ho = hs
        .iter()
        .map(|&x| if x < i / 2 { 0 } else { 1 })
        .fold(0, |mut total, x| {
            total *= 2;
            total + x
        });

    let hm = hs
        .iter()
        .map(|&x| if x < i / 2 { 1 } else { 0 })
        .fold(0, |mut total, x| {
            total *= 2;
            total + x
        });

    dbg!(hm*ho);

}

#[allow(clippy::cast_possible_truncation)]
pub fn solve_day3p2() {

    let filename = "inputs/day3input.txt";
    let contents = fs::read_to_string(filename).expect("they makin me do this");
    let vecsquad = contents.lines();

    let mut oxy_iter = vecsquad
        .clone()
        .filter(|&x| u8::from_str(x.split_terminator("").nth(1).unwrap()).unwrap() == 0)
        .collect::<Vec<&str>>()
        .into_iter();

    for pos in 2..12 {
        let craw = oxyen_rating(
            do_stuff(oxy_iter.clone(), pos),
            oxy_iter.clone().count() as u16,
        );
        oxy_iter = oxy_iter
            .filter(|x| u8::from_str(x.split_terminator("").nth(pos).unwrap()).unwrap() == craw)
            .collect::<Vec<&str>>()
            .into_iter();

        if oxy_iter.len() == 1 {
            break;
        }
    }

    let a= i32::from_str_radix(oxy_iter.next().unwrap(), 2).unwrap();

    let mut c02_iter = vecsquad
        .filter(|&x| u8::from_str(x.split_terminator("").nth(1).unwrap()).unwrap() == 1)
        .collect::<Vec<&str>>()
        .into_iter();

    for pos in 2..12 {
        let craw = c02_rating(
            do_stuff(c02_iter.clone(), pos),
            c02_iter.clone().count() as u16,
        );
        c02_iter = c02_iter
            .filter(|x| u8::from_str(x.split_terminator("").nth(pos).unwrap()).unwrap() == craw)
            .collect::<Vec<&str>>()
            .into_iter();

        if c02_iter.len() == 1 {
            break;
        }
    }
    let b= i32::from_str_radix(c02_iter.next().unwrap(), 2).unwrap();

    dbg!(a*b);
}

fn do_stuff<'a, T>(vec: T, pos: usize) -> u16
where
    T: Iterator<Item = &'a str>,
{
    let mut ind: u16 = 0;
    // counts up the bits at an index in the iterator
    for bit in vec {
        ind += u16::from_str(bit.split_terminator("").into_iter().nth(pos).unwrap()).unwrap();
    }
    ind
}

fn oxyen_rating(num: u16, comp: u16) -> u8 {
    if num >= comp / 2 {
        // always rounds down  which could cause bugs
        1
    } else {
        0
    }
}

fn c02_rating(num: u16, comp: u16) -> u8 {
    // dbg!(comp);
    if num >= comp / 2 {
        0
    } else {
        1
    }
}

use::std::iter::Step;
use::std::ops::{Add, Sub, Mul};


pub fn solve_day04_p1() {
    let content = include_str!("../inputs/day04_input.txt").trim_end();
    let molley = content
        .split('-')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut min_val = Numb::from_num(molley[0], 10);
    //let max_val = Numb::from_num(molley[1], 10);

    //let mut min_val: Numb = Numb::from_num(11333, 10);
    let mut sum: u64 = 0;

    for _ in molley[0]..=molley[1] {
        let twos: usize = min_val.get_num_doubles();
        let threes: usize = min_val.get_num_triples();

        // fails on 111333
        if twos > 0
            && min_val
                .numb_vec
                .is_sorted_by(std::cmp::PartialOrd::partial_cmp)
            && (twos - 1) != threes
        {
            sum += 1;
        }
        min_val.increment();
    }

    dbg!(sum);

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Numb {
    numb_vec: Vec<u64>,
    base: u64,
}

impl Numb {
    fn new(base: u64) -> Self {
        Numb {
            numb_vec: Vec::new(), // empty vec represents 0.
            base,
        }
    }

    fn from_num(mut num: u64, base: u64) -> Self {
        let mut temp_vec: Vec<u64> = Vec::new();
        while num >= base {
            temp_vec.push(num % base);
            num /= base;
        }
        temp_vec.push(num);

        Numb {
            numb_vec: temp_vec,
            base,
        }
    }

    fn to_num(&self) -> u64 {
        self.numb_vec
            .iter()
            .rev()
            .fold(0, |acc, x| (acc + x) * self.base)
            / self.base // divide here because it over multiplies in the iter
    }

    fn increment(&mut self) {
        // add 1 to each index until it doesn't overflow
        for val in &mut self.numb_vec {
            *val += 1;
            if val >= &mut self.base {
                // &mut self.base to allow comparison
                *val = 0;
                continue;
            } else {
                return;
            }
        }
        // if we make it here, then everything is zero's,
        // so we need to add 1 to represent the increased value;

        self.numb_vec.push(1);
    }

    fn add(&mut self, other: Numb) -> &mut Self {
        assert_eq!(self.base, other.base);

        let mut carry: bool = false;
        for (index, val) in other.numb_vec.iter().enumerate() {
            self.numb_vec[index] += val;
            if carry {
                self.numb_vec[index] += 1;
                carry = false;
            }

            if (self.numb_vec[index]) >= self.base {
                self.numb_vec[index] -= self.base;
                carry = true;
            }
        }

        if carry {
            self.numb_vec.push(1);
        }

        self
    }

    fn mul(&mut self, other: Numb) -> &mut Self {
        // let mut carry:u64 = 0;

        // for (index, val) in other.numb_vec.iter().enumerate() {
        //
        // }
        todo!()
    }

    fn get_num_doubles(&self) -> usize {
        self.numb_vec.windows(2).filter(|x| x[0] == x[1]).count()
    }

    fn get_num_triples(&self) -> usize {
        self.numb_vec
            .windows(3)
            .filter(|x| x[0] == x[1] && x[1] == x[2])
            .count()
    }

    fn non_multi_duplicate(&self) -> bool {
        let bra:Vec<&[u64]> = self.numb_vec.windows(2).collect();
        let temp:Vec<&[&[u64]]> = bra.windows(2).collect();
        dbg!(&temp);
        let hu = bra.windows(2).filter(|x |
            if x[0][0] == x[0][1] && x[0] != x[1] {
                true
            } else {
                false
            }
        ).collect::<Vec<&[&[u64]]>>();

        dbg!(&hu);
        if dbg!(bra.windows(2).filter(
            |x|
            if x[0][0] == x[0][1] && x[0] != x[1] {
                true
            } else {
                false
            }
        )).count() != 0 {
            return true;
        }

        let index:usize = 0;

        let temp_vec = self.numb_vec.clone();

        while index < self.numb_vec.len() {
            let curr_num = self.numb_vec[index];
            let mut counter:usize = index;

            loop {
                if self.numb_vec[index] == self.numb_vec[index +1] {
                    counter +=1;
                } else {
                    break;
                }
            }

            if
            if self.numb_vec[index] == self.numb_vec[index + 1] {
                if self.numb_vec[index + 1] == self.numb_vec[index+2] {

                } 
                index +=2;
            }
        }
        true
        //for val in bra.windows(2) {
        //    if (val[0] == val[1]) {
        //        return false;
        //    }
        //}

        //true
    }
    fn is_sum(&self) -> bool {
        todo!()
    }
}



//impl Step for Numb {
//    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
//        let start
//    }
//
//    fn forward_checked(start: Self, count: usize) -> Option<Self> {
//
//    }
//    fn backward_checked(start: Self, count: usize) -> Option<Self> {
//
//    }
//
//    fn forward(start: Self, count: usize) -> Self {
//
//    }
//    unsafe fn forward_unchecked(start: Self, count: usize) -> Self {
//
//    }
//    fn backward(start: Self, count: usize) -> Self {
//
//    }
//
//    unsafe fn backward_unchecked(start: Self, count: usize) -> Self {
//    }
//
//}

#[cfg(test)]
mod tests {
    use super::Numb;

    #[test]
    fn test_to_num() {
        let original_value = 12345;
        let v1 = Numb::from_num(original_value, 10);

        let v2 = v1.to_num();

        assert_eq!(v2, original_value);
    }

    #[test]
    fn test_add() {
        let v1: u64 = 123456;
        let v2: u64 = 654321;

        let mut numb1: Numb = Numb::from_num(v1, 10);
        let numb2: Numb = Numb::from_num(v2, 10);

        assert_eq!(numb1.add(numb2).to_num(), v1 + v2);
    }

    #[test]
    fn increment_overflow() {
        let val: u64 = 9999;

        let mut num = Numb::from_num(val, 10);
        num.increment();
        assert_eq!(num.to_num(), 10000);
    }

    #[ignore = "mul not implemented"]
    #[test]
    fn test_mul() {
        let v1: u64 = 123456;
        let v2: u64 = 654321;

        let mut numb1: Numb = Numb::from_num(v1, 10);
        let numb2: Numb = Numb::from_num(v2, 10);

        assert_eq!(numb1.mul(numb2).to_num(), v1 * v2);
    }

    #[test]
    fn test_impl() {
        let v1 = 111222;
        let numb1: Numb = Numb::from_num(v1, 10);

        let doubl = numb1.get_num_doubles();
        let trupl = numb1.get_num_triples();

        assert_eq!(doubl, 4);
        assert_eq!(trupl, 2);

    }

    #[test]
    fn test_impl2() {
        let v1 = 111122;
        let numb1: Numb = Numb::from_num(v1, 10);

        let doubl: usize = numb1.get_num_doubles();
        let trupl: usize = numb1.get_num_triples();

        assert_eq!(doubl, 4);
        assert_eq!(trupl, 2);
    }

    //#[test]
    //fn test_windows() {
    //    let vra = vec![1, 2, 3, 4, 5];

    //    let bra:Vec<&[u64]> = vra.windows(2).collect();
    //    dbg!(&bra);

    //    for val in bra.windows(2) {
    //        if (val[0] == val[1]) {
    //            return false;
    //        }
    //    }

    //}

    #[test]
    fn test_numb_windows() {

        let v1 = 111222;
        let numb1: Numb = Numb::from_num(v1, 10);

        let sa:bool = numb1.non_multi_duplicate();

        assert_eq!(sa, false)

    }
}
pub fn solve_day04_p2() {}

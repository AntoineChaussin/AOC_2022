use std::{fmt::Display, iter::Sum};

use crate::get_input;

pub fn aoc_1_1() {
    let input = get_input("resource/aoc_1/data.txt");

    let mut max: u64 = 0;
    let mut current: u64 = 0;
    for line in input.iter() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0
        } else {
            current += line
                .parse::<u64>()
                .expect(&format!("Expect a number get {}", &line));
        }
    }

    println!("AOC-1-1 Max 1 elf: {}", &max);
}

pub fn aoc_1_2() {
    let input = get_input("resource/aoc_1/data.txt");

    let mut top3 = OrderedList::new();

    let mut current: u64 = 0;
    for line in input.iter() {
        if line == "" {
            top3.add(current);
            current = 0
        } else {
            current += line
                .parse::<u64>()
                .expect(&format!("Expect a number get {}", &line));
        }
    }

    println!("AOC-1-2 Max 3 elf: {}", &top3.sum());
}

pub struct OrderedList<T>
where
    T: Ord + Sum + Copy + Display,
{
    internal_list: Vec<T>,
}

impl<T> OrderedList<T>
where
    T: Ord + Sum + Copy + Display,
{
    pub fn new() -> Self {
        OrderedList {
            internal_list: vec![],
        }
    }

    pub fn add(&mut self, e: T) {
        if self.internal_list.is_empty() {
            self.internal_list.push(e)
        } else {
            let mut i = self.internal_list.len();
            while i > 0 && &e > &self.internal_list[i - 1] {
                i -= 1;
            }

            self.internal_list.insert(i, e);

            if self.internal_list.len() > 3 {
                self.internal_list.pop();
            }
        }
    }

    pub fn sum(&self) -> T {
        self.internal_list.iter().copied().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_list() {
        let mut list = OrderedList::new();

        assert_eq!(0, list.sum());

        list.add(3);
        list.add(2);
        list.add(1);

        assert_eq!(6, list.sum());

        assert_eq!(1, *list.internal_list.last().unwrap());

        list.add(4);
        assert_eq!(3, list.internal_list.len());
        assert_eq!(9, list.sum());
    }

    #[test]
    fn do_aoc_1_1() {
        aoc_1_1()
    }

    #[test]
    fn do_aoc_1_2() {
        aoc_1_2()
    }
}

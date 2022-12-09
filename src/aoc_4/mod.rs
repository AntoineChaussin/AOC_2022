use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use tuple::Map;

use crate::get_input;

pub fn aoc_4_1() {
    let input = get_input("resource/aoc_4/data.txt");

    let mut result = 0;

    for line in input.iter() {
        let (interval_1, interval_2) = to_intervals(line);

        if interval_1.contains(&interval_2) || interval_2.contains(&interval_1) {
            result += 1;
        }
    }

    println!("AOC-4-1 Number of full overlaps {}", &result);
}

pub fn aoc_4_2() {
    let input = get_input("resource/aoc_4/data.txt");

    let mut result = 0;

    for line in input.iter() {
        let (interval_1, interval_2) = to_intervals(line);

        if interval_1.overlaps(&interval_2) || interval_2.overlaps(&interval_1) {
            result += 1;
        }
    }

    println!("AOC-4-2 Number of overlaps {}", &result);
}

fn to_intervals(input: &String) -> (Interval, Interval) {
    input
        .split(",") //split the line at the ,
        .tuples::<(&str, &str)>() //we expect 2 elements per line, so iterate by groups of 2
        .next() // next group of 2
        .expect("There should be 2 elements in line") // if there aren't 2 elements, we have a problem
        .map(|s| TryInto::<Interval>::try_into(s).unwrap()) //convert each element in the group of 2 to an interval
}

struct Interval {
    min: u32,
    max: u32,
}

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.contains_elem(other.min)
            || self.contains_elem(other.max)
            || other.contains_elem(self.min)
            || other.contains_elem(self.max)
    }

    fn contains_elem(&self, elem: u32) -> bool {
        self.min <= elem && self.max >= elem
    }
}

impl TryFrom<&str> for Interval {
    type Error = String; //just sends an error message because I'm lazy

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref INTERVAL_REGEX: Regex = Regex::new(r"(?P<min>[0-9]+)-(?P<max>[0-9]+)")
                .expect("Interval regex should be correct");
        }

        if let Some(captures) = INTERVAL_REGEX.captures(value) {
            Ok(Interval {
                min: captures["min"].parse().unwrap(),
                max: captures["max"].parse().unwrap(),
            })
        } else {
            Err(format!("Interval format is not correct {}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test<F>(test_func: F, v1: Vec<&str>, v2: Vec<&str>)
    where
        F: Fn(&Interval, &Interval) -> bool,
    {
        let results: Vec<bool> = v1
            .iter()
            .zip(v2.iter())
            .map(|(f, s)| {
                (
                    TryInto::<Interval>::try_into(*f).unwrap(),
                    TryInto::<Interval>::try_into(*s).unwrap(),
                )
            })
            .map(|(f, s)| test_func(&f, &s))
            .collect();

        assert!(results.iter().all(|b| *b));
    }

    #[test]
    fn interval_contains() {
        do_test(
            |i1: &Interval, i2: &Interval| i1.contains(i2),
            vec!["10-20", "1-5", "6-12", "408-509", "0-0"],
            vec!["13-15", "2-5", "6-11", "408-409", "0-0"],
        );
    }

    #[test]
    fn interval_not_contains() {
        do_test(
            |i1: &Interval, i2: &Interval| !i1.contains(i2),
            vec!["10-20", "1-5", "6-12", "408-509", "0-0"],
            vec!["8-26", "0-5", "6-16", "90-98", "6-6"],
        );
    }

    #[test]
    fn interval_overlaps() {
        do_test(
            |i1: &Interval, i2: &Interval| i1.overlaps(i2),
            vec!["10-20", "5-8", "6-12", "408-509", "0-100"],
            vec!["5-25", "3-6", "5-11", "300-800", "0-0"],
        );
    }

    #[test]
    fn interval_not_overlaps() {
        do_test(
            |i1: &Interval, i2: &Interval| !i1.overlaps(i2),
            vec!["10-20", "5-10", "6-6"],
            vec!["25-35", "0-4", "7-7"],
        );
    }

    #[test]
    fn test_aoc_4_1() {
        aoc_4_1();
    }

    #[test]
    fn test_aoc_4_2() {
        aoc_4_2();
    }
}

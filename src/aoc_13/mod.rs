use std::{fmt::Display, iter::Peekable};

use itertools::Itertools;

use crate::get_input;

#[derive(PartialEq, Eq, Debug, Ord)]
enum PacketData {
    Int(u32),
    Vec(Vec<Box<PacketData>>),
}

#[derive(PartialEq, Eq, Debug)]
enum OrderIs {
    Ok,
    Ng,
    NotSureYet,
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::Int(i) => write!(f, "{}", i),
            PacketData::Vec(v) => write!(f, "[{}]", v.iter().join(",")),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match ordered(self, other) {
            OrderIs::Ok => Some(std::cmp::Ordering::Less),
            OrderIs::Ng => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    }
}

fn parse<I: Iterator<Item = String>>(mut input: I) -> Vec<(PacketData, PacketData)> {
    let mut res = vec![];

    while let Some(line) = input.next() {
        let packet1 = parse_packet(&mut line.chars().peekable());
        let packet2 = parse_packet(&mut input.next().unwrap().chars().peekable());

        res.push((packet1, packet2));

        let _ = input.next();
    }
    res
}

fn parse_packet<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> PacketData {
    let first = input.next().unwrap();

    assert_eq!(first, '[');
    parse_packet_rec(input, vec![])
}

fn parse_packet_rec<I: Iterator<Item = char>>(
    input: &mut Peekable<I>,
    mut current_vec: Vec<Box<PacketData>>,
) -> PacketData {
    while let Some(c) = input.next() {
        match c {
            '[' => {
                let sub_list = parse_packet_rec(input, vec![]);
                current_vec.push(Box::new(sub_list));
            }
            ']' => return PacketData::Vec(current_vec),
            d if d.is_numeric() => {
                let mut char_vec = vec![d];
                while let Some(peek) = input.peek() {
                    if peek.is_numeric() {
                        char_vec.push(input.next().unwrap());
                    } else {
                        break;
                    }
                }

                current_vec.push(Box::new(PacketData::Int(
                    char_vec.into_iter().collect::<String>().parse().unwrap(),
                )));
            }
            ',' => (),
            _ => unreachable!(),
        }
    }

    PacketData::Vec(current_vec)
}

fn ordered(left: &PacketData, right: &PacketData) -> OrderIs {
    match (left, right) {
        (PacketData::Int(l), PacketData::Int(r)) if l < r => OrderIs::Ok,
        (PacketData::Int(l), PacketData::Int(r)) if l == r => OrderIs::NotSureYet,
        (PacketData::Int(_l), PacketData::Int(_r)) => OrderIs::Ng, //l>r
        (PacketData::Int(l), PacketData::Vec(_)) => {
            ordered(&PacketData::Vec(vec![Box::new(PacketData::Int(*l))]), right)
        }
        (PacketData::Vec(_), PacketData::Int(r)) => {
            ordered(left, &PacketData::Vec(vec![Box::new(PacketData::Int(*r))]))
        }
        (PacketData::Vec(l), PacketData::Vec(r)) => {
            let (mut left_iter, mut right_iter) = (l.iter(), r.iter());
            let (mut current_left, mut current_right) = (left_iter.next(), right_iter.next());
            while current_left.is_some() || current_right.is_some() {
                let check_ordered = match (current_left, current_right) {
                    (Some(l), Some(r)) => ordered(l, r),
                    (None, Some(_)) => OrderIs::Ok,
                    (Some(_), None) => OrderIs::Ng,
                    (None, None) => OrderIs::NotSureYet,
                };

                match check_ordered {
                    OrderIs::NotSureYet => (),
                    _ => return check_ordered,
                }

                (current_left, current_right) = (left_iter.next(), right_iter.next());
            }
            OrderIs::NotSureYet
        }
    }
}

fn sum_ok_pairs(pairs: Vec<(PacketData, PacketData)>) -> usize {
    pairs
        .iter()
        .map(|(l, r)| ordered(l, r))
        .enumerate()
        .filter(|(_, o)| o == &OrderIs::Ok)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn aoc_13_1() {
    let input = get_input("resource/aoc_13/data.txt");
    let packets = parse(input.into_iter());

    let sum = sum_ok_pairs(packets);

    println!("AOC-13-1 sum ok pairs: {}", &sum)
}

fn sort_packets(packets: Vec<(PacketData, PacketData)>) -> Vec<PacketData> {
    let mut all_packets: Vec<PacketData> = packets
        .into_iter()
        .map(|(l, r)| vec![l, r])
        .flatten()
        .collect();

    all_packets.push(PacketData::Vec(vec![Box::new(PacketData::Vec(vec![
        Box::new(PacketData::Int(2)),
    ]))]));
    all_packets.push(PacketData::Vec(vec![Box::new(PacketData::Vec(vec![
        Box::new(PacketData::Int(6)),
    ]))]));

    all_packets.sort();

    all_packets
}

fn find_dividers(packets: Vec<PacketData>) -> usize {
    packets
        .iter()
        .map(|p| p.to_string())
        .enumerate()
        .filter(|(_, s)| s == "[[2]]" || s == "[[6]]")
        .map(|(i, _)| i + 1)
        .fold(1, |acc, i| acc * i)
}

pub fn aoc_13_2() {
    let input = get_input("resource/aoc_13/data.txt");
    let packets = parse(input.into_iter());

    let divs = find_dividers(sort_packets(packets));

    println!("AOC-13-2 dividers index product: {}", &divs)
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::OrderIs::{Ng, Ok};
    use super::*;

    const INPUT: &'static str = "[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse() {
        let i = INPUT.split("\n").map(|s| s.trim().to_string());

        let packets = parse(i);
        let (test1, test2) = &packets[0];
        assert_eq!(test1.to_string().as_str(), "[1,1,3,1,1]");
        assert_eq!(test2.to_string().as_str(), "[1,1,5,1,1]");
        let (test5, test6) = &packets[2];
        assert_eq!(test5.to_string().as_str(), "[9]");
        assert_eq!(test6.to_string().as_str(), "[[8,7,6]]");

        let (test15, test16) = &packets[7];
        assert_eq!(test15.to_string().as_str(), "[1,[2,[3,[4,[5,6,7]]]],8,9]");
        assert_eq!(test16.to_string().as_str(), "[1,[2,[3,[4,[5,6,0]]]],8,9]");
    }

    #[test]
    fn test_ordered() {
        let i = INPUT.split("\n").map(|s| s.trim().to_string());

        let packets = parse(i);

        let ordered = packets.iter().map(|(l, r)| ordered(l, r));

        assert_equal(ordered, vec![Ok, Ok, Ng, Ok, Ng, Ok, Ng, Ng])
    }

    #[test]
    fn test_sum() {
        let i = INPUT.split("\n").map(|s| s.trim().to_string());

        let packets = parse(i);

        assert_eq!(sum_ok_pairs(packets), 13);
    }

    #[test]
    fn test_delim() {
        let i = INPUT.split("\n").map(|s| s.trim().to_string());

        let packets = parse(i);

        assert_eq!(find_dividers(sort_packets(packets)), 140);
    }

    #[test]
    fn test_sort() {
        let i = INPUT.split("\n").map(|s| s.trim().to_string());

        let packets = parse(i);

        let sorted = sort_packets(packets);

        assert_equal(
            sorted.iter().map(|p| p.to_string()),
            vec![
                "[]",
                "[[]]",
                "[[[]]]",
                "[1,1,3,1,1]",
                "[1,1,5,1,1]",
                "[[1],[2,3,4]]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[[1],4]",
                "[[2]]",
                "[3]",
                "[[4,4],4,4]",
                "[[4,4],4,4,4]",
                "[[6]]",
                "[7,7,7]",
                "[7,7,7,7]",
                "[[8,7,6]]",
                "[9]",
            ],
        );
    }

    #[test]
    fn test_aoc_13_1() {
        aoc_13_1()
    }
    #[test]
    fn test_aoc_13_2() {
        aoc_13_2()
    }
}

use std::collections::{HashSet, VecDeque};

use crate::get_input;

fn find_marker<T: AsRef<str>>(s: T, marker_size: usize) -> usize {
    let mut iter_chars = s.as_ref().chars();
    let mut marker = iter_chars.by_ref().take(3).collect::<VecDeque<char>>();

    let mut position = 3;

    for c in iter_chars {
        position += 1;

        add(&mut marker, c, marker_size);

        if marker.iter().copied().collect::<HashSet<char>>().len() == marker_size {
            break; //we have 4 different characters in the set
        }
    }

    position
}

fn add(marker: &mut VecDeque<char>, c: char, marker_size: usize) {
    marker.push_back(c);
    if marker.len() > marker_size {
        marker.pop_front();
    }
}

pub fn aoc_6_1() {
    let input = get_input("resource/aoc_6/data.txt");

    assert!(input.len() == 1);

    let first_marker = find_marker(input.first().unwrap(), 4);

    println!("AOC-6-1 first marker 4 {}", first_marker);
}

pub fn aoc_6_2() {
    let input = get_input("resource/aoc_6/data.txt");

    assert!(input.len() == 1);

    let first_marker = find_marker(input.first().unwrap(), 14);

    println!("AOC-6-2 first marker 14 {}", first_marker);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_6_1() {
        aoc_6_1();
    }

    #[test]
    fn test_aoc_6_2() {
        aoc_6_2();
    }

    #[test]
    fn test_find_first_4() {
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test_find_first_14() {
        assert_eq!(19, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}

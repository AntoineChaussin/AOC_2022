use std::collections::HashSet;

use itertools::Itertools;
use tuple::Map;

use crate::get_input;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Item(char);

type Priority = u32;

impl Item {
    fn priority(&self) -> Priority {
        let code = self.0 as u32;

        if self.0.is_ascii_lowercase() {
            let code_a = 'a' as u32;

            1 + code - code_a
        } else if self.0.is_ascii_uppercase() {
            let code_uppera = 'A' as u32;

            27 + code - code_uppera
        } else {
            panic!("Non ascii in input {}", self.0)
        }
    }
}

pub fn aoc_3_1() {
    let input = get_input("resource/aoc_3/data.txt");

    let mut result = 0;

    for line in input.iter() {
        assert!(line.len() % 2 == 0);

        let mid = line.len() / 2;

        let (first_half, second_half) = line.split_at(mid).map(to_items);

        let common_items = first_half
            .intersection(&second_half)
            .map(|el| el.clone())
            .collect::<Vec<Item>>();

        assert!(common_items.len() == 1);

        result += common_items[0].priority();
    }

    println!("ACO-3-1 Sum of priorities {}", &result);
}

pub fn aoc_3_2() {
    let input = get_input("resource/aoc_3/data.txt");

    assert!(input.len() % 3 == 0);

    let mut iter = input.iter();

    let mut result = 0;

    while let Some(three_elves) = iter.next_tuple::<(&String, &String, &String)>() {
        let (elf1, elf2, elf3) = three_elves.map(to_items);

        let common_items: HashSet<Item> = elf1
            .intersection(&elf2)
            .map(|el| el.clone())
            .collect::<HashSet<Item>>()
            .intersection(&elf3)
            .map(|el| el.clone())
            .collect();

        assert!(common_items.len() == 1);

        result += common_items
            .iter()
            .next()
            .expect("No common items")
            .priority();
    }

    println!("AOC-3-2 Sum of priorities for 3 elves {}", &result);
}

fn to_items<T: AsRef<str>>(s: T) -> HashSet<Item> {
    s.as_ref()
        .chars()
        .map(|c| Item(c))
        .collect::<HashSet<Item>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let a = Item('a');
        let z = Item('z');

        let upper_a = Item('A');
        let upper_z = Item('Z');

        assert_eq!(a.priority(), 1);
        assert_eq!(z.priority(), 26);
        assert_eq!(upper_a.priority(), 27);
        assert_eq!(upper_z.priority(), 52);
    }

    #[test]
    fn test_aoc_3_1() {
        aoc_3_1();
    }

    #[test]
    fn test_aoc_3_2() {
        aoc_3_2();
    }
}

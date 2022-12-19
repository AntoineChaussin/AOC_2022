use std::{fs, path::Path};

pub mod aoc_1;
pub mod aoc_2;
pub mod aoc_3;
pub mod aoc_4;
pub mod aoc_5;
pub mod aoc_6;
pub mod aoc_7;
pub mod aoc_8;

fn get_input<T: AsRef<Path> + ?Sized>(path: &T) -> Vec<String> {
    let data = fs::read_to_string(path).expect("Cannot read file");

    data.split("\r\n").map(|s| s.to_string()).collect()
}

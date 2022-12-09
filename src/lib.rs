use std::{path::Path, fs};

pub mod aoc_1;
pub mod aoc_2;
pub mod aoc_3;
pub mod aoc_4;
fn get_input<T: AsRef<Path> + ?Sized> (path : &T) -> Vec<String>  {
    let data = fs::read_to_string(path).expect("Cannot read file");

    data.split("\r\n").map(|s| s.to_string()).collect()
}

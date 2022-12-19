use std::{collections::VecDeque, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

use crate::get_input;

enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn update_register(self, register: i32) -> i32 {
        match self {
            Instruction::Noop => register,
            Instruction::Add(add) => register + add,
        }
    }
}

struct Stack {
    stack: VecDeque<Instruction>,
}

impl Stack {
    fn update_register(&mut self, register: i32) -> i32 {
        let instr = self.stack.pop_front();
        match instr {
            Some(instr) => instr.update_register(register),
            None => panic!("No more instructions"),
        }
    }

    fn after_n_instructions(&mut self, register: i32, n: usize) -> i32 {
        let mut updated_register = register;
        for _ in 0..n {
            updated_register = self.update_register(updated_register);
        }

        updated_register
    }
}

impl<'a> FromIterator<&'a str> for Stack {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        lazy_static! {
            static ref NOOP_REGEX: Regex = Regex::new("noop").unwrap();
            static ref ADDX_REGEX: Regex = Regex::new("addx (?P<x>[-]?[0-9]+)").unwrap();
        }

        let mut stack = VecDeque::new();
        for s in iter {
            if NOOP_REGEX.is_match(s) {
                stack.push_back(Instruction::Noop);
            } else {
                let captures = ADDX_REGEX.captures(s).unwrap();
                let to_add = captures["x"].parse::<i32>().unwrap();
                stack.push_back(Instruction::Noop);
                stack.push_back(Instruction::Add(to_add));
            }
        }

        Stack { stack }
    }
}

impl IntoIterator for Stack {
    type Item = Instruction;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.into_iter()
    }
}

enum ScreenState {
    Lit,
    Dark,
}

impl Display for ScreenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScreenState::Lit => write!(f, "#"),
            ScreenState::Dark => write!(f, "."),
        }
    }
}

struct Screen {
    states: Vec<ScreenState>,
}

impl Screen {
    fn new() -> Self {
        Screen { states: vec![] }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.states.len() {
            if i > 0 && i % 40 == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", &self.states[i])?;
        }

        Ok(())
    }
}

fn run_instructions(stack: Stack) -> Screen {
    let mut screen = Screen::new();

    let mut register = 1;
    let mut crt_position = 0;

    for instr in stack {
        crt_position = crt_position % 40;
        if crt_position == register || crt_position == register - 1 || crt_position == register + 1
        {
            screen.states.push(ScreenState::Lit);
        } else {
            screen.states.push(ScreenState::Dark);
        }
        register = instr.update_register(register);
        crt_position += 1;
    }

    screen
}

pub fn aoc_10_1() {
    let input = get_input("resource/aoc_10/data.txt");

    let mut stack: Stack = input.iter().map(|s| s.as_str()).into_iter().collect();

    let mut register = 1;

    register = stack.after_n_instructions(register, 19);
    let mut signal = 20 * register;

    for i in 1..=5 {
        register = stack.after_n_instructions(register, 40);
        signal += register * (20 + 40 * i);
    }

    println!("AOC-10-1 signal {}", signal);
}

pub fn aoc_10_2() {
    let input = get_input("resource/aoc_10/data.txt");

    let stack: Stack = input.iter().map(|s| s.as_str()).into_iter().collect();

    let screen = run_instructions(stack);

    println!("AOC-10-2 screen\n{}", screen);
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    const INPUT: [&'static str; 3] = ["noop", "addx 3", "addx -5"];

    #[test]
    fn test_parse() {
        let mut stack: Stack = INPUT.into_iter().collect();

        let mut register = 1;

        //during cycle 2
        register = stack.update_register(register);
        assert_eq!(register, 1);
        //during cycle 3
        register = stack.update_register(register);
        assert_eq!(register, 1);
        //during cycle 4
        register = stack.update_register(register);
        assert_eq!(register, 4);
        //during cycle 5
        register = stack.update_register(register);
        assert_eq!(register, 4);
        //during cycle 6
        register = stack.update_register(register);
        assert_eq!(register, -1);
    }

    #[test]
    fn test_aftern() {
        let input = get_input("resource/aoc_10/test_data.txt");

        let mut stack: Stack = input.iter().map(|s| s.as_str()).into_iter().collect();

        let mut register = 1;
        register = stack.after_n_instructions(register, 19);
        assert_eq!(register, 21);
        register = stack.after_n_instructions(register, 40);
        assert_eq!(register, 19);
        register = stack.after_n_instructions(register, 40);
        assert_eq!(register, 18);
        register = stack.after_n_instructions(register, 40);
        assert_eq!(register, 21);
        register = stack.after_n_instructions(register, 40);
        assert_eq!(register, 16);
        register = stack.after_n_instructions(register, 40);
        assert_eq!(register, 18);
    }

    #[test]
    fn test_aoc_10_1() {
        aoc_10_1()
    }
    #[test]
    fn test_aoc_10_2() {
        aoc_10_2()
    }

    #[test]
    fn test_print_screen() {
        let input = get_input("resource/aoc_10/test_data.txt");

        let stack: Stack = input.iter().map(|s| s.as_str()).into_iter().collect();

        let screen = run_instructions(stack);

        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n")
        .to_string();

        assert_eq!(screen.to_string(), expected);
    }
}

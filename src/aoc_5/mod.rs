use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
    slice::Iter,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::get_input;

type Crates = Vec<VecDeque<Crate>>;

struct Crate {
    name: String, //could use char here instead but that means more conversion nonsense
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TryFrom<&str> for Crate {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref CRATE_REGEX: Regex = Regex::new(r"\[(?P<name>[A-Z])\]").unwrap();
        };

        match CRATE_REGEX.captures(value) {
            Some(capture) => Ok(Crate {
                name: capture["name"].to_string(),
            }),
            None => Err("No match"),
        }
    }
}

fn parse_crates(input_iter: &mut Iter<String>) -> Crates {
    let mut result = Vec::new();

    while let Some(line) = input_iter.next() {
        if line.contains(|c: char| c.is_digit(10)) {
            break;
        }

        let mut iter_char = line.chars().peekable();
        let mut col_nb = 1;

        while iter_char.peek().is_some() {
            let three_char = iter_char.by_ref().take(3).collect::<String>();

            if col_nb > result.len() {
                result.push(VecDeque::new());
            }

            if let Ok(as_crate) = TryInto::<Crate>::try_into(three_char.as_str()) {
                let pile = &mut result[col_nb - 1];
                pile.push_front(as_crate);
            }
            col_nb += 1;
            let _ = iter_char.next(); //skip white space
        }
    }

    let _ = input_iter.next(); //skip the blank line before returning

    result
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref MOVE_REGEX: Regex =
                Regex::new(r"move (?P<nb>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)").unwrap();
        };

        let captures = MOVE_REGEX.captures(value).unwrap();

        Move {
            nb: captures["nb"].parse().unwrap(),
            from: captures["from"].parse().unwrap(),
            to: captures["to"].parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    nb: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn do_move<F: Fn(&mut VecDeque<Crate>) -> Vec<Crate>>(
        &self,
        mut crates: Crates,
        drain_fn: F,
    ) -> Crates {
        let to_move: Vec<Crate> = {
            let from = &mut crates[self.from - 1];
            drain_fn(from) //need to collect so the compiler does not complain about mutating crates from 2 references
        };

        let to = &mut crates[self.to - 1];
        for c in to_move {
            to.push_back(c);
        }
        crates
    }

    fn do_move_one_by_one(&self, crates: Crates) -> Crates {
        self.do_move(crates, |from| {
            let mut to_move: Vec<Crate> = from.drain(from.len() - self.nb..).collect();
            to_move.reverse(); // reverse so the head (last item of the deque) is inserted first in the new queue
            to_move
        })
    }

    fn do_move_by_stack(&self, crates: Crates) -> Crates {
        self.do_move(crates, |from| {
            let to_move: Vec<Crate> = from.drain(from.len() - self.nb..).collect();
            to_move
        })
    }
}

fn parse_move(input_iter: &mut Iter<String>) -> Vec<Move> {
    let mut result = Vec::new();

    for line in input_iter {
        result.push(line.as_str().into());
    }

    result
}

fn parse(input_iter: &mut Iter<String>) -> (Crates, Vec<Move>) {
    (parse_crates(input_iter), parse_move(input_iter))
}

pub fn aoc_5_1() {
    let input = get_input("resource/aoc_5/data.txt");

    let (mut crates, moves) = parse(&mut input.iter());

    for m in moves {
        crates = m.do_move_one_by_one(crates);
    }

    let code: String = crates
        .iter()
        .map(|pile| pile.back().map_or(String::from(""), |c| c.to_string()))
        .collect();

    println!("AOC-5-1 Crates code {}", &code);
}

pub fn aoc_5_2() {
    let input = get_input("resource/aoc_5/data.txt");

    let (mut crates, moves) = parse(&mut input.iter());

    for m in moves {
        crates = m.do_move_by_stack(crates);
    }

    let code: String = crates
        .iter()
        .map(|pile| pile.back().map_or(String::from(""), |c| c.to_string()))
        .collect();

    println!("AOC-5-2 Crates code {}", &code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let input_as_vec: Vec<String> = input.iter().map(|s| s.to_string()).collect();

        let mut iter = input_as_vec.iter();

        let (parsed, moves) = parse(&mut iter);

        assert_eq!(3, parsed.len());

        itertools::assert_equal(parsed[0].iter().map(|c| c.to_string()), vec!["Z", "N"]);
        itertools::assert_equal(parsed[1].iter().map(|c| c.to_string()), vec!["M", "C", "D"]);
        itertools::assert_equal(parsed[2].iter().map(|c| c.to_string()), vec!["P"]);

        itertools::assert_equal(
            moves,
            vec![
                Move {
                    nb: 1,
                    from: 2,
                    to: 1,
                },
                Move {
                    nb: 3,
                    from: 1,
                    to: 3,
                },
                Move {
                    nb: 2,
                    from: 2,
                    to: 1,
                },
                Move {
                    nb: 1,
                    from: 1,
                    to: 2,
                },
            ],
        )
    }

    #[test]
    fn test_move_one_by_one() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let input_as_vec: Vec<String> = input.iter().map(|s| s.to_string()).collect();

        let mut iter = input_as_vec.iter();

        let (mut crates, moves) = parse(&mut iter);

        for m in moves {
            crates = m.do_move_one_by_one(crates);
            print_crates(&crates);
        }

        itertools::assert_equal(crates[0].iter().map(|c| c.to_string()), vec!["C"]);
        itertools::assert_equal(crates[1].iter().map(|c| c.to_string()), vec!["M"]);
        itertools::assert_equal(
            crates[2].iter().map(|c| c.to_string()),
            vec!["P", "D", "N", "Z"],
        );
    }

    #[test]
    fn test_move_by_stack() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let input_as_vec: Vec<String> = input.iter().map(|s| s.to_string()).collect();

        let mut iter = input_as_vec.iter();

        let (mut crates, moves) = parse(&mut iter);

        for m in moves {
            crates = m.do_move_by_stack(crates);
            print_crates(&crates);
        }

        itertools::assert_equal(crates[0].iter().map(|c| c.to_string()), vec!["M"]);
        itertools::assert_equal(crates[1].iter().map(|c| c.to_string()), vec!["C"]);
        itertools::assert_equal(
            crates[2].iter().map(|c| c.to_string()),
            vec!["P", "Z", "N", "D"],
        );
    }

    #[test]
    fn test_aoc_5_1() {
        aoc_5_1();
    }

    #[test]
    fn test_aoc_5_2() {
        aoc_5_2();
    }

    fn print_crates(crates: &Crates) {
        for i in 0..crates.len() {
            for j in 0..crates[i].len() {
                println!("{}-{}: {}", i + 1, j + 1, crates[i][j]);
            }
        }
    }
}

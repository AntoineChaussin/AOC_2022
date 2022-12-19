use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;
use Move::{Down, Left, Right, Up};

use crate::get_input;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref MOVE_REGEX: Regex =
                Regex::new("(?P<direction>[UDLR]) (?P<distance>[0-9]+)").unwrap();
        }
        let captured = MOVE_REGEX.captures(value).unwrap();
        match &captured["direction"] {
            "U" => Up(captured["distance"].parse().unwrap()),
            "D" => Down(captured["distance"].parse().unwrap()),
            "R" => Right(captured["distance"].parse().unwrap()),
            "L" => Left(captured["distance"].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

type Operation = Box<dyn FnOnce(Position) -> Position>; //an operation takes a position and moves it by 1

impl Move {
    fn to_op_sequence(&self) -> Vec<Operation> {
        match self {
            Up(d) => (0..*d)
                .map(|_| Box::new(|p: Position| Position::new(p.x, p.y + 1)) as Operation)
                .collect(),
            Down(d) => (0..*d)
                .map(|_| Box::new(|p: Position| Position::new(p.x, p.y - 1)) as Operation)
                .collect(),
            Left(d) => (0..*d)
                .map(|_| Box::new(|p: Position| Position::new(p.x - 1, p.y)) as Operation)
                .collect(),
            Right(d) => (0..*d)
                .map(|_| Box::new(|p: Position| Position::new(p.x + 1, p.y)) as Operation)
                .collect(),
        }
    }
}

struct Chain {
    positions: Vec<Position>,
}

impl Chain {
    fn new() -> Self {
        Chain {
            positions: vec![Position::new(0, 0); 10],
        }
    }

    fn head_mut(&mut self) -> &mut Position {
        self.positions.first_mut().unwrap()
    }

    fn tail(&self) -> &Position {
        self.positions.last().unwrap()
    }

    fn move_chain(&mut self, op: Operation) {
        let head = self.head_mut();
        *head = op(*head);
        for i in 1..self.positions.len() {
            self.positions[i] = move_next(self.positions[i], &self.positions[i - 1]);
        }
    }
}

fn move_next(position: Position, previous_position: &Position) -> Position {
    let dist_x = previous_position.x - position.x;
    let dist_y = previous_position.y - position.y;

    match (dist_x, dist_y) {
        (dx, dy) if i32::abs(dx) + i32::abs(dy) <= 1 => position,
        (dx, dy) if i32::abs(dx) == 1 && i32::abs(dy) == 1 => position,
        (dx, dy) => Position {
            x: position.x + 1 * dx.signum(),
            y: position.y + 1 * dy.signum(),
        },
    }
}

fn tail_positions<I: Iterator<Item = Move>>(moves: I) -> usize {
    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);

    let mut tail_positions = HashSet::new();
    tail_positions.insert(tail.clone());

    for m in moves {
        for o in m.to_op_sequence() {
            head = o(head);
            tail = move_next(tail, &head);
            tail_positions.insert(tail.clone());
        }
    }

    tail_positions.len()
}

fn tail_positions_chain<I: Iterator<Item = Move>>(moves: I) -> usize {
    let mut chain = Chain::new();
    let mut tail_positions = HashSet::new();
    tail_positions.insert(chain.tail().clone());

    for m in moves {
        for o in m.to_op_sequence() {
            chain.move_chain(o);
            tail_positions.insert(chain.tail().clone());
        }
    }

    tail_positions.len()
}

pub fn aoc_9_1() {
    let input = get_input("resource/aoc_9/data.txt");

    let moves = input.iter().map(|s| Into::<Move>::into(s.as_str()));

    let pos_count = tail_positions(moves);

    println!("AOC-9-1 count {}", &pos_count);
}

pub fn aoc_9_2() {
    let input = get_input("resource/aoc_9/data.txt");

    let moves = input.iter().map(|s| Into::<Move>::into(s.as_str()));

    let pos_count = tail_positions_chain(moves);

    println!("AOC-9-2 count chain {}", &pos_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&'static str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const INPUT2: [&'static str; 8] = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    fn step_by_step_test<I: Iterator<Item = (Position, Position)>>(
        mut head: Position,
        mut tail: Position,
        m: Move,
        expected: &mut I,
    ) -> (Position, Position) {
        for o in m.to_op_sequence() {
            head = o(head);
            tail = move_next(tail, &head);

            assert_eq!((head, tail), expected.next().unwrap());
        }

        (head, tail)
    }

    #[test]
    fn test_move() {
        let mut head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);

        (head, tail) = step_by_step_test(
            head,
            tail,
            Right(4),
            &mut vec![
                (Position::new(1, 0), Position::new(0, 0)),
                (Position::new(2, 0), Position::new(1, 0)),
                (Position::new(3, 0), Position::new(2, 0)),
                (Position::new(4, 0), Position::new(3, 0)),
            ]
            .into_iter(),
        );

        (_, _) = step_by_step_test(
            head,
            tail,
            Up(4),
            &mut vec![
                (Position::new(4, 1), Position::new(3, 0)),
                (Position::new(4, 2), Position::new(4, 1)),
                (Position::new(4, 3), Position::new(4, 2)),
                (Position::new(4, 4), Position::new(4, 3)),
            ]
            .into_iter(),
        );
    }

    #[test]
    fn test_parse_move() {
        let input: Vec<Move> = INPUT.iter().map(|s| Into::<Move>::into(*s)).collect();

        let mut head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);

        let mut expected = vec![
            //R 4
            (Position::new(1, 0), Position::new(0, 0)),
            (Position::new(2, 0), Position::new(1, 0)),
            (Position::new(3, 0), Position::new(2, 0)),
            (Position::new(4, 0), Position::new(3, 0)),
            //U 4
            (Position::new(4, 1), Position::new(3, 0)),
            (Position::new(4, 2), Position::new(4, 1)),
            (Position::new(4, 3), Position::new(4, 2)),
            (Position::new(4, 4), Position::new(4, 3)),
            //L 3
            (Position::new(3, 4), Position::new(4, 3)),
            (Position::new(2, 4), Position::new(3, 4)),
            (Position::new(1, 4), Position::new(2, 4)),
            //D 1
            (Position::new(1, 3), Position::new(2, 4)),
            //R 4
            (Position::new(2, 3), Position::new(2, 4)),
            (Position::new(3, 3), Position::new(2, 4)),
            (Position::new(4, 3), Position::new(3, 3)),
            (Position::new(5, 3), Position::new(4, 3)),
            //D 1
            (Position::new(5, 2), Position::new(4, 3)),
            //L 5
            (Position::new(4, 2), Position::new(4, 3)),
            (Position::new(3, 2), Position::new(4, 3)),
            (Position::new(2, 2), Position::new(3, 2)),
            (Position::new(1, 2), Position::new(2, 2)),
            (Position::new(0, 2), Position::new(1, 2)),
            //R 2
            (Position::new(1, 2), Position::new(1, 2)),
            (Position::new(2, 2), Position::new(1, 2)),
        ]
        .into_iter();

        for m in input {
            (head, tail) = step_by_step_test(head, tail, m, &mut expected);
        }
    }

    #[test]
    fn test_count_pos() {
        let input = INPUT.iter().map(|s| Into::<Move>::into(*s));

        let position_count = tail_positions(input);

        assert_eq!(position_count, 13);
    }

    #[test]
    fn test_aoc_9_1() {
        aoc_9_1();
    }

    #[test]
    fn test_count_pos_chain() {
        let input = INPUT.iter().map(|s| Into::<Move>::into(*s));

        let pos_count = tail_positions_chain(input);

        assert_eq!(pos_count, 1);

        let input2 = INPUT2.iter().map(|s| Into::<Move>::into(*s));

        let pos_count2 = tail_positions_chain(input2);

        assert_eq!(pos_count2, 36);
    }

    #[test]
    fn test_aoc_9_2() {
        aoc_9_2();
    }
}

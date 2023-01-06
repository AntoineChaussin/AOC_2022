use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::get_input;

#[derive(Debug)]
enum SquareContent {
    Sand,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

struct Grid {
    content: HashMap<Position, SquareContent>,
    floor: usize,
    infinite_floor: bool,
}

impl Grid {
    fn get_content(&self, position: &Position) -> Option<&SquareContent> {
        if self.infinite_floor && position.y == self.floor {
            Some(&SquareContent::Rock)
        } else {
            self.content.get(position)
        }
    }
}

enum MoveResult {
    Moved(Position),
    AtRest(Position),
    FellDown,
}

fn move_sand_once(position: Position, grid: &Grid) -> MoveResult {
    let (x, y) = (position.x, position.y);

    if y + 1 > grid.floor {
        return MoveResult::FellDown;
    }

    let straight_down = Position::new(x, y + 1);
    let down_left = Position::new(x - 1, y + 1);
    let down_right = Position::new(x + 1, y + 1);

    for new_pos in vec![straight_down, down_left, down_right] {
        match grid.get_content(&new_pos) {
            None => return MoveResult::Moved(new_pos),
            Some(_) => (),
        }
    }

    MoveResult::AtRest(position)
}

fn move_sand(start_position: Position, grid: &mut Grid) -> MoveResult {
    let mut move_result = move_sand_once(start_position, grid);
    while let MoveResult::Moved(new_pos) = move_result {
        move_result = move_sand_once(new_pos, grid);
    }

    if let MoveResult::AtRest(rest_position) = move_result {
        grid.content.insert(rest_position, SquareContent::Sand);
    }

    move_result
}

fn drop_sand(grid: &mut Grid) -> usize {
    let mut res = 0;

    let start_position = Position::new(500, 0);

    let mut at_rest = match move_sand(start_position, grid) {
        MoveResult::AtRest(_) => true,
        MoveResult::FellDown => false,
        _ => unreachable!(),
    };

    while at_rest {
        res += 1;
        at_rest = match move_sand(start_position, grid) {
            MoveResult::AtRest(_) => true,
            MoveResult::FellDown => false,
            _ => unreachable!(),
        };
    }

    res
}

fn fill_with_sand(grid: &mut Grid) -> usize {
    let mut res = 0;

    let start_position = Position::new(500, 0);

    let mut move_result = move_sand(start_position, grid);

    while let MoveResult::AtRest(rest_position) = move_result {
        res += 1;
        if rest_position == start_position {
            break;
        }
        move_result = move_sand(start_position, grid);
    }

    res
}

fn parse<I: Iterator<Item = String>>(input: I) -> Grid {
    let mut content = HashMap::new();
    for line in input {
        let mut rock_positions = parse_line(line).into_iter();
        if let Some(mut current_pos) = rock_positions.next() {
            content.insert(current_pos, SquareContent::Rock);
            while let Some(next_pos) = rock_positions.next() {
                let (start_x, end_x) = if current_pos.x <= next_pos.x {
                    (current_pos.x, next_pos.x)
                } else {
                    (next_pos.x, current_pos.x)
                };
                let (start_y, end_y) = if current_pos.y <= next_pos.y {
                    (current_pos.y, next_pos.y)
                } else {
                    (next_pos.y, current_pos.y)
                };
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        content.insert(Position::new(x, y), SquareContent::Rock);
                    }
                }
                current_pos = next_pos;
            }
        }
    }

    let floor = content.keys().map(|p| p.y).max().unwrap();

    Grid {
        content,
        floor,
        infinite_floor: false,
    }
}

fn parse_line(line: String) -> Vec<Position> {
    let mut res = vec![];
    lazy_static! {
        static ref COORD_REGEX: Regex = Regex::new("(?P<x>[0-9]+),(?P<y>[0-9]+)").unwrap();
    }

    let split = line.split(" -> ");

    for coords in split {
        let captures = COORD_REGEX.captures(coords).unwrap();
        res.push(Position::new(
            captures["x"].parse().unwrap(),
            captures["y"].parse().unwrap(),
        ));
    }

    res
}

pub fn aoc_14_1() {
    let input = get_input("resource/aoc_14/data.txt");
    let mut grid = parse(input.into_iter());

    let sand_count = drop_sand(&mut grid);

    println!("AOC-14-1 sand count : {}", sand_count);
}
pub fn aoc_14_2() {
    let input = get_input("resource/aoc_14/data.txt");
    let mut grid = parse(input.into_iter());

    grid.floor = grid.floor + 2;
    grid.infinite_floor = true;
    let sand_count = fill_with_sand(&mut grid);

    println!("AOC-14-2 fill sand count : {}", sand_count);
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    const INPUT: [&'static str; 2] = [
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn test_parse() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let grid = parse(lines);

        let expected_positions = vec![
            Position::new(498, 4),
            Position::new(498, 5),
            Position::new(498, 6),
            Position::new(497, 6),
            Position::new(496, 6),
            Position::new(503, 4),
            Position::new(502, 4),
            Position::new(502, 5),
            Position::new(502, 6),
            Position::new(502, 7),
            Position::new(502, 8),
            Position::new(502, 9),
            Position::new(501, 9),
            Position::new(500, 9),
            Position::new(499, 9),
            Position::new(498, 9),
            Position::new(497, 9),
            Position::new(496, 9),
            Position::new(495, 9),
            Position::new(494, 9),
        ];

        assert_eq!(grid.content.len(), 20);

        for pos in expected_positions {
            assert_matches!(grid.content.get(&pos), Some(SquareContent::Rock));
        }

        assert_eq!(grid.floor, 9);
    }

    #[test]
    fn test_drop() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let mut grid = parse(lines);

        assert_eq!(drop_sand(&mut grid), 24);
    }

    #[test]
    fn test_fill() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let mut grid = parse(lines);

        grid.floor = grid.floor + 2;
        grid.infinite_floor = true;

        assert_eq!(fill_with_sand(&mut grid), 93);
    }

    #[test]
    fn test_aoc_14_1() {
        aoc_14_1()
    }
    #[test]
    fn test_aoc_14_2() {
        aoc_14_2()
    }
}

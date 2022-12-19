use std::fmt::Display;
use Direction::{East, North, South, West};

use crate::get_input;

type TreeSize = i32;
type ScenicDistance = u32;
type ScenicScore = u32;

struct ElvenMap {
    map: Vec<Vec<TreeSize>>,
    max_col: usize,
    max_row: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct MaxByDirection {
    north: Option<TreeSize>,
    south: Option<TreeSize>,
    west: Option<TreeSize>,
    east: Option<TreeSize>,
}

impl MaxByDirection {
    fn new() -> Self {
        MaxByDirection {
            north: None,
            south: None,
            west: None,
            east: None,
        }
    }

    fn update(&mut self, dir: &Direction, value: TreeSize) {
        match dir {
            North => self.north = Some(value),
            South => self.south = Some(value),
            East => self.east = Some(value),
            West => self.west = Some(value),
        }
    }

    fn get_direction(&self, dir: &Direction) -> Option<TreeSize> {
        match dir {
            North => self.north,
            South => self.south,
            East => self.east,
            West => self.west,
        }
    }

    fn check_visible(&self, tree_size: TreeSize) -> bool {
        let (n, s, w, e) = (
            self.north.unwrap(),
            self.south.unwrap(),
            self.west.unwrap(),
            self.east.unwrap(),
        );
        tree_size > n || tree_size > s || tree_size > w || tree_size > e
    }
}

type MaxMap = Vec<Vec<MaxByDirection>>;

impl ElvenMap {
    fn new<I: Iterator<Item = String>>(input: I) -> ElvenMap {
        let mut result = vec![];
        let mut max_col = 0;

        for line in input {
            let mut row: Vec<TreeSize> = vec![];
            for character in line.chars() {
                row.push(character.to_digit(10).unwrap().try_into().unwrap());
            }
            max_col = row.len();
            result.push(row);
        }
        let max_row = result.len();

        ElvenMap {
            map: result,
            max_col,
            max_row,
        }
    }

    fn make_max_map(&self) -> MaxMap {
        let mut max_map: MaxMap = vec![vec![MaxByDirection::new(); self.max_col]; self.max_row];

        for row in 0..self.max_row {
            for col in 0..self.max_col {
                //left to right up to down > do north and west
                Self::update(&mut max_map, row, col, self, &Direction::North);
                Self::update(&mut max_map, row, col, self, &Direction::West);

                //right to left and down to up > do south and east
                let reverse_row = self.max_row - 1 - row;
                let reverse_col = self.max_col - 1 - col;
                Self::update(
                    &mut max_map,
                    reverse_row,
                    reverse_col,
                    self,
                    &Direction::South,
                );
                Self::update(
                    &mut max_map,
                    reverse_row,
                    reverse_col,
                    self,
                    &Direction::East,
                );
            }
        }

        max_map
    }

    fn update(max_map: &mut MaxMap, row: usize, col: usize, elf_map: &ElvenMap, dir: &Direction) {
        if max_map[row][col].get_direction(dir).is_none() {
            let updated_max = if elf_map.get_boundary_check(dir)(row, col) {
                TreeSize::MIN
            } else {
                let (t_row, t_col) = dir.move_coordinates(row, col);
                let previous_max = max_map[t_row][t_col].get_direction(dir);
                assert!(
                    previous_max.is_some(),
                    "not is some {} {} {:?}",
                    t_row,
                    t_col,
                    &dir
                );
                let previous_max = previous_max.unwrap();
                let adjacent_value = elf_map.map[t_row][t_col];

                adjacent_value.max(previous_max)
            };

            max_map[row][col].update(dir, updated_max);
        }
    }

    fn get_boundary_check<'a>(&'a self, dir: &Direction) -> Box<dyn Fn(usize, usize) -> bool + 'a> {
        match dir {
            North => Box::new(|row, _col| row == 0),
            South => Box::new(|row, _col| row == self.max_row - 1),
            West => Box::new(|_row, col| col == 0),
            East => Box::new(|_row, col| col == self.max_col - 1),
        }
    }

    fn count_visible(&self) -> usize {
        let max_map = self.make_max_map();

        self.map
            .iter()
            .zip(max_map.iter())
            .fold(0, |acc, (heights, maxes)| {
                acc + heights
                    .iter()
                    .zip(maxes.iter())
                    .filter(|(height, max)| max.check_visible(**height))
                    .count() //counts visible trees in one row
            })
    }

    fn calculate_scenic_distance(&self, row: usize, col: usize, dir: Direction) -> ScenicDistance {
        let height = self.map[row][col];
        let check_bound = self.get_boundary_check(&dir);

        if check_bound(row, col) {
            0
        } else {
            let mut dist = 1;
            let (mut t_row, mut t_col) = dir.move_coordinates(row, col);
            while !check_bound(t_row, t_col) && height > self.map[t_row][t_col] {
                dist += 1;
                (t_row, t_col) = dir.move_coordinates(t_row, t_col);
            }
            dist
        }
    }

    fn calculate_scenic_score(&self, row: usize, col: usize) -> ScenicScore {
        vec![North, South, East, West]
            .into_iter()
            .map(|dir| self.calculate_scenic_distance(row, col, dir))
            .fold(1, |acc, sd| acc * sd)
    }

    fn max_scenic_score(&self) -> ScenicScore {
        let mut max_score = ScenicScore::MIN;
        for i in 0..self.max_row {
            for j in 0..self.max_col {
                let score = self.calculate_scenic_score(i, j);
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }
}
#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_coordinates(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            North => (row - 1, col),
            South => (row + 1, col),
            East => (row, col + 1),
            West => (row, col - 1),
        }
    }
}

impl Display for ElvenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = &self.map;
        for row in map {
            for ts in row {
                write!(f, "{}", ts)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn aoc_8_1() {
    let input = get_input("resource/aoc_8/data.txt");

    let elven_map = ElvenMap::new(input.into_iter());
    let count = elven_map.count_visible();

    println!("AOC-8-1 count visible: {}", &count);
}

pub fn aoc_8_2() {
    let input = get_input("resource/aoc_8/data.txt");

    let elven_map = ElvenMap::new(input.into_iter());
    let max_score = elven_map.max_scenic_score();

    println!("AOC-8-2 max score: {}", &max_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&'static str; 5] = ["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn test_parse() {
        let input = INPUT.iter().map(|s| s.to_string());

        let map = ElvenMap::new(input);

        assert_eq!(5, map.max_col);
        assert_eq!(5, map.max_row);

        let expected = "30373\n\
                              25512\n\
                              65332\n\
                              33549\n\
                              35390\n";
        assert_eq!(expected, map.to_string().as_str());
    }

    #[test]
    fn test_max_map() {
        let input = INPUT.iter().map(|s| s.to_string());

        let map = ElvenMap::new(input);

        let max_map = map.make_max_map();

        let m = TreeSize::MIN;

        let expected = vec![
            vec![
                nswe(m, 6, m, 7),
                nswe(m, 5, 3, 7),
                nswe(m, 5, 3, 7),
                nswe(m, 9, 3, 3),
                nswe(m, 9, 7, m),
            ],
            vec![
                nswe(3, 6, m, 5),
                nswe(0, 5, 2, 5),
                nswe(3, 5, 5, 2),
                nswe(7, 9, 5, 2),
                nswe(3, 9, 5, m),
            ],
            vec![
                nswe(3, 3, m, 5),
                nswe(5, 5, 6, 3),
                nswe(5, 5, 6, 3),
                nswe(7, 9, 6, 2),
                nswe(3, 9, 6, m),
            ],
            vec![
                nswe(6, 3, m, 9),
                nswe(5, 5, 3, 9),
                nswe(5, 3, 3, 9),
                nswe(7, 9, 5, 9),
                nswe(3, 0, 5, m),
            ],
            vec![
                nswe(6, m, m, 9),
                nswe(5, m, 3, 9),
                nswe(5, m, 5, 9),
                nswe(7, m, 5, 0),
                nswe(9, m, 9, m),
            ],
        ];

        itertools::assert_equal(max_map, expected);
    }

    fn nswe(n: TreeSize, s: TreeSize, w: TreeSize, e: TreeSize) -> MaxByDirection {
        MaxByDirection {
            north: Some(n),
            south: Some(s),
            west: Some(w),
            east: Some(e),
        }
    }

    #[test]
    fn test_count_visible() {
        let input = INPUT.iter().map(|s| s.to_string());

        let map = ElvenMap::new(input);
        let count = map.count_visible();

        assert_eq!(count, 21);
    }

    #[test]
    fn test_aoc_8_1() {
        aoc_8_1()
    }

    #[test]
    fn test_scenic_score() {
        let input = INPUT.iter().map(|s| s.to_string());

        let map = ElvenMap::new(input);

        let sscore = map.calculate_scenic_score(3, 2);
        assert_eq!(sscore, 8);

        let sscore = map.calculate_scenic_score(1, 2);
        assert_eq!(sscore, 4);

        let max = map.max_scenic_score();

        assert_eq!(max, 8);
    }
    
    #[test]
    fn test_aoc_8_2() {
        aoc_8_2()
    }
}

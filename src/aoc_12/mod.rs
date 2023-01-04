use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeSet, HashMap},
    hash::Hash,
    rc::Rc,
};

use crate::get_input;

type Grid = Vec<Vec<Square>>;
type NodeRef = Rc<RefCell<GraphNode>>;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.row.cmp(&other.row);
        match cmp {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            _ => cmp,
        }
    }
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Square {
    elevation: char,
    is_start: bool,
    is_goal: bool,
}

impl Square {
    fn can_move_up(current: &Square, destination: &Square) -> bool {
        let (el_current, el_dest) = (current.elevation as i32, destination.elevation as i32);
        el_dest - el_current <= 1
    }
    fn can_move_down(current: &Square, destination: &Square) -> bool {
        let (el_current, el_dest) = (current.elevation as i32, destination.elevation as i32);
        el_current - el_dest <= 1
    }
}

#[derive(PartialEq, Eq, Debug)]
struct GraphNode {
    visited: bool,
    tentative_dist: usize,
    position: Position,
    elevation: char,
    is_goal: bool,
    is_start: bool,
    neighbors: Vec<NodeRef>,
}

#[derive(Clone, Debug, PartialOrd)]
struct PosAndDist {
    position: Position,
    tentative_dist: usize,
}

impl PartialEq for PosAndDist {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for PosAndDist {}

impl Ord for PosAndDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.tentative_dist.cmp(&other.tentative_dist);
        match cmp {
            std::cmp::Ordering::Equal => self.position.cmp(&other.position),
            _ => cmp,
        }
    }
}

impl Hash for PosAndDist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl GraphNode {
    fn new(position: Position, is_goal: bool, is_start: bool, elevation: char) -> GraphNode {
        GraphNode {
            visited: false,
            tentative_dist: usize::MAX,
            is_goal,
            is_start,
            position,
            elevation,
            neighbors: vec![],
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn move_pos(&self, position: &Position, max_row: usize, max_col: usize) -> Option<Position> {
        match self {
            Direction::North if position.row == 0 => None,
            Direction::South if position.row == max_row => None,
            Direction::East if position.col == max_col => None,
            Direction::West if position.col == 0 => None,
            Direction::North => Some(Position::new(position.row - 1, position.col)),
            Direction::South => Some(Position::new(position.row + 1, position.col)),
            Direction::West => Some(Position::new(position.row, position.col - 1)),
            Direction::East => Some(Position::new(position.row, position.col + 1)),
        }
    }
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            'S' => Square {
                elevation: 'a',
                is_start: true,
                is_goal: false,
            },
            'E' => Square {
                elevation: 'z',
                is_start: false,
                is_goal: true,
            },
            c if c >= 'a' && c <= 'z' => Square {
                elevation: c,
                is_start: false,
                is_goal: false,
            },
            _ => unreachable!(),
        }
    }
}

fn parse_up<I: Iterator<Item = String>>(iter: I) -> HashMap<Position, Rc<RefCell<GraphNode>>> {
    let mut res = vec![];
    for s in iter {
        let mut row: Vec<Square> = vec![];
        for c in s.chars() {
            row.push(c.into());
        }
        res.push(row);
    }

    let graph = create_graph(res, Square::can_move_up);
    graph
}

fn parse_down<I: Iterator<Item = String>>(iter: I) -> HashMap<Position, Rc<RefCell<GraphNode>>> {
    let mut res = vec![];
    for s in iter {
        let mut row: Vec<Square> = vec![];
        for c in s.chars() {
            row.push(c.into());
        }
        res.push(row);
    }

    let graph = create_graph(res, Square::can_move_down);
    graph
}

fn find<F: Fn(&Square) -> bool>(grid: &Grid, f: F) -> Position {
    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            if f(&row[j]) {
                return Position::new(i, j);
            }
        }
    }

    unreachable!()
}

fn create_unvisited_set(nodes: &HashMap<Position, NodeRef>) -> BTreeSet<PosAndDist> {
    let mut set = BTreeSet::new();

    for n in nodes.values() {
        let n = n.borrow();
        set.insert(PosAndDist {
            position: n.position,
            tentative_dist: n.tentative_dist,
        });
    }

    assert_eq!(nodes.values().len(), set.len());

    set
}

fn create_graph<F: Fn(&Square, &Square) -> bool>(
    grid: Grid,
    can_move: F,
) -> HashMap<Position, NodeRef> {
    let mut res: HashMap<Position, NodeRef> = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let square = &grid[row][col];
            let position = Position::new(row, col);

            if let None = res.get(&position) {
                let new_ref =
                    GraphNode::new(position, square.is_goal, square.is_start, square.elevation);
                let new_ref = Rc::new(RefCell::new(new_ref));
                res.insert(position, new_ref);
            }

            for d in DIRECTIONS {
                if let Some(new_pos) = d.move_pos(&position, grid.len() - 1, grid[row].len() - 1) {
                    let destination = &grid[new_pos.row][new_pos.col];
                    if can_move(square, destination) {
                        let node_ref = if let Some(existing) = res.get(&new_pos) {
                            //we already saw the node
                            existing.clone()
                        } else {
                            let new_neighbor = GraphNode::new(
                                new_pos,
                                destination.is_goal,
                                destination.is_start,
                                destination.elevation,
                            );
                            let new_ref = Rc::new(RefCell::new(new_neighbor));
                            res.insert(new_pos, new_ref.clone());
                            new_ref
                        };
                        //update neighbors
                        let mut current_node = res.get(&position).unwrap().borrow_mut();
                        current_node.neighbors.push(node_ref.clone());
                    }
                }
            }
        }
    }

    res
}

fn prep_min_route<F: Fn(&RefMut<GraphNode>) -> bool>(
    nodes: &mut HashMap<Position, NodeRef>,
    start_check: F,
) {
    for nr in nodes.values() {
        let mut nr_mut = nr.borrow_mut();
        nr_mut.tentative_dist = if start_check(&nr_mut) { 0 } else { usize::MAX };
        nr_mut.visited = false;
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

fn min_route<F: Fn(&Ref<GraphNode>) -> bool>(
    nodes: &HashMap<Position, NodeRef>,
    is_goal: F,
) -> Option<usize> {
    let mut unvisited_set = create_unvisited_set(nodes);
    while let Some(current_pad) = unvisited_set.pop_first() {
        let current = nodes.get(&current_pad.position).unwrap();
        {
            let current = current.borrow();

            if current.tentative_dist == usize::MAX {
                for n in unvisited_set {
                    println!("unvisited {} {}", n.position.row, n.position.col);
                }
                return None;
            }

            if is_goal(&current) {
                return Some(current.tentative_dist);
            }

            for neighbor in &current.neighbors {
                if !neighbor.borrow().visited {
                    let before = unvisited_set.len();
                    unvisited_set.retain(|pad| pad.position != neighbor.borrow().position);
                    {
                        let mut neighbor_mut = neighbor.borrow_mut();
                        neighbor_mut.tentative_dist =
                            usize::min(neighbor_mut.tentative_dist, 1 + current.tentative_dist);
                    }
                    unvisited_set.insert(PosAndDist {
                        position: neighbor.borrow().position,
                        tentative_dist: neighbor.borrow().tentative_dist,
                    });

                    assert_eq!(before, unvisited_set.len());
                }
            }
        }

        {
            let mut current_mut = current.borrow_mut();
            current_mut.visited = true;
        }

        /*println!("----------------------------");
        for pad in unvisited_set.iter() {
            println!(
                "Pos {} {} dist {}",
                pad.position.row, pad.position.col, pad.tentative_dist
            );
        }*/
    }

    unreachable!()
}

pub fn aoc_12_1() {
    let input = get_input("resource/aoc_12/data.txt");

    let mut nodes = parse_up(input.into_iter());

    prep_min_route(&mut nodes, |n| n.is_start);

    let min = min_route(&nodes, |n| n.is_goal);

    println!("AOC-12-1 shortest route {}", min.unwrap());
}

pub fn aoc_12_2() {
    let input = get_input("resource/aoc_12/data.txt");

    let mut nodes = parse_down(input.into_iter());

    prep_min_route(&mut nodes, |n| n.is_goal);
    let min = min_route(&nodes, |n| n.elevation == 'a');

    println!("AOC-12-2 shortest among low elevation {}", min.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&'static str; 5] = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn test_parse() {
        let input = INPUT.map(|s| s.to_string());
        let graph = parse_up(input.into_iter());

        assert_eq!(
            graph.get(&Position::new(0, 0)).unwrap().borrow().is_start,
            true
        );
        assert_eq!(
            graph
                .get(&Position::new(0, 0))
                .unwrap()
                .borrow()
                .neighbors
                .len(),
            2
        );
        assert_eq!(
            graph.get(&Position::new(2, 5)).unwrap().borrow().is_goal,
            true
        );
        assert_eq!(
            graph
                .get(&Position::new(2, 5))
                .unwrap()
                .borrow()
                .neighbors
                .len(),
            4
        );
    }

    #[test]
    fn test_min() {
        let mut grid = parse_up(INPUT.map(|s| s.to_string()).into_iter());

        prep_min_route(&mut grid, |n| n.is_start);

        assert_eq!(min_route(&grid, |n| n.is_goal).unwrap(), 31);
    }

    #[test]
    fn test_multi_min() {
        let mut grid = parse_down(INPUT.map(|s| s.to_string()).into_iter());
        prep_min_route(&mut grid, |n| n.is_goal);
        let min = min_route(&grid, |n| n.elevation == 'a');

        assert_eq!(min.unwrap(), 29);
    }

    #[test]
    fn test_aoc_12_1() {
        aoc_12_1()
    }

    #[test]
    fn test_aoc_12_2() {
        aoc_12_2()
    }
}

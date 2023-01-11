use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::get_input;

type Distance = u32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn manhattan_dist(&self, other: &Point) -> Distance {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Circle {
    center: Point,
    sensor_pos: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Interval {
    low: i32,
    high: i32,
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let compare_low = self.low.cmp(&other.low);
        match compare_low {
            std::cmp::Ordering::Equal => Some(self.high.cmp(&other.high)),
            _ => Some(compare_low),
        }
    }
}

impl Interval {
    fn new(low: i32, high: i32) -> Self {
        Interval { low, high }
    }

    fn merge(self, other: Interval) -> IntervalType {
        match (self, other) {
            (Interval { low: sl, high: sh }, Interval { low: ol, high: oh })
                if sl > oh + 1 || ol > sh + 1 =>
            {
                let (min, max) = if &self < &other {
                    (self, other)
                } else {
                    (other, self)
                };
                IntervalType::Disjoined(min, max)
            }
            _ => IntervalType::Joined(Interval {
                low: i32::min(self.low, other.low),
                high: i32::max(self.high, other.high),
            }),
        }
    }

    fn disjoin(&self, other: &Interval) -> Option<IntervalType> {
        match (self.low, self.high, other.low, other.high) {
            (sl, _, _, oh) if sl > oh => Some(IntervalType::Joined(*self)),
            (_, sh, ol, _) if sh < ol => Some(IntervalType::Joined(*self)),
            (sl, sh, ol, oh) if sl >= ol && sh <= oh => None,
            (sl, sh, ol, oh) if sl < ol && sh > oh => Some(IntervalType::Disjoined(
                Interval::new(sl, ol - 1),
                Interval::new(oh + 1, sh),
            )),
            (sl, sh, ol, oh) if sl < ol && sh <= oh => {
                Some(IntervalType::Joined(Interval::new(sl, ol - 1)))
            }
            (sl, sh, ol, oh) if sl >= ol && sh > oh => {
                Some(IntervalType::Joined(Interval::new(oh + 1, sh)))
            }
            _ => unreachable!(),
        }
    }
}

struct IntervalList {
    intervals: Vec<Interval>,
    low_bound: i32,
    high_bound: i32,
}

impl IntervalList {
    fn insert(self, interval: Interval) -> IntervalList {
        if interval.high < self.low_bound || interval.low > self.high_bound {
            return self;
        }
        let mut current_interval = Interval::new(
            interval.low.max(self.low_bound),
            interval.high.min(self.high_bound),
        );

        let mut after_insert = vec![];
        let mut intervals_iter = self.intervals.into_iter();

        while let Some(next_interval) = intervals_iter.next() {
            current_interval = match next_interval.merge(current_interval) {
                IntervalType::Joined(merged) => merged,
                IntervalType::Disjoined(min, max) => {
                    after_insert.push(min);
                    max
                }
            }
        }

        after_insert.push(current_interval);

        IntervalList {
            intervals: after_insert,
            low_bound: self.low_bound,
            high_bound: self.high_bound,
        }
    }
}

#[derive(Debug)]
enum IntervalType {
    Joined(Interval),
    Disjoined(Interval, Interval),
}

impl Circle {
    fn new(center: Point, sensor_pos: Point) -> Self {
        Circle { center, sensor_pos }
    }

    fn radius(&self) -> u32 {
        self.center.manhattan_dist(&self.sensor_pos)
    }

    fn min_x(&self) -> i32 {
        self.center.x.checked_sub_unsigned(self.radius()).unwrap()
    }
    fn max_x(&self) -> i32 {
        self.center.x.checked_add_unsigned(self.radius()).unwrap()
    }
    fn min_y(&self) -> i32 {
        self.center.y.checked_sub_unsigned(self.radius()).unwrap()
    }
    fn max_y(&self) -> i32 {
        self.center.y.checked_add_unsigned(self.radius()).unwrap()
    }

    /// Returns the segment created by the intersection of the circle with
    /// an horizontal line at y
    fn intersect_with_y(&self, y: i32) -> Option<Interval> {
        if y > self.max_y() || y < self.min_y() {
            None
        } else {
            let y_diff = y.abs_diff(self.center.y) as i32;
            let radius = self.radius() as i32;
            Some(Interval::new(
                self.center.x + y_diff - radius,
                self.center.x + radius - y_diff,
            ))
        }
    }
}

struct Field {
    circles: Vec<Circle>,
    min_x: i32,
    max_x: i32,
}

impl Field {
    fn count_unchecked(&self, line_number: i32) -> i32 {
        let merged_intervals = self.covered_intervals(line_number, self.min_x, self.max_x);

        let covered_including_beacons = merged_intervals
            .intervals
            .iter()
            .fold(0, |acc, i| acc + i.high - i.low + 1);

        let nb_of_beacons: i32 = self
            .circles
            .iter()
            .filter_map(|c| {
                if c.sensor_pos.y == line_number {
                    Some(c.sensor_pos)
                } else {
                    None
                }
            })
            .collect::<HashSet<Point>>()
            .len()
            .try_into()
            .unwrap();

        covered_including_beacons - nb_of_beacons
    }

    fn covered_intervals(&self, y: i32, low_bound: i32, high_bound: i32) -> IntervalList {
        let intersections = self.circles.iter().map(|c| c.intersect_with_y(y)).flatten();

        let mut merged_intervals = IntervalList {
            intervals: vec![],
            low_bound,
            high_bound,
        };

        for interval in intersections {
            merged_intervals = merged_intervals.insert(interval);
        }

        merged_intervals
    }

    fn find_uncovered(
        &self,
        x_interval: Interval,
        y_interval: Interval,
    ) -> HashMap<i32, Vec<Interval>> {
        let mut res = HashMap::new();
        for y in y_interval.low..=y_interval.high {
            let covered = self.covered_intervals(y, x_interval.low, x_interval.high);
            let uncovered =
                covered
                    .intervals
                    .iter()
                    .fold(vec![x_interval], |acc, covered_interval| {
                        acc.into_iter()
                            .map(|uncovered_interval| uncovered_interval.disjoin(covered_interval))
                            .flatten()
                            .map(|disj| match disj {
                                IntervalType::Joined(single) => vec![single],
                                IntervalType::Disjoined(disj1, disj2) => vec![disj1, disj2],
                            })
                            .flatten()
                            .collect()
                    });
            if !uncovered.is_empty() {
                res.insert(y, uncovered);
            }
        }

        res
    }
}

fn parse<I: Iterator<Item = String>>(input: I) -> Field {
    let mut circles = vec![];
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);

    lazy_static! {
        static ref SENSOR_REGEX: Regex = Regex::new("Sensor at x=(?P<center_x>-?[0-9]+), y=(?P<center_y>-?[0-9]+): closest beacon is at x=(?P<radius_x>-?[0-9]+), y=(?P<radius_y>-?[0-9]+)").unwrap();
    }
    for line in input {
        let captures = SENSOR_REGEX.captures(&line).unwrap();
        let center = Point::new(
            captures["center_x"].parse().unwrap(),
            captures["center_y"].parse().unwrap(),
        );
        let sensor = Point::new(
            captures["radius_x"].parse().unwrap(),
            captures["radius_y"].parse().unwrap(),
        );

        let circle = Circle::new(center, sensor);

        min_x = min_x.min(circle.min_x());
        max_x = max_x.max(circle.max_x());

        circles.push(circle);
    }

    Field {
        circles,
        min_x,
        max_x,
    }
}

pub fn aoc_15_1() {
    let input = get_input("resource/aoc_15/data.txt");
    let field = parse(input.into_iter());

    let unchecked = field.count_unchecked(2000000);

    println!("AOC-15-1 unchecked count: {}", unchecked)
}
pub fn aoc_15_2() {
    let input = get_input("resource/aoc_15/data.txt");
    let field = parse(input.into_iter());

    let uncovered = field.find_uncovered(Interval::new(0, 4000000), Interval::new(0, 4000000));

    assert!(uncovered.len() == 1);
    let pos: Vec<(i64, i64)> = uncovered
        .into_iter()
        .map(|(y, intervals)| {
            assert!(intervals.len() == 1);
            let i = intervals[0];
            assert!(i.low == i.high);
            (i.low as i64, y as i64)
        })
        .collect();

    assert!(pos.len() == 1);

    let pos = pos[0];

    println!("AOC-15-2 beacon freq: {}", pos.0 * 4000000 + pos.1)
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use itertools::assert_equal;

    use super::IntervalType::{Disjoined, Joined};
    use super::*;

    const INPUT: [&'static str; 14] = [
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn test_parse() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let field = parse(lines);

        assert_eq!(field.circles.len(), 14);

        assert_eq!(
            field.circles[6],
            Circle {
                center: Point::new(8, 7),
                sensor_pos: Point::new(2, 10)
            }
        );

        assert_eq!(field.min_x, -8);
        assert_eq!(field.max_x, 28);
    }

    #[test]
    fn test_count_unchecked() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let field = parse(lines);

        assert_eq!(field.count_unchecked(10), 26);
    }

    #[test]
    fn test_aoc_15_1() {
        aoc_15_1()
    }
    #[test]
    fn test_aoc_15_2() {
        aoc_15_2()
    }

    #[test]
    fn test_interval_merge() {
        let i1 = Interval::new(0, 5);
        let i2 = Interval::new(-3, 4);

        assert_matches!(i1.merge(i2), Joined(Interval { low: -3, high: 5 }));

        let i1 = Interval::new(0, 5);
        let i2 = Interval::new(-3, -2);
        assert_matches!(
            i1.merge(i2),
            Disjoined(Interval { low: -3, high: -2 }, Interval { low: 0, high: 5 })
        );

        let i1 = Interval::new(0, 5);
        let i2 = Interval::new(6, 8);
        assert_matches!(i1.merge(i2), Joined(Interval { low: 0, high: 8 }));
    }

    #[test]
    fn test_interval_insert() {
        let mut list = IntervalList {
            intervals: vec![],
            low_bound: 0,
            high_bound: 20,
        };

        let intervals = vec![
            Interval::new(-3, -2),
            Interval::new(-1, 3),
            Interval::new(1, 3),
            Interval::new(6, 9),
            Interval::new(5, 10),
            Interval::new(11, 12),
            Interval::new(14, 16),
        ];

        for i in intervals {
            list = list.insert(i);
        }

        assert_equal(
            list.intervals,
            vec![
                Interval::new(0, 3),
                Interval::new(5, 12),
                Interval::new(14, 16),
            ],
        );
    }

    #[test]
    fn test_intersect() {
        let circle = Circle::new(Point::new(0, 0), Point::new(0, 5));

        assert_eq!(circle.intersect_with_y(0), Some(Interval::new(-5, 5)));
        assert_eq!(circle.intersect_with_y(5), Some(Interval::new(0, 0)));
        assert_eq!(circle.intersect_with_y(-5), Some(Interval::new(0, 0)));
        assert_eq!(circle.intersect_with_y(1), Some(Interval::new(-4, 4)));
        assert_eq!(circle.intersect_with_y(-1), Some(Interval::new(-4, 4)));
    }

    #[test]
    fn test_find_covered() {
        let lines = INPUT.iter().map(|s| s.to_string());

        let field = parse(lines);

        let uncovered = field.find_uncovered(Interval::new(0, 20), Interval::new(0, 20));

        assert_eq!(uncovered.get(&11).unwrap()[0], Interval::new(14, 14))
    }
}

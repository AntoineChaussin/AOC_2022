use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

use crate::get_input;

type WorryLevel = u64;
type MonkeyNb = usize;
type UpdateFunction = Box<dyn Fn(WorryLevel) -> WorryLevel>;
struct Monkey {
    nb: MonkeyNb,
    worry_lvs: VecDeque<WorryLevel>,
    update_fn: UpdateFunction,
    divide_test: WorryLevel, //to be coherent with dimensions
    on_succ: MonkeyNb,
    on_fail: MonkeyNb,
    dampen_func: UpdateFunction,
}

impl Monkey {
    fn inspect(&mut self) -> Option<(MonkeyNb, WorryLevel)> {
        if let Some(inspected) = self.worry_lvs.pop_front() {
            let updated_worry = (self.dampen_func)((self.update_fn)(inspected));
            let new_monkey = if updated_worry % self.divide_test == 0 {
                self.on_succ
            } else {
                self.on_fail
            };

            Some((new_monkey, updated_worry))
        } else {
            None
        }
    }
}

fn process_monkey(m: MonkeyNb, monkeys: &mut Vec<Monkey>) -> u64 {
    let mut nb_inspected = 0;
    while let Some((new_monkey, updated_worry)) = monkeys[m].inspect() {
        nb_inspected += 1;
        monkeys[new_monkey].worry_lvs.push_back(updated_worry);
    }

    nb_inspected
}

fn update_dampen_func(monkeys: &mut Vec<Monkey>) {
    let dampen_coeff = monkeys.iter().fold(1, |acc, m| acc * m.divide_test);

    for m in monkeys {
        m.dampen_func = Box::new(move |x| x % dampen_coeff);
    }
}

fn play_n_rounds(monkeys: &mut Vec<Monkey>, n: u32) -> u64 {
    let mut inspects: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..n {
        for m in 0..monkeys.len() {
            let nb_inspected = process_monkey(m, monkeys);
            inspects[m] += nb_inspected;
        }
    }

    inspects.sort();

    let n = inspects.len();
    inspects[n - 1] * inspects[n - 2]
}

fn play_20_rounds(monkeys: &mut Vec<Monkey>) -> u64 {
    play_n_rounds(monkeys, 20)
}

pub fn aoc_11_1() {
    let input = get_input("resource/aoc_11/data.txt");
    let mut monkeys = parse(input.into_iter());

    let monkey_biz = play_20_rounds(&mut monkeys);

    println!("AOC-11-1 monkey biz {}", monkey_biz);
}

pub fn aoc_11_2() {
    let input = get_input("resource/aoc_11/data.txt");
    let mut monkeys = parse(input.into_iter());

    update_dampen_func(&mut monkeys);
    let monkey_biz = play_n_rounds(&mut monkeys, 10000);

    println!("AOC-11-2 monkey biz 10000 {}", monkey_biz);
}

fn parse<I: Iterator<Item = String>>(iter: I) -> Vec<Monkey> {
    let mut res = vec![];
    let mut peekable = iter.peekable();

    while peekable.peek().is_some() {
        let next_6 = peekable.by_ref().take(6);
        let monkey: Monkey = FromIterator::from_iter(next_6);
        res.push(monkey);

        _ = peekable.next(); //skip 1
    }

    res
}

impl FromIterator<String> for Monkey {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        lazy_static! {
            static ref MONKEY_REGEX: Regex = Regex::new("Monkey (?P<monkeynb>[0-9]+)").unwrap();
            static ref ITEMS_REGEX: Regex =
                Regex::new("Starting items: (?P<items>(([0-9]+)(, )*)+)").unwrap();
            static ref OPERATION_REGEX: Regex = Regex::new(
                r"Operation: new = (?P<var1>old|[0-9]+) (?P<operator>[\+\*-/]) (?P<var2>old|[0-9]+)"
            )
            .unwrap();
            static ref TEST_REGEX: Regex =
                Regex::new("Test: divisible by (?P<test>[0-9]+)").unwrap();
            static ref SUCC_REGEX: Regex =
                Regex::new("If true: throw to monkey (?P<succ>[0-9]+)").unwrap();
            static ref FAIL_REGEX: Regex =
                Regex::new("If false: throw to monkey (?P<fail>[0-9]+)").unwrap();
        }

        let mut iter = iter.into_iter();
        //line1
        let line1 = iter.next().unwrap();
        let nb: MonkeyNb = MONKEY_REGEX.captures(&line1).unwrap()["monkeynb"]
            .parse()
            .unwrap();
        //line2
        let line2 = iter.next().unwrap();
        let worry_lvs: VecDeque<WorryLevel> = ITEMS_REGEX.captures(&line2).unwrap()["items"]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        //line3
        let line3 = iter.next().unwrap();
        let captures_3 = OPERATION_REGEX.captures(&line3).unwrap();
        let (var1, operator, var2) = (
            captures_3["var1"].to_string(),
            captures_3["operator"].to_string(),
            captures_3["var2"].to_string(),
        );
        let update_fn = Box::new(move |x: WorryLevel| {
            let v1 = match var1.as_str() {
                "old" => x,
                _ => var1.parse::<WorryLevel>().unwrap(),
            };
            let v2 = match var2.as_str() {
                "old" => x,
                _ => var2.parse::<WorryLevel>().unwrap(),
            };
            match operator.as_str() {
                "*" => v1 * v2,
                "+" => v1 + v2,
                "-" => v1 - v2,
                "/" => v1 / v2,
                _ => unreachable!(),
            }
        });
        //line 4
        let line4 = iter.next().unwrap();
        let divide_test: WorryLevel = TEST_REGEX.captures(&line4).unwrap()["test"]
            .parse()
            .unwrap();
        //line5
        let line5 = iter.next().unwrap();
        let on_succ: MonkeyNb = SUCC_REGEX.captures(&line5).unwrap()["succ"]
            .parse()
            .unwrap();
        //line6
        let line6 = iter.next().unwrap();
        let on_fail: MonkeyNb = FAIL_REGEX.captures(&line6).unwrap()["fail"]
            .parse()
            .unwrap();

        Monkey {
            nb,
            worry_lvs,
            update_fn,
            divide_test,
            on_succ,
            on_fail,
            dampen_func: Box::new(|x| x / 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn test_parse() {
        let input = INPUT.split("\n").map(|s| s.to_string());
        let res = parse(input);

        let expected = vec![
            MonkeyAsserts::new(0, vec![79, 98], 38, 23, 2, 3),
            MonkeyAsserts::new(1, vec![54, 65, 75, 74], 8, 19, 2, 0),
            MonkeyAsserts::new(2, vec![79, 60, 97], 4, 13, 1, 3),
            MonkeyAsserts::new(3, vec![74], 5, 17, 0, 1),
        ];

        itertools::assert_equal(res.into_iter().map(|m| MonkeyAsserts::from(m)), expected);
    }

    #[test]
    fn test_20_rounds() {
        let input = INPUT.split("\n").map(|s| s.to_string());
        let mut res = parse(input);

        let monkey_biz = play_20_rounds(&mut res);

        assert_eq!(monkey_biz, 10605);
    }

    #[test]
    fn test_10000_rounds() {
        let input = INPUT.split("\n").map(|s| s.to_string());
        let mut res = parse(input);

        update_dampen_func(&mut res);

        let monkey_biz = play_n_rounds(&mut res, 10000);

        assert_eq!(monkey_biz, 2713310158);
    }

    #[test]
    fn test_aoc_11_1() {
        aoc_11_1();
    }
    #[test]
    fn test_aoc_11_2() {
        aoc_11_2();
    }

    #[derive(PartialEq, Eq, Debug)]
    struct MonkeyAsserts {
        nb: MonkeyNb,
        worry_lvs: Vec<WorryLevel>,
        test_fn: WorryLevel,
        div: WorryLevel,
        on_succ: MonkeyNb,
        on_fail: MonkeyNb,
    }

    impl MonkeyAsserts {
        fn from(m: Monkey) -> MonkeyAsserts {
            MonkeyAsserts {
                nb: m.nb,
                worry_lvs: m.worry_lvs.into_iter().collect(),
                test_fn: (m.update_fn)(2),
                div: m.divide_test,
                on_succ: m.on_succ,
                on_fail: m.on_fail,
            }
        }

        fn new(
            nb: MonkeyNb,
            worry_lvs: Vec<WorryLevel>,
            test_fn: WorryLevel,
            div: WorryLevel,
            on_succ: MonkeyNb,
            on_fail: MonkeyNb,
        ) -> MonkeyAsserts {
            MonkeyAsserts {
                nb,
                worry_lvs,
                test_fn,
                div,
                on_succ,
                on_fail,
            }
        }
    }
}

use core::num;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    usize, vec,
};

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());
    let sol2 = solve2(lines);

    dbg!(sol1);
    dbg!(sol2);
}

fn solve1(lines: Vec<String>) -> usize {
    let mut monkeygroup = MonkeyGroup::from_string_input(lines);

    for idx in 0..20 {
        monkeygroup.round();

        println!("Round {}", idx);
        for monkey in &monkeygroup.monkeys {
            println!("  Monkey {}: {:?}", monkey.id, monkey.items);
        }
    }

    println!("Inspections:");
    monkeygroup.monkeys.sort_by_key(|x| x.inspections);

    let highest_two_monkeys = &monkeygroup.monkeys[monkeygroup.monkeys.len() - 2..];

    for monkey in highest_two_monkeys {
        println!("  Monkey {}: {}", monkey.id, monkey.inspections);
    }

    // dbg!(monkeygroup);

    highest_two_monkeys.get(0).unwrap().inspections
        * highest_two_monkeys.get(1).unwrap().inspections
}

fn solve2(lines: Vec<String>) -> usize {
    let mut monkeygroup = MonkeyGroup::from_string_input(lines);

    for idx in 0..10000 {
        monkeygroup.round2();

        println!("Round {}", idx);
        for monkey in &monkeygroup.monkeys {
            println!("  Monkey {}: {:?}", monkey.id, monkey.items);
        }
    }

    println!("Inspections:");
    monkeygroup.monkeys.sort_by_key(|x| x.inspections);

    let highest_two_monkeys = &monkeygroup.monkeys[monkeygroup.monkeys.len() - 2..];

    for monkey in highest_two_monkeys {
        println!("  Monkey {}: {}", monkey.id, monkey.inspections);
    }

    // dbg!(monkeygroup);

    highest_two_monkeys.get(0).unwrap().inspections
        * highest_two_monkeys.get(1).unwrap().inspections
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    inspections: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    // A function which returns the number of the monkey to which it throws it
    test: Box<dyn Fn(usize) -> usize>,
    divisible_by: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("id", &self.id)
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    fn from_string(lines: Vec<String>) -> Option<Self> {
        let monkey_id: usize = lines.get(0)?[7..]
            .trim_end_matches(":")
            .to_string()
            .parse()
            .ok()?;

        let mut starting_items: Vec<usize> = lines.get(1)?[18..]
            .to_string()
            .split(",")
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        let items: Vec<String> = lines
            .get(2)?
            .to_string()
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect();

        let (operator, number): (&str, Option<usize>) = (items.get(4)?, items.get(5)?.parse().ok());

        let operation: Box<dyn Fn(usize) -> usize> = match (operator, number) {
            ("+", Some(number)) => Box::new(move |old| old + number),
            ("-", Some(number)) => Box::new(move |old| old - number),
            ("*", Some(number)) => Box::new(move |old| old * number),
            ("/", Some(number)) => Box::new(move |old| old / number),
            ("+", None) => Box::new(move |old| old + old),
            ("-", None) => Box::new(move |old| old - old),
            ("*", None) => Box::new(move |old| old * old),
            ("/", None) => Box::new(move |old| old / old),
            _ => return None,
        };

        let items: Vec<String> = lines
            .get(3)?
            .to_string()
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect();

        let divisible_by: usize = items.get(3)?.parse().ok()?;

        let if_true_monkey: usize = lines
            .get(4)?
            .to_string()
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(5)?
            .parse()
            .ok()?;

        let if_false_monkey: usize = lines
            .get(5)?
            .to_string()
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(5)?
            .parse()
            .ok()?;

        let test = Box::new(move |val| {
            if val % divisible_by == 0 {
                if_true_monkey
            } else {
                if_false_monkey
            }
        });

        Some(Self {
            id: monkey_id,
            items: starting_items.into(),
            inspections: 0,
            operation,
            test,
            divisible_by,
        })
    }
}

#[derive(Debug)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
}

impl MonkeyGroup {
    fn new() -> Self {
        Self { monkeys: vec![] }
    }

    fn from_string_input(lines: Vec<String>) -> Self {
        let monkeys: Vec<Monkey> = lines
            .split(|item| item.len() == 0)
            .filter(|x| x.len() > 0)
            .filter_map(|x| Monkey::from_string(x.to_vec()))
            .collect();

        Self { monkeys }
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            if let Some(monkey) = self.monkeys.get_mut(idx) {
                let items = monkey.items.clone();

                let mut movings: Vec<(usize, usize)> = Vec::new();

                monkey.items = Vec::new();
                for worry_level in items {
                    // inspection
                    monkey.inspections += 1;

                    let new_worry_level = (monkey.operation)(worry_level);

                    let new_worry_level = new_worry_level / 3;

                    let target_monkey = (monkey.test)(new_worry_level);

                    movings.push((target_monkey, new_worry_level));
                }

                for (target_monkey, worry_level) in movings {
                    self.monkeys[target_monkey].items.push(worry_level);
                }
            }
        }
    }

    fn round2(&mut self) {
        let magic_trick_number: usize = self.monkeys.iter().map(|x| x.divisible_by).product();

        for idx in 0..self.monkeys.len() {
            if let Some(monkey) = self.monkeys.get_mut(idx) {
                let items = monkey.items.clone();

                let mut movings: Vec<(usize, usize)> = Vec::new();

                monkey.items = Vec::new();
                for worry_level in items {
                    // inspection
                    monkey.inspections += 1;

                    let new_worry_level = (monkey.operation)(worry_level);

                    let new_worry_level = new_worry_level % magic_trick_number;

                    let target_monkey = (monkey.test)(new_worry_level);

                    movings.push((target_monkey, new_worry_level));
                }

                for (target_monkey, worry_level) in movings {
                    self.monkeys[target_monkey].items.push(worry_level);
                }
            }
        }
    }
}

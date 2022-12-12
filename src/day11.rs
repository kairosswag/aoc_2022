use std::collections::{hash_set, HashSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Clone)]
struct Monkey {
    items_held: VecDeque<u64>,
    operation: Operation,
    test: Test,
    true_condition: Condition,
    false_condition: Condition,
    handled_items: u64,
}

impl Monkey {
    fn handle_next_item(&mut self) -> Option<(u64, usize)> {
        let worry = self.items_held.pop_front()?;
        self.handled_items += 1;

        let worry = self.operation.handle(worry) / 3;
        let monkey = if self.test.test(worry) {
            self.true_condition.get_monkey()
        } else {
            self.false_condition.get_monkey()
        };

        Some((worry, monkey))
    }

    
    fn handle_next_item_worryingly(&mut self, global_modulo: u64) -> Option<(u64, usize)> {
        let worry = self.items_held.pop_front()?;
        self.handled_items += 1;

        let worry = self.operation.handle(worry) % global_modulo;
        let monkey = if self.test.test(worry) {
            self.true_condition.get_monkey()
        } else {
            self.false_condition.get_monkey()
        };

        Some((worry, monkey))
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Operation {
    #[display("  Operation: new = old * {0}")]
    Mul(u64),
    #[display("  Operation: new = old + {0}")]
    Plus(u64),
    #[display("  Operation: new = old * old")]
    MulSelf,
}

impl Operation {
    fn handle(&self, worry: u64) -> u64 {
        match self {
            Operation::Mul(val) => worry * val,
            Operation::Plus(val) => worry + val,
            Operation::MulSelf => worry * worry,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("  Test: divisible by {0}")]
struct Test(u64);

impl Test {
    fn test(&self, worry: u64) -> bool {
        let Test(test_val) = self;
        worry % test_val == 0
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("    If {0}: throw to monkey {1}")]
struct Condition(bool, usize);

impl Condition {
    fn get_monkey(&self) -> usize {
        let Condition(_, monkey) = self;
        *monkey
    }
}

pub fn run() {
    let f = File::open("input/day11").expect("could not open file");
    let reader = BufReader::new(f);

    let mut monkeys = reader
        .lines()
        .chunks(7)
        .into_iter()
        .map(|ch| parse_monkey(ch))
        .collect::<Vec<Monkey>>();

    let mut worrying_monkeys = monkeys.clone();

    for _round in 0..20 {
        for monkey in 0..monkeys.len() {
            while let Some((worry, throwee)) = monkeys[monkey].handle_next_item() {
                monkeys[throwee].items_held.push_back(worry);
            }
        }
    }
    let mut sorted = monkeys.iter().map(|monkey| monkey.handled_items).sorted().rev();
    println!("Day 11 Solution Part 1: {}", sorted.next().unwrap() * sorted.next().unwrap());

    let global_modulo = worrying_monkeys.iter().map(|monkey| monkey.test.0).fold(1, |a, b| a*b);
    for _round in 0..10000 {
        for monkey in 0..worrying_monkeys.len() {
            while let Some((worry, throwee)) = worrying_monkeys[monkey].handle_next_item_worryingly(global_modulo) {
                worrying_monkeys[throwee].items_held.push_back(worry);
            }
        }
    }
    let mut sorted = worrying_monkeys.iter().map(|monkey| monkey.handled_items).sorted().rev();
    println!("Day 11 Solution Part 2: {}", sorted.next().unwrap() * sorted.next().unwrap());
}

fn parse_monkey(mut ch: itertools::Chunk<std::io::Lines<BufReader<File>>>) -> Monkey {
    let _id = ch.next().unwrap().unwrap();
    let items_held = parse_starting_items(ch.next().unwrap().unwrap());
    let operation = ch.next().unwrap().unwrap().parse().unwrap();
    let test = ch.next().unwrap().unwrap().parse().unwrap();
    let true_condition = ch.next().unwrap().unwrap().parse().unwrap();
    let false_condition = ch.next().unwrap().unwrap().parse().unwrap();
    Monkey {
        items_held,
        operation,
        test,
        true_condition,
        false_condition,
        handled_items: 0,
    }
}

fn parse_starting_items(input: String) -> VecDeque<u64> {
    input[18..]
        .split(", ")
        .map(|val| val.parse().unwrap())
        .collect()
}

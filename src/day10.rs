use std::collections::{hash_set, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Operation {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Add(i32),
}

pub fn run() {
    part1();
}

pub fn part1() {
    let f = File::open("input/day10").expect("could not open file");
    let reader = BufReader::new(f);

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;

    for op in reader.lines().map(|l| {
        l.unwrap()
            .parse::<Operation>()
            .expect("could not parse as Operation")
    }) {
        match op {
            Operation::Noop => increase_cycle(&mut cycle, &mut signal_strength, x),
            Operation::Add(val) => {
                increase_cycle(&mut cycle, &mut signal_strength, x);
                increase_cycle(&mut cycle, &mut signal_strength, x);
                x += val;
            }
        }
    }

    println!("Day 10 Solution Part 1: {}", signal_strength);
}

fn increase_cycle(cycle: &mut i32, signal_strength: &mut i32, x: i32) {
    *cycle += 1;
    if *cycle == 20 || *cycle % 40 == 20 {
        *signal_strength += *cycle * x;
    }
}

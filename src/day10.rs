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
    let f = File::open("input/day10").expect("could not open file");
    let reader = BufReader::new(f);

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;
    let mut crt = Vec::with_capacity(240);

    for op in reader.lines().map(|l| {
        l.unwrap()
            .parse::<Operation>()
            .expect("could not parse as Operation")
    }) {
        match op {
            Operation::Noop => increase_cycle(&mut cycle, &mut signal_strength, x, &mut crt),
            Operation::Add(val) => {
                increase_cycle(&mut cycle, &mut signal_strength, x, &mut crt);
                increase_cycle(&mut cycle, &mut signal_strength, x, &mut crt);
                x += val;
            }
        }
    }

    println!(
        "Day 10 Solution Part 1: {} in {cycle} steps",
        signal_strength
    );
    print_crt(crt);
}

fn increase_cycle(cycle: &mut i32, signal_strength: &mut i32, x: i32, crt: &mut Vec<&str>) {
    *cycle += 1;
    if *cycle == 20 || *cycle % 40 == 20 {
        *signal_strength += *cycle * x;
    }

    let row_pos = (*cycle-1)  % 40;
    if row_pos == x - 1 || row_pos == x || row_pos == x + 1 {
        crt.push("⬜");
    } else {
        crt.push("⬛");
    }
}

fn print_crt(crt: Vec<&str>) {
    print!("\nCRT:\n");
    for i in 1..=crt.len() {
        print!("{}", crt[i - 1]);
        if i % 40 == 0 {
            print!("\n");
        }
    }
    print!("\n");
}

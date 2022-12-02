use std::fs;
use std::str::FromStr;
use std::time::Instant;

pub fn run() {
    let res = fs::read_to_string("input/day01")
        .expect("Could not open file.")
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|val| u32::from_str(val).unwrap_or(0))
                .sum::<u32>()
        })
        .fold([0, 0, 0], |[a, b, c], val| {
            let mut res = [a, b, c, val];
            res.sort();
            [res[1], res[2], res[3]]
        });

    println!("Day 01 Solution Part 1: {}", res[2]);
    println!("Day 01 Solution Part 2: {}", res[0] + res[1] + res[2]);
}
use std::fs;
use std::str::FromStr;

fn main() {
    day01();
}

pub fn day01() {
    let res = fs::read_to_string("input/day01")
        .expect("Could not open file.")
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|val| u32::from_str(val).unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("Day 01 Solution Part 1: {}", res);
}

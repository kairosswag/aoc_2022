use std::fs;
use std::str::FromStr;

fn main() {
    day01();
}

pub fn day01() {
    let res_p1 = fs::read_to_string("input/day01")
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

    println!("Day 01 Solution Part 1: {}", res_p1);

    let res_p2 = fs::read_to_string("input/day01")
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
        })
        .iter()
        .sum::<u32>();

    println!("Day 01 Solution Part 2: {}", res_p2);
}

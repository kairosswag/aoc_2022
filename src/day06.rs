use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let filestr = fs::read_to_string("input/day06")
        .expect("Could not open file.");
    let bytes = filestr.as_bytes();

    let mut res = 0;
    for i in 14.. {
        let slice = &bytes[i-14..i];
        if slice.iter().unique().count() == 14 {
            res = i;
            break;
        }
    };

    println!("Day 06 Solution Part 2: {}", res);
}

fn part1() {
    let res = fs::read_to_string("input/day06")
        .expect("Could not open file.")
        .char_indices()
        .tuple_windows()
        .flat_map(is_packet_marker)
        .next()
        .expect("no result");

    println!("Day 06 Solution Part 1: {}", res);
}

fn is_packet_marker(
    ((_, a), (_, b), (_, c), (pos, d)): (
        (usize, char),
        (usize, char),
        (usize, char),
        (usize, char),
    ),
) -> Option<usize> {
    if a == b || a == c || a == d || b == c || b == d || c == d {
        None
    } else {
        Some(pos + 1)
    }
}


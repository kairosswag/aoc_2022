use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{first_assignment_start}-{first_assignment_end},{second_assignment_start}-{second_assignment_end}")]
struct ElfPair {
    first_assignment_start: u32,
    first_assignment_end: u32,
    second_assignment_start: u32,
    second_assignment_end: u32,
}

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let f = File::open("input/day04").expect("could not open file");
    let reader = BufReader::new(f);

    let res = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|r| !r.is_empty())
        .map(|r| r.parse::<ElfPair>().expect("could not parse elf pair "))
        .filter(|pair| overlaps(pair))
        .count();

    println!("Day 04 Solution Part 2: {}", res);
}


fn part1() {
    let f = File::open("input/day04").expect("could not open file");
    let reader = BufReader::new(f);

    let res = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|r| !r.is_empty())
        .map(|r| r.parse::<ElfPair>().expect("could not parse elf pair "))
        .filter(|pair| is_contained(pair))
        .count();

    println!("Day 04 Solution Part 1: {}", res);
}

fn is_contained(pair: &ElfPair) -> bool {
    let contained = |(fs, fe, ss, se): (u32, u32, u32, u32)| fs <= ss && fe >= se;

    contained((
        pair.first_assignment_start,
        pair.first_assignment_end,
        pair.second_assignment_start,
        pair.second_assignment_end,
    )) || contained((
        pair.second_assignment_start,
        pair.second_assignment_end,
        pair.first_assignment_start,
        pair.first_assignment_end,
    ))
}

fn overlaps(pair: &ElfPair) -> bool {
    pair.first_assignment_start <= pair.second_assignment_end && pair.second_assignment_start <= pair.first_assignment_end
    || pair.second_assignment_start <= pair.first_assignment_end && pair.first_assignment_start <= pair.second_assignment_end
}

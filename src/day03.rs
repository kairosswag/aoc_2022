use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn run() {
    part_1();
    part_2();
}

fn part_2() {
    let f = File::open("input/day03").expect("could not open file");
    let reader = BufReader::new(f);

    let result = reader.lines().chunks(3).into_iter().map(|chunk| {
        chunk
            .map(|s| s.unwrap().chars().collect::<HashSet<char>>())
            .reduce(|mut set1, set2| {
                set1.retain(|val| set2.contains(val));
                set1
            })
    }).map(|set| *set.unwrap().iter().next().unwrap()).map(|c| priority(c as u8)).sum::<u32>();
    println!("Day 03 Solution Part 2: {}", result);
}

fn part_1() {
    let f = File::open("input/day03").expect("could not open file");
    let mut reader = BufReader::new(f);

    // let mut reader = Cursor::new(b"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT");

    let mut buf = Vec::new();
    let mut accum = 0;
    while let Ok(num_bytes) = reader.read_until(b'\n', &mut buf) {
        if num_bytes == 0 {
            break;
        }
        'search: for i in 0..(num_bytes / 2) {
            for j in (num_bytes / 2)..buf.len() {
                if buf[i] == buf[j] {
                    accum += priority(buf[i]);
                    break 'search;
                }
            }
        }
        buf.clear();
    }

    println!("Day 03 Solution Part 1: {}", accum);
}

fn priority(val: u8) -> u32 {
    if val < 'a' as u8 {
        val as u32 - b'A' as u32 + 1 + 26
    } else {
        val as u32 - b'a' as u32 + 1
    }
}

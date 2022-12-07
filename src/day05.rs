use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {amount} from {source} to {target}")]
struct Step {
    amount: usize,
    source: usize,
    target: usize,
}

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let f = File::open("input/day05").expect("could not open file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut stacks = vec![Vec::new();10];
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        if line.starts_with(" 1") {
            continue;
        }

        parse_stack(&line, &mut stacks);
    }

    let steps: Vec<Step> = lines.map(|l| l.unwrap()).map(|l| l.parse::<Step>().expect(&format!("Could not parse step {:?}", l))).collect();

    for step in steps {
        let mut crane_arm = Vec::new();
        for _ in 0..step.amount {
            let elf_crate = stacks[step.source].pop().expect("There should be something here.");
            crane_arm.push(elf_crate);
        }

        while let Some(elf_crate) = crane_arm.pop() {
            stacks[step.target].push(elf_crate);
        }
    }


    let res: String = stacks.iter_mut().skip(1).map(|substack| substack.pop().unwrap()).collect();
    println!("Day 05 Solution Part 2: {}", res);
}

fn part1() {
    let f = File::open("input/day05").expect("could not open file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut stacks = vec![Vec::new();10];
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        if line.starts_with(" 1") {
            continue;
        }

        parse_stack(&line, &mut stacks);
    }

    let steps: Vec<Step> = lines.map(|l| l.unwrap()).map(|l| l.parse::<Step>().expect(&format!("Could not parse step {:?}", l))).collect();

    for step in steps {
        for _ in 0..step.amount {
            let elf_crate = stacks[step.source].pop().expect("There should be something here.");
            stacks[step.target].push(elf_crate);
        }
    }


    let res: String = stacks.iter_mut().skip(1).map(|substack| substack.pop().unwrap()).collect();
    println!("Day 05 Solution Part 1: {}", res);
}


fn parse_stack(line: &str, res: &mut [Vec<char>]) {
    let char_arr: Vec<char> = line.chars().collect();

    for i in 1..=9 {
        if char_arr[1 + (i - 1) * 4] != ' ' {
            res[i].insert(0, char_arr[1 + (i - 1) * 4]);
        }
    }
}

fn _print_stack(stack: &[Vec<char>]) {
    for i in 1..stack.len() {
        println!("{}: {}", i, stack[i].iter().collect::<String>());
    }
}


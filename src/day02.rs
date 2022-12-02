use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{turn_elf} {turn_me}")]
struct Duel {
    turn_elf: ElfTurn,
    turn_me: MyTurn,
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum ElfTurn {
    #[display("A")]
    Rock,
    #[display("B")]
    Paper,
    #[display("C")]
    Scissors,
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum MyTurn {
    #[display("X")]
    TurnX,
    #[display("Y")]
    TurnY,
    #[display("Z")]
    TurnZ,
}

impl Duel {

    pub fn get_count_p2(&self) -> u32 {
        self.get_duel_value_2() + self.get_turn_value_2()
    }

    
    fn get_duel_value_2(&self) -> u32 {
        match self.turn_me {
            MyTurn::TurnX => 0,
            MyTurn::TurnY => 3,
            MyTurn::TurnZ => 6,
        }
    }

    fn get_turn_value_2(&self) -> u32 {
        use ElfTurn::*;
        use MyTurn::*;

        match (&self.turn_elf, &self.turn_me) {
            // I choose rock
            (Rock, TurnY) | (Scissors, TurnZ) | (Paper, TurnX) => 1,
            // I choose paper
            (Paper, TurnY) | (Rock, TurnZ) | (Scissors, TurnX) => 2,
            // Rest: Choose Scissors
            _ => 3,
        }
    }

    pub fn get_count_p1(&self) -> u32 {
        self.get_duel_value() + self.get_turn_value()
    }

    fn get_turn_value(&self) -> u32 {
        match self.turn_me {
            MyTurn::TurnX => 1,
            MyTurn::TurnY => 2,
            MyTurn::TurnZ => 3,
        }
    }

    fn get_duel_value(&self) -> u32 {
        use ElfTurn::*;
        use MyTurn::*;

        match (&self.turn_elf, &self.turn_me) {
            (Rock, TurnX) | (Paper, TurnY) | (Scissors, TurnZ) => 3,
            (Rock, TurnY) | (Paper, TurnZ) | (Scissors, TurnX) => 6,
            (_, _) => 0,
        }
    }
}

pub fn run() {
    let f = File::open("input/day02").expect("could not open file");
    let reader = BufReader::new(f);

    let res: u32 = reader
        .lines()
        .map(|l| {
            l.expect("should be line")
                .parse::<Duel>()
                .expect("could not parse to duel")
        })
        .map(|d| d.get_count_p1())
        .sum();

    let f = File::open("input/day02").expect("could not open file");
    let reader = BufReader::new(f);
    let res2: u32 = reader
        .lines()
        .map(|l| {
            l.expect("should be line")
                .parse::<Duel>()
                .expect("could not parse to duel")
        })
        .map(|d| d.get_count_p2())
        .sum();

    println!("Day 02 Solution Part 1: {}", res);
    println!("Day 02 Solution Part 2: {}", res2);
}

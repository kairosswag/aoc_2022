use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    str::FromStr,
    vec,
};

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum ParsedLine {
    #[display("$ {0}")]
    Command(Command),
    #[display("{0}")]
    Result(Result),
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Command {
    #[display("ls")]
    LS,
    #[display("cd {0}")]
    CD(String),
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Result {
    #[display("dir {0}")]
    Dir(String),
    #[display("{0} {1}")]
    File(u32, String),
}

pub fn run() {
    part1();
}

fn part1() {
    let f = File::open("input/day07").expect("could not open file");
    let reader = BufReader::new(f);

    let mut curr_path = Vec::new();
    let mut curr_dir = 0;
    let mut node_map: HashMap<String, u32> = HashMap::new();
    node_map.insert("/".to_string(), 0);

    let mut curr_idx = 1;

    let mut parents: HashMap<u32, u32> = HashMap::new();
    let mut children: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut files: HashMap<u32, u32> = HashMap::new();

    let mut parsed = reader.lines().map(|line| {
        line.unwrap()
            .parse::<ParsedLine>()
            .expect("Could not parse line.")
    });
    while let Some(parsed) = parsed.next() {
        match parsed {
            ParsedLine::Command(Command::LS) => handle_ls(),
            ParsedLine::Command(Command::CD(dir_name)) => handle_cd(
                &dir_name,
                &mut curr_path,
                &mut node_map,
                &mut parents,
                &mut curr_dir,
            ),
            ParsedLine::Result(Result::Dir(dir_name)) => handle_dir(
                &dir_name,
                &mut curr_path,
                &mut node_map,
                &mut parents,
                &mut children,
                curr_dir,
                &mut curr_idx,
            ),
            ParsedLine::Result(Result::File(size, file_name)) => {
                handle_file(size, &file_name, curr_dir, &mut files)
            }
        }
    }

    let mut sizes: HashMap<u32, u32> = HashMap::new();

    parse_recursive(0, &mut sizes, &children, &files);

    let res: u32 = sizes.values().filter(|&size| *size <= 100000).sum();
    println!("Day 07 Solution Part 1: {}", res);
}

fn parse_recursive(
    curr_idx: u32,
    sizes: &mut HashMap<u32, u32>,
    children: &HashMap<u32, Vec<u32>>,
    files: &HashMap<u32, u32>,
) -> u32 {
    let mut accum = 0;
    if let Some(curr_children) = children.get(&curr_idx) {
        for child in curr_children {
            accum += parse_recursive(*child, sizes, children, files)
        }
    }

    if let Some(filesize) = files.get(&curr_idx) {
        accum += *filesize;
    }

    sizes.insert(curr_idx, accum);

    accum
}

fn handle_ls() {
    // do nothing
}

fn handle_cd(
    dir_name: &str,
    curr_path: &mut Vec<String>,
    node_map: &mut HashMap<String, u32>,
    parents: &mut HashMap<u32, u32>,
    curr_dir: &mut u32,
) {
    if dir_name == ".." {
        *curr_dir = *parents.get(curr_dir).expect("no parent for some reason");
        curr_path.pop();
    } else {
        let node = &(curr_path.iter().map(|s| s.clone()).collect::<String>() + &dir_name.to_owned());
        let next_dir = *node_map
            .get(node)
            .expect(&format!("Node {node} not yet in node map"));
        *curr_dir = next_dir;
        curr_path.push(dir_name.to_owned());
    }
}

fn handle_dir(
    dir_name: &str,
    curr_path: &mut Vec<String>,
    node_map: &mut HashMap<String, u32>,
    parents: &mut HashMap<u32, u32>,
    children: &mut HashMap<u32, Vec<u32>>,
    curr_dir: u32,
    curr_idx: &mut u32,
) {
    let idx = if let Some(dir_idx) = node_map
        .get(&(curr_path.iter().map(|s| s.clone()).collect::<String>() + &dir_name.to_owned()))
    {
        *dir_idx
    } else {
        let idx = *curr_idx;
        node_map.insert(
            curr_path.iter().map(|s| s.clone()).collect::<String>() + &dir_name.to_owned(),
            idx,
        );
        *curr_idx += 1;
        idx
    };

    parents.insert(idx, curr_dir);

    children
        .entry(curr_dir)
        .and_modify(|val| val.push(idx))
        .or_insert(vec![idx]);
}

fn handle_file(size: u32, _file_name: &str, curr_dir: u32, files: &mut HashMap<u32, u32>) {
    files
        .entry(curr_dir)
        .and_modify(|val| *val += size)
        .or_insert(size);
}

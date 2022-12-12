use std::collections::{hash_set, HashSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;
use parse_display::{Display, FromStr};

pub fn run() {
    let f = File::open("input/day12").expect("could not open file");
    let reader = BufReader::new(f);

    let mut grid = Vec::new();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut a_poses = HashSet::new();
    for (line, row) in reader.lines().zip(0..) {
        let mut row_vec = Vec::new();
        for (val, column) in line.unwrap().bytes().zip(0..) {
            if 'a' as u8 == val {
                a_poses.insert((row, column));
            }
            if 'S' as u8 == val {
                row_vec.push('a' as u8);
                start_pos = (row, column);
            } else if 'E' as u8 == val {
                end_pos = (row, column);
                row_vec.push('z' as u8);
            } else {
                row_vec.push(val);
            }
        }
        grid.push(row_vec);
    }

    
    let mut visited = HashSet::new();
    visited.insert(start_pos);
    
    let mut next_gen = HashSet::new();
    
    for neighbor in get_neighbors(start_pos, &grid) {
        next_gen.insert(neighbor);
    }
    
    let res = bfs(visited, next_gen, end_pos, &grid);
    println!("Day 12 Solution Part 1: {}", res);

    let res2 = bfs(HashSet::new(), a_poses, end_pos, &grid);
    println!("Day 12 Solution Part 2: {}", res2 - 1);
}

fn bfs(mut visited: HashSet<(i32, i32)>, mut next_gen: HashSet<(i32, i32)>, end_pos: (i32, i32), grid: &Vec<Vec<u8>>) -> i32 {
    
    for i in 1.. {
        // if i % 100 == 0 {
        //     println!("at step {i}, already visited {} nodes", visited.len());
        // }
        if next_gen.is_empty() {
            panic!("should never happen");
        }
        let mut nexter_gen = HashSet::new();
        for node in next_gen {
            if !visited.contains(&node) {
                if node == end_pos {
                    return i;
                }
                visited.insert(node);

                for neighbor in get_neighbors(node, grid) {
                    nexter_gen.insert(neighbor);
                }
            }
        }

        next_gen = nexter_gen;
    }

    5
}

fn get_neighbors(curr_pos: (i32, i32), grid: &Vec<Vec<u8>>) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let (x, y) = curr_pos;
    let curr_val = get_safe((x, y), grid).expect(&format!(
        "I did expecto to be able to find curr_pos {:?}",
        curr_pos
    ));

    let mut maybe_push = |(x, y): (i32, i32)| {
        if let Some(val) = get_safe((x, y), grid) {
            if val as i32 - curr_val as i32 <= 1 {
                res.push((x, y));
            }
        }
    };

    maybe_push((x - 1, y));
    maybe_push((x + 1, y));
    maybe_push((x, y - 1));
    maybe_push((x, y + 1));

    res
}

fn get_safe<T: Copy>(pos: (i32, i32), grid: &Vec<Vec<T>>) -> Option<T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    Some(*grid.get(pos.0 as usize)?.get(pos.1 as usize)?)
}

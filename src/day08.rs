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
    let f = File::open("input/day08").expect("could not open file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    const side_len: usize = 99;
    let mut res_arr = [0u8; side_len * side_len];
    let mut curr_idx = 0;

    while let Some(Ok(line)) = lines.next() {
        for byte in line.bytes() {
            res_arr[curr_idx] = byte - '0' as u8;
            curr_idx += 1;
        }
    }

    
    // let score = calc_scenic_score(&res_arr, 2, 1, side_len);

    let mut max_score = 0;
    for row in 0..side_len {
        for column in 0..side_len {
            let score = calc_scenic_score(&res_arr, row, column, side_len);
            max_score = u32::max(max_score, score);
        }
    }

    println!("Day 08 Solution Part 2: {}", max_score);
}

fn calc_scenic_score(res_arr: &[u8], row: usize, column: usize, side_len: usize) -> u32 {
    let mut score_w: u32 = 0;
    let mut score_e: u32 = 0;
    let mut score_n: u32 = 0;
    let mut score_s: u32 = 0;
    let orig_height = res_arr[row + column * side_len];

    {
        let mut curr_row = row;
        while curr_row > 0 && res_arr[(curr_row - 1) + column * side_len] < orig_height {
            score_e += 1;
            curr_row -= 1;
        }
        if curr_row > 0 {
            score_e += 1;
        }
    }

    {
        let mut curr_row = row;
        while curr_row < (side_len - 1) && res_arr[(curr_row + 1) + column * side_len] < orig_height
        {
            score_w += 1;
            curr_row += 1;
        }
        if curr_row < (side_len - 1) {
            score_w += 1;
        }
    }

    {
        let mut curr_col = column;
        while curr_col > 0 && res_arr[row + (curr_col - 1) * side_len] < orig_height {
            score_n += 1;
            curr_col -= 1;
        }
        if curr_col > 0 {
            score_n += 1;
        }
    }

    {
        let mut curr_col = column;
        while curr_col < (side_len - 1) && res_arr[row + (curr_col + 1) * side_len] < orig_height {
            score_s += 1;
            curr_col += 1;
        }
        if curr_col < (side_len - 1) {
            score_s += 1;
        }
    }
    // println!("{score_e} * {score_w} * {score_n} * {score_s}");
    score_e * score_w * score_n * score_s
}

fn part1() {
    let f = File::open("input/day08").expect("could not open file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    const side_len: usize = 99;
    let mut res_arr = [0u8; side_len * side_len];
    let mut curr_idx = 0;

    while let Some(Ok(line)) = lines.next() {
        for byte in line.bytes() {
            res_arr[curr_idx] = byte - '0' as u8;
            curr_idx += 1;
        }
    }

    let mut visible_horizont = [1u8; side_len * side_len];
    for row in 0..side_len {
        let base = row * side_len;
        let mut curr_highest = res_arr[base];
        for column in 1..side_len {
            let val = res_arr[base + column];
            if val <= curr_highest {
                visible_horizont[base + column] = 0;
            } else {
                curr_highest = val;
            }
        }
    }

    let mut visible_horizont_rev = [1u8; side_len * side_len];
    for row in 0..side_len {
        let base = row * side_len;
        let mut curr_highest = res_arr[base + (side_len - 1)];
        for column in (0..(side_len - 1)).rev() {
            let val = res_arr[base + column];
            if val <= curr_highest {
                visible_horizont_rev[base + column] = 0;
            } else {
                curr_highest = val;
            }
        }
    }

    let mut visible_vertic = [1u8; side_len * side_len];
    for column in 0..side_len {
        let base = column;
        let mut curr_highest = res_arr[base];
        for row in 1..side_len {
            let val = res_arr[base + row * side_len];
            if val <= curr_highest {
                visible_vertic[base + row * side_len] = 0;
            } else {
                curr_highest = val;
            }
        }
    }

    let mut visible_vertic_rev = [1u8; side_len * side_len];
    for column in 0..side_len {
        let base = column;
        let mut curr_highest = res_arr[base + side_len * (side_len - 1)];
        for row in (0..(side_len - 1)).rev() {
            let val = res_arr[base + row * side_len];
            if val <= curr_highest {
                visible_vertic_rev[base + row * side_len] = 0;
            } else {
                curr_highest = val;
            }
        }
    }

    let mut counter = 0;
    for i in 0..(side_len * side_len) {
        if visible_horizont[i] == 0
            && visible_horizont_rev[i] == 0
            && visible_vertic[i] == 0
            && visible_vertic_rev[i] == 0
        {
            // do nothing
        } else {
            counter += 1;
        }
    }

    println!("Day 08 Solution Part 1: {}", counter);
}

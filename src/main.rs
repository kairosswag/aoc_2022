use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let now = Instant::now();
    day08::run();
    println!("Took {} µs", now.elapsed().as_micros());
}


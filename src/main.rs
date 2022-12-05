use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let now = Instant::now();
    day05::run();
    println!("Took {} µs", now.elapsed().as_micros());
}


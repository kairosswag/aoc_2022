use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let now = Instant::now();
    day04::run();
    println!("Took {} µs", now.elapsed().as_micros());
}


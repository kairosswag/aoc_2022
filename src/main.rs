use std::time::Instant;

mod day01;
mod day02;
mod day03;

fn main() {
    let now = Instant::now();
    day03::run();
    println!("Took {} µs", now.elapsed().as_micros());
}


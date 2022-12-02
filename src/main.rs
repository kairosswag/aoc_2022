use std::time::Instant;

mod day01;
mod day02;

fn main() {
    let now = Instant::now();
    day02::run();
    println!("Took {} µs", now.elapsed().as_micros());
}


use std::collections::{hash_set, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Direction {
    #[display("U {0}")]
    Up(u32),
    #[display("D {0}")]
    Down(u32),
    #[display("L {0}")]
    Left(u32),
    #[display("R {0}")]
    Right(u32),
}

impl Direction {
    fn get_steps(&self) -> u32 {
        use Direction::*;
        match &self {
            Up(val) | Down(val) | Left(val) | Right(val) => *val,
        }
    }
}

struct State {
    h_pos: (i32, i32),
    t_pos: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

struct SnekState{
    snek: [(i32, i32); 10],
    visited: HashSet<(i32, i32)>,
}

impl SnekState{
    fn new() -> SnekState {
        let snek: [(i32, i32); 10] = [(0, 0); 10];
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        SnekState {
            snek,
            visited,
        }
    }
}

impl State {
    fn new() -> State {
        let h_pos = (0, 0);
        let t_pos = (0, 0);
        let mut visited = HashSet::new();
        visited.insert(h_pos);

        State {
            h_pos,
            t_pos,
            visited,
        }
    }

    fn print_state(&self) {
        let ((min_x, max_x), (min_y, max_y)) = self
            .visited
            .iter()
            .chain([self.h_pos, self.t_pos].iter())
            .fold(
                ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                |((min_x, max_x), (min_y, max_y)), (x, y)| {
                    (
                        (i32::min(min_x, *x), i32::max(max_x, *x)),
                        (i32::min(min_y, *y), i32::max(max_y, *y)),
                    )
                },
            );

        print!("\n");
        for col in (min_y..=max_y).rev() {
            for row in min_x..=max_x {
                if (row, col) == self.h_pos {
                    print!("H");
                } else if (row, col) == self.t_pos {
                    print!("T");
                } else if (row, col) == (0, 0) {
                    print!("s");
                } else if self.visited.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

pub fn run() {
    part1();
    part2();
}

pub fn part2() {
    let f = File::open("input/day09").expect("could not open file");
    let reader = BufReader::new(f);
    let res_state = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<Direction>()
                .expect("could not parse as direction")
        })
        .fold(SnekState::new(), perform_move2);

    println!("Day 09 Solution Part 2: {}", res_state.visited.len());
    // println!("Day 01 Solution Part 2: {}", res[0] + res[1] + res[2]);
}

fn perform_move2(mut state: SnekState, dir: Direction) -> SnekState {
    for _ in 0..dir.get_steps() {
        move_snek_head(&mut state, &dir);

        for i in 1..state.snek.len() {
            let res = move_snek_tail(state.snek[i-1], state.snek[i]);
            state.snek[i] = res;
            if i == state.snek.len() - 1 {
                state.visited.insert(res);
            }
        }
    }
    // state.print_state();
    state
}

fn move_snek_head(state: &mut SnekState, dir: &Direction) {
    use Direction::*;
    let (a, b) = state.snek[0];
    let new_pos = match dir {
        Up(_) => (a, b + 1),
        Down(_) => (a, b - 1),
        Left(_) => (a - 1, b),
        Right(_) => (a + 1, b),
    };

    state.snek[0] = new_pos;
}


fn move_snek_tail((prev_x, prev_y): (i32, i32), (foll_x, foll_y): (i32, i32)) -> (i32, i32) {
    let delta_horiz = prev_x - foll_x;
    let delta_vert = prev_y - foll_y;
    let (o_x, o_y) = (foll_x, foll_y);

    match (delta_horiz, delta_vert) {
        (-1..=1, -1..=1) => (o_x, o_y),
        (x, y) => (o_x + x.signum(), o_y + y.signum()),
    }

}


pub fn part1() {
    let f = File::open("input/day09").expect("could not open file");
    let reader = BufReader::new(f);
    let res_state = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<Direction>()
                .expect("could not parse as direction")
        })
        .fold(State::new(), perform_move);

    println!("Day 09 Solution Part 1: {}", res_state.visited.len());
    // println!("Day 01 Solution Part 2: {}", res[0] + res[1] + res[2]);
}

fn perform_move(mut state: State, dir: Direction) -> State {
    for _ in 0..dir.get_steps() {
        move_head(&mut state, &dir);
        move_tail(&mut state);
        // state.print_state();
    }
    state
}

fn move_tail(state: &mut State) {
    let delta_horiz = state.h_pos.0 - state.t_pos.0;
    let delta_vert = state.h_pos.1 - state.t_pos.1;
    let (o_x, o_y) = state.t_pos;

    let new_pos = match (delta_horiz, delta_vert) {
        (-1..=1, -1..=1) => (o_x, o_y),
        (x, y) => (o_x + x.signum(), o_y + y.signum()),
    };

    state.t_pos = new_pos;
    state.visited.insert(new_pos);
    // println!("old_pos: ({} {})", o_x, o_y);
    // println!("delta: ({} {})", delta_horiz, delta_vert);
    // println!("new_pos: ({} {})", new_pos.0, new_pos.1);
}

fn move_head(state: &mut State, dir: &Direction) {
    use Direction::*;
    let (a, b) = state.h_pos;
    let new_pos = match dir {
        Up(_) => (a, b + 1),
        Down(_) => (a, b - 1),
        Left(_) => (a - 1, b),
        Right(_) => (a + 1, b),
    };

    state.h_pos = new_pos;
}



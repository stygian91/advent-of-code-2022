#[macro_use]
extern crate lazy_static;

// use part1::part1;

use std::fs::read_to_string;

use part2::part2;

// mod board;
// mod part1;
mod part2;

fn main() {
    // part1();
    let input = read_to_string("./data/demo.txt").unwrap();
    part2(&input);
}

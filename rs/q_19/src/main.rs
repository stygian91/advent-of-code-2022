use std::fs::read_to_string;

use blueprint::parse_blueprints;
use part1::part1;
use part2::part2;

mod action;
mod blueprint;
mod part1;
mod part2;
mod state;

fn main() {
    let input = read_to_string("./data/demo.txt").unwrap();
    let blueprints = parse_blueprints(&input);
    part1(&blueprints);
    // part2(&blueprints);
}

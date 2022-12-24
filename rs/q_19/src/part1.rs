use crate::blueprint::{run_blueprint, Blueprint};

pub fn part1(blueprints: &[Blueprint]) {
    let mut qualities = 0u64;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let geodes = run_blueprint(24, blueprint);
        qualities += geodes * (i as u64 + 1);
    }

    println!("Part 1: {}", qualities);
}

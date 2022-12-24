use crate::blueprint::{run_blueprint, Blueprint};
use rayon::prelude::*;

pub fn part1(blueprints: &[Blueprint]) {
    let qualities: u64 = blueprints
        .par_iter()
        .map(|blueprint| {
            let geodes = run_blueprint(24, blueprint);
            geodes * blueprint.id as u64
        })
        .sum();

    println!("Part 1: {}", qualities);
}

use crate::blueprint::{run_blueprint, Blueprint};
use rayon::prelude::*;

pub fn part2(blueprints: &[Blueprint]) {
    let prod: u64 = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| run_blueprint(32, blueprint))
        .product();

    println!("Part 2: {}", prod);
}

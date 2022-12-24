use crate::blueprint::{run_blueprint, Blueprint};

pub fn part2(blueprints: &[Blueprint]) {
    let mut prod = 1;

    for blueprint in blueprints.iter().take(3) {
        let geodes = run_blueprint(32, blueprint);
        prod *= geodes;
    }

    println!("Part 2: {}", prod);
}

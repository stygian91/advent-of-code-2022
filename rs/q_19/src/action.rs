use crate::{
    blueprint::Blueprint,
    state::{Resources, State},
};
use Action::*;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
    Wait,
}

pub fn get_possible_actions(blueprint: &Blueprint, resources: &Resources) -> Vec<Action> {
    let mut res = Vec::with_capacity(5);
    res.push(Wait);

    if resources.ore >= blueprint.ore_robot {
        res.push(BuildOre);
    }

    if resources.ore >= blueprint.clay_robot {
        res.push(BuildClay);
    }

    if resources.ore >= blueprint.obsidian_robot.0 && resources.clay >= blueprint.obsidian_robot.1 {
        res.push(BuildObsidian);
    }

    if resources.ore >= blueprint.geode_robot.0 && resources.obsidian >= blueprint.geode_robot.1 {
        res.push(BuildGeode);
    }

    res
}

#[derive(Debug)]
pub struct ActionNode {
    pub state: State,
    pub action: Action,
}

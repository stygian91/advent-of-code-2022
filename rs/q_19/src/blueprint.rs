use regex::Regex;

use crate::{
    action::{get_possible_actions, Action, ActionNode},
    state::State,
};

#[derive(Debug)]
pub struct Blueprint {
    pub id: usize,
    pub ore_robot: u64,
    pub clay_robot: u64,
    pub obsidian_robot: (u64, u64),
    pub geode_robot: (u64, u64),
}

impl Blueprint {
    pub fn get_max_ore_cost(&self) -> u64 {
        [
            self.ore_robot,
            self.clay_robot,
            self.obsidian_robot.0,
            self.geode_robot.0,
        ]
        .iter()
        .max()
        .unwrap()
        .to_owned()
    }
}

pub fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let reg = Regex::new(r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();

    input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let captures = reg.captures(line).unwrap();
            let get_i = |i: usize| captures.get(i).unwrap().as_str().parse::<u64>().unwrap();
            Blueprint {
                id: line_num + 1,
                ore_robot: get_i(1),
                clay_robot: get_i(2),
                obsidian_robot: (get_i(3), get_i(4)),
                geode_robot: (get_i(5), get_i(6)),
            }
        })
        .collect()
}

pub fn run_blueprint(limit: u64, blueprint: &Blueprint) -> u64 {
    let root = ActionNode {
        state: State::new(),
        action: Action::Wait,
    };
    let mut max = 0;
    let mut earliest_geode = u64::MAX;

    step_blueprint(limit, 1, &mut max, &mut earliest_geode, blueprint, root);

    max
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }

    (2..=n).product()
}

fn get_path_max(limit: u64, minute: u64, state: &State) -> u64 {
    let fact = state.robots.geode_robot + limit - minute;
    if fact > 20 {
        return u64::MAX;
    }
    factorial(fact) + state.resources.geode
}

fn step_blueprint(
    limit: u64,
    minute: u64,
    max: &mut u64,
    earliest_geode: &mut u64,
    blueprint: &Blueprint,
    mut node: ActionNode,
) {
    if let Action::BuildGeode = node.action {
        if node.state.earliest.is_none() {
            node.state.earliest = Some(minute);
        }
    }

    if let Some(earliest) = node.state.earliest {
        if earliest > *earliest_geode {
            return;
        } else {
            *earliest_geode = earliest;
        }
    }

    node.state.do_action(&node.action, blueprint);

    let path_max = get_path_max(limit, minute, &node.state);
    let geodes = node.state.resources.geode;
    if geodes > *max {
        *max = geodes;
    }

    if minute == limit || path_max < *max {
        return;
    }

    let mut actions = get_possible_actions(blueprint, &node.state.resources);
    let mut new_minutes = 0;
    // if all we can do is wait, don't bother creating child nodes
    // and cloning the state - just update the state directly
    while actions.len() == 1 {
        node.state.do_action(&actions[0], blueprint);
        actions = get_possible_actions(blueprint, &node.state.resources);

        new_minutes += 1;

        let geodes = node.state.resources.geode;
        if geodes > *max {
            *max = geodes;
        }

        if minute + new_minutes == limit {
            return;
        }
    }

    for next_action in actions {
        let state = node.state.clone();

        // prune paths where we're trying to build a robot (except geode robots)
        // but the extra resources it would provide don't increase our purchasing power
        // (we can only build 1 robot per minute)
        match next_action {
            Action::BuildOre => {
                if state.robots.ore_robot >= blueprint.get_max_ore_cost() {
                    continue;
                }
            }
            Action::BuildClay => {
                if state.robots.clay_robot >= blueprint.obsidian_robot.1 {
                    continue;
                }
            }
            Action::BuildObsidian => {
                if state.robots.obsidian_robot >= blueprint.geode_robot.1 {
                    continue;
                }
            }
            _ => (),
        }

        let child = ActionNode {
            state,
            action: next_action,
        };

        step_blueprint(
            limit,
            minute + new_minutes + 1,
            max,
            earliest_geode,
            blueprint,
            child,
        );
    }
}

use crate::{action::Action, blueprint::Blueprint};

#[derive(Debug, Clone)]
pub struct Robots {
    pub ore_robot: u64,
    pub clay_robot: u64,
    pub obsidian_robot: u64,
    pub geode_robot: u64,
}

impl Robots {
    pub fn new() -> Self {
        Self {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Resources {
    pub ore: u64,
    pub clay: u64,
    pub obsidian: u64,
    pub geode: u64,
}

#[derive(Debug, Clone)]
pub struct State {
    pub resources: Resources,
    pub robots: Robots,
    pub earliest: Option<u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            resources: Resources::default(),
            robots: Robots::new(),
            earliest: None,
        }
    }

    pub fn do_action(&mut self, action: &Action, blueprint: &Blueprint) {
        self.dig();

        match action {
            Action::BuildOre => {
                self.resources.ore -= blueprint.ore_robot;
                self.robots.ore_robot += 1;
            }
            Action::BuildClay => {
                self.resources.ore -= blueprint.clay_robot;
                self.robots.clay_robot += 1;
            }
            Action::BuildObsidian => {
                self.resources.ore -= blueprint.obsidian_robot.0;
                self.resources.clay -= blueprint.obsidian_robot.1;
                self.robots.obsidian_robot += 1;
            }
            Action::BuildGeode => {
                self.resources.ore -= blueprint.geode_robot.0;
                self.resources.obsidian -= blueprint.geode_robot.1;
                self.robots.geode_robot += 1;
            }
            Action::Wait => (),
        }
    }

    fn dig(&mut self) {
        self.resources.ore += self.robots.ore_robot;
        self.resources.clay += self.robots.clay_robot;
        self.resources.obsidian += self.robots.obsidian_robot;
        self.resources.geode += self.robots.geode_robot;
    }
}

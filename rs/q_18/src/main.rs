use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cube {
    pos: (usize, usize, usize),
}

impl Cube {
    pub fn from_str(input: &str) -> Self {
        let pos = input
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        Self { pos }
    }

    pub fn is_touching(&self, other: &Self) -> bool {
        if self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1 {
            usize_diff(&self.pos.2, &other.pos.2) < 2
        } else if self.pos.0 == other.pos.0 && self.pos.2 == other.pos.2 {
            usize_diff(&self.pos.1, &other.pos.1) < 2
        } else if self.pos.1 == other.pos.1 && self.pos.2 == other.pos.2 {
            usize_diff(&self.pos.0, &other.pos.0) < 2
        } else {
            false
        }
    }
}

fn usize_diff(a: &usize, b: &usize) -> usize {
    a.max(b) - a.min(b)
}

fn main() {
    let content = read_to_string("./data/input.txt").unwrap();

    let cubes = content
        .lines()
        .map(Cube::from_str)
        .collect_vec();

    let combinations = cubes
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .combinations(2)
        .collect_vec();

    let mut area = cubes.len() * 6;

    for combination in combinations {
        let id1 = combination[0];
        let id2 = combination[1];
        if cubes[id1].is_touching(&cubes[id2]) {
            area -= 2;
        }
    }

    println!("{:#?}", area);
}

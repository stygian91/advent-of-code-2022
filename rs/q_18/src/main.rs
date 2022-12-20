use std::{fs::read_to_string, ops::Index};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cube {
    pos: (usize, usize, usize),
    pub covered: u8,
}

impl Cube {
    pub fn from_str(input: &str) -> Self {
        let pos = input
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        Self { pos, covered: 0 }
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

    let mut cubes = content
        .lines()
        .map(|line| Cube::from_str(&line))
        .collect_vec();

    let combinations = cubes
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .combinations(2)
        .collect_vec();

    for combination in combinations {
        let id1 = combination[0];
        let id2 = combination[1];
        if cubes[id1].is_touching(&cubes[id2]) {
            cubes[id1].covered += 1;
            cubes[id2].covered += 1;
        }
    }

    let covered = cubes.iter().map(|c| c.covered as usize).sum::<usize>();
    let area = cubes.len() * 6 - covered;
    println!("{:#?}", area);
}

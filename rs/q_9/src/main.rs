#![allow(unused)]

#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, fs::read_to_string, path::Path};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

type Pos = (i32, i32);

#[derive(Debug)]
struct Rope {
    pub positions: Vec<Pos>,
    pub visited: BTreeSet<Pos>,
}

impl Rope {
    pub fn handle_move(&mut self, mv: &Move) {
        let pos_idx: Vec<usize> = (0..self.positions.len()).collect();

        for _ in 0..mv.amount {
            self.positions[0] = match mv.direction {
                Direction::Up => (self.positions[0].0, self.positions[0].1 + 1),
                Direction::Down => (self.positions[0].0, self.positions[0].1 - 1),
                Direction::Left => (self.positions[0].0 - 1, self.positions[0].1),
                Direction::Right => (self.positions[0].0 + 1, self.positions[0].1),
            };

            for win in pos_idx.windows(2) {
                self.handle_segment_move(win[0], win[1], mv);
            }

            self.visited.insert(*self.positions.last().unwrap());
        }
    }

    fn handle_segment_move(&mut self, head_idx: usize, tail_idx: usize, mv: &Move) {
        if are_neighbours(&self.positions[head_idx], &self.positions[tail_idx])
            || self.positions[head_idx] == self.positions[tail_idx]
        {
            return;
        }

        if self.positions[head_idx].0 == self.positions[tail_idx].0 {
            if self.positions[head_idx].1 > self.positions[tail_idx].1 {
                self.positions[tail_idx] =
                    (self.positions[tail_idx].0, self.positions[head_idx].1 - 1);
            } else {
                self.positions[tail_idx] =
                    (self.positions[tail_idx].0, self.positions[head_idx].1 + 1);
            }
        } else if self.positions[head_idx].1 == self.positions[tail_idx].1 {
            if self.positions[head_idx].0 > self.positions[tail_idx].0 {
                self.positions[tail_idx] =
                    (self.positions[head_idx].0 - 1, self.positions[tail_idx].1);
            } else {
                self.positions[tail_idx] =
                    (self.positions[head_idx].0 + 1, self.positions[tail_idx].1);
            }
        } else if self.positions[head_idx].0 < self.positions[tail_idx].0
            && self.positions[head_idx].1 > self.positions[tail_idx].1
        {
            self.positions[tail_idx] = (
                self.positions[tail_idx].0 - 1,
                self.positions[tail_idx].1 + 1,
            );
        } else if self.positions[head_idx].0 > self.positions[tail_idx].0
            && self.positions[head_idx].1 > self.positions[tail_idx].1
        {
            self.positions[tail_idx] = (
                self.positions[tail_idx].0 + 1,
                self.positions[tail_idx].1 + 1,
            );
        } else if self.positions[head_idx].0 > self.positions[tail_idx].0
            && self.positions[head_idx].1 < self.positions[tail_idx].1
        {
            self.positions[tail_idx] = (
                self.positions[tail_idx].0 + 1,
                self.positions[tail_idx].1 - 1,
            );
        } else {
            self.positions[tail_idx] = (
                self.positions[tail_idx].0 - 1,
                self.positions[tail_idx].1 - 1,
            );
        }
    }
}

impl Move {
    pub fn new(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<&str>>();
        let amount = parts[1].parse::<i32>().unwrap();
        let direction = match parts[0].chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        Self { direction, amount }
    }
}

fn parse_moves(path: &str) -> Vec<Move> {
    read_to_string(&Path::new(path))
        .unwrap()
        .lines()
        .map(|line| Move::new(line))
        .collect::<Vec<Move>>()
}

fn are_neighbours(pos1: &Pos, pos2: &Pos) -> bool {
    pos1.0 - 1 <= pos2.0 && pos2.0 <= pos1.0 + 1 && pos1.1 - 1 <= pos2.1 && pos2.1 <= pos1.1 + 1
}

fn part1(moves: &[Move]) -> usize {
    let mut rope = Rope {
        positions: vec![(0, 0), (0, 0)],
        visited: BTreeSet::new(),
    };
    rope.visited.insert((0, 0));

    for mv in moves {
        rope.handle_move(&mv);
    }

    rope.visited.len()
}

fn part2(moves: &[Move]) -> usize {
    let mut rope = Rope {
        positions: [(0, 0)].repeat(10),
        visited: BTreeSet::new(),
    };
    rope.visited.insert((0, 0));

    for mv in moves {
        rope.handle_move(&mv);
    }

    rope.visited.len()
}

fn main() {
    let moves = parse_moves("./data/input.txt");

    let part1_res = part1(&moves);
    println!("{:#?}", part1_res);

    let part2_res = part2(&moves);
    println!("{:#?}", part2_res);
}

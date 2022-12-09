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

struct Move {
    direction: Direction,
    amount: i32,
}

type Pos = (i32, i32);

#[derive(Debug)]
struct Rope {
    pub head: Pos,
    pub tail: Pos,
    pub tail_positions: BTreeSet<Pos>,
}

impl Rope {
    pub fn handle_move(&mut self, mv: &Move) {
        for _ in 0..mv.amount {
            self.head = match mv.direction {
                Direction::Up => (self.head.0, self.head.1 + 1),
                Direction::Down => (self.head.0, self.head.1 - 1),
                Direction::Left => (self.head.0 - 1, self.head.1),
                Direction::Right => (self.head.0 + 1, self.head.1),
            };

            if are_neighbours(&self.head, &self.tail) || self.head == self.tail {
                continue;
            }

            if self.head.0 == self.tail.0 {
                if self.head.1 > self.tail.1 {
                    self.tail = (self.tail.0, self.head.1 - 1);
                } else {
                    self.tail = (self.tail.0, self.head.1 + 1);
                }
            } else if self.head.1 == self.tail.1 {
                if self.head.0 > self.tail.0 {
                    self.tail = (self.head.0 - 1, self.tail.1);
                } else {
                    self.tail = (self.head.0 + 1, self.tail.1);
                }
            } else if self.head.0 < self.tail.0 && self.head.1 > self.tail.1 {
                self.tail = (self.tail.0 - 1, self.tail.1 + 1);
            } else if self.head.0 > self.tail.0 && self.head.1 > self.tail.1 {
                self.tail = (self.tail.0 + 1, self.tail.1 + 1);
            } else if self.head.0 > self.tail.0 && self.head.1 < self.tail.1 {
                self.tail = (self.tail.0 + 1, self.tail.1 - 1);
            } else {
                self.tail = (self.tail.0 - 1, self.tail.1 - 1);
            }

            self.tail_positions.insert(self.tail);
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

fn part1(moves: &[Move], rope: &mut Rope) -> usize {
    rope.tail_positions.insert((0, 0));

    for mv in moves {
        rope.handle_move(&mv);
    }

    rope.tail_positions.len()
}

fn main() {
    let moves = parse_moves("./data/input.txt");

    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
        tail_positions: BTreeSet::new(),
    };

    let part1_res = part1(&moves, &mut rope);
    println!("{:#?}", part1_res);
}

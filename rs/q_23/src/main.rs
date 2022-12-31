#![allow(unused)]

use direction::{rotate_proposed, Direction, STARTING_DIRECTIONS};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    iter::Filter,
};

use aoc_common::{BTreeMap2D, Coord};
use tile::Tile;

mod direction;
mod tile;

fn parse(input: &str) -> BTreeMap2D<Tile> {
    let mut map = BTreeMap2D::new();

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let tile = match ch {
                '#' => Tile::Elf,
                _ => Tile::Empty,
            };

            map.insert((i as isize, j as isize), tile);
        }
    }

    map
}

fn has_elves<'a, I: Iterator<Item = &'a (&'a Coord, &'a Tile)>>(tiles: I) -> bool {
    tiles.fold(false, |acc, (_, tile)| acc || matches!(tile, Tile::Elf))
}

fn move_elf(from: &Coord, to: &Coord, map: &mut BTreeMap2D<Tile>) {
    *map.entry(*from).or_insert(Tile::Empty) = Tile::Empty;
    *map.entry(*to).or_insert(Tile::Elf) = Tile::Elf;
}

fn round(map: &mut BTreeMap2D<Tile>, proposed_dirs: &mut VecDeque<Direction>) {
    let mut proposed = HashMap::new();

    // first phase of round:
    // gather propositions for each elf
    for (pos, tile) in map.iter() {
        if matches!(tile, Tile::Empty) {
            continue;
        }

        let neighbours = map.neighbours(*pos).collect_vec();
        if !has_elves(neighbours.iter()) {
            continue;
        }

        for dir in proposed_dirs.iter() {
            let dir_neighbours = match dir {
                Direction::N => [&neighbours[0], &neighbours[1], &neighbours[2]],
                Direction::E => [&neighbours[2], &neighbours[4], &neighbours[7]],
                Direction::S => [&neighbours[5], &neighbours[6], &neighbours[7]],
                Direction::W => [&neighbours[0], &neighbours[3], &neighbours[5]],
                _ => panic!("Unexpected direction in proposed directions."),
            };

            let dir_neighbours_has_elves = has_elves(dir_neighbours.into_iter());
            if !dir_neighbours_has_elves {
                let proposed_pos = match dir {
                    Direction::N => (pos.0 - 1, pos.1),
                    Direction::E => (pos.0, pos.1 + 1),
                    Direction::S => (pos.0 + 1, pos.1),
                    Direction::W => (pos.0, pos.1 - 1),
                    _ => panic!("Unexpected direction when adding proposed direction"),
                };
                let current_proposed = proposed.entry(proposed_pos).or_insert(vec![]);
                current_proposed.push(*pos);
                break;
            }
        }
    }

    // second phase of round:
    // move elves around based on their propositions
    for (proposed_pos, propositioners) in proposed.iter() {
        if propositioners.len() != 1 {
            continue;
        }

        move_elf(&propositioners[0], proposed_pos, map);
    }

    rotate_proposed(proposed_dirs);
}

// TODO: write function that finds the AABB of all elves

fn part1(input: &str) {
    let mut map = parse(input);
    let mut proposed = VecDeque::from(STARTING_DIRECTIONS);
    for _ in 0..10 {
        round(&mut map, &mut proposed);
    }
}

fn main() {
    let input = read_to_string("./data/demo.txt").unwrap();
    part1(&input);
}

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

fn count_elves<'a, I: Iterator<Item = &'a (&'a Coord, &'a Tile)>>(tiles: I) -> usize {
    tiles.filter(|(_, tile)| matches!(tile, Tile::Elf)).count()
}

fn has_elves<'a, I: Iterator<Item = &'a (&'a Coord, &'a Tile)>>(tiles: I) -> bool {
    count_elves(tiles) > 0
}

fn move_elf(from: &Coord, to: &Coord, map: &mut BTreeMap2D<Tile>) {
    *map.entry(*from).or_insert(Tile::Empty) = Tile::Empty;
    *map.entry(*to).or_insert(Tile::Elf) = Tile::Elf;
}

fn filter_neighbours<'a>(
    pos: &Coord,
    dir: &Direction,
    neighbours: &'a [(&'a (isize, isize), &'a Tile)],
) -> Vec<&'a (&'a (isize, isize), &'a Tile)> {
    neighbours.iter().filter(|(&c_pos, _)| {
            match dir {
                Direction::N => c_pos.0 == pos.0 - 1 && c_pos.1 >= pos.1 - 1 && c_pos.1 <= pos.1 + 1,
                Direction::E => c_pos.1 == pos.1 + 1 && c_pos.0 >= pos.0 - 1 && c_pos.0 <= pos.0 + 1,
                Direction::S => c_pos.0 == pos.0 + 1 && c_pos.1 >= pos.1 - 1 && c_pos.1 <= pos.1 + 1,
                Direction::W => c_pos.1 == pos.1 + 1 && c_pos.0 >= pos.0 - 1 && c_pos.0 <= pos.0 + 1,
                _ => panic!("Unexpected direction in proposed directions."),
            }
    }).collect_vec()
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
            let dir_neighbours = filter_neighbours(pos, dir, &neighbours);

            if !has_elves(dir_neighbours.into_iter()) {
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

fn get_bounding_rectangle(map: &BTreeMap2D<Tile>) -> (Coord, Coord) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    map.iter()
        .filter(|(_, tile)| matches!(tile, Tile::Elf))
        .for_each(|(pos, _)| {
            if min_x > pos.1 {
                min_x = pos.1;
            }

            if min_y > pos.0 {
                min_y = pos.0;
            }

            if max_x < pos.1 {
                max_x = pos.1;
            }

            if max_y < pos.0 {
                max_y = pos.0;
            }
        });

    ((min_y, min_x), (max_y, max_x))
}

fn count_empty(aabb: (Coord, Coord), elves: usize) -> usize {
    let h = (aabb.1 .0 - aabb.0 .0).abs();
    let w = (aabb.1 .1 - aabb.0 .1).abs();
    let total = h * w;
    println!("w: {w} h: {h} total: {total}");

    total as usize - elves
}

fn print_map(map: &BTreeMap2D<Tile>) {
    let aabb = get_bounding_rectangle(&map);
    let mut buff = String::new();
    for y in aabb.0.0..=aabb.1.0 {
        for x in aabb.0.1..=aabb.1.1 {
            let el = map.get(&(y, x)).unwrap_or(&Tile::Empty);
            match el {
                Tile::Empty => buff.push('.'),
                Tile::Elf => buff.push('#'),
            }
        }
        buff.push('\n');
    }

    print!("{buff}");
}

fn part1(input: &str) {
    let mut map = parse(input);
    let elves = map.iter().filter(|(_, tile)| matches!(tile, Tile::Elf)).count();
    let mut proposed = VecDeque::from(STARTING_DIRECTIONS);
    for _ in 0..10 {
        round(&mut map, &mut proposed);
    }

    print_map(&map);

    let aabb = get_bounding_rectangle(&map);
    let empty = count_empty(aabb, elves);

    println!("Part 1: {empty}");
}

fn main() {
    let input = read_to_string("./data/demo.txt").unwrap();
    part1(&input);
}

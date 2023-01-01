use direction::{rotate_proposed, Direction, STARTING_DIRECTIONS};
use itertools::Itertools;
use std::{
    collections::{BTreeMap, VecDeque},
    fs::read_to_string,
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
    neighbours
        .iter()
        .filter(|(&c_pos, _)| match dir {
            Direction::N => c_pos.0 == pos.0 - 1 && c_pos.1 >= pos.1 - 1 && c_pos.1 <= pos.1 + 1,
            Direction::E => c_pos.1 == pos.1 + 1 && c_pos.0 >= pos.0 - 1 && c_pos.0 <= pos.0 + 1,
            Direction::S => c_pos.0 == pos.0 + 1 && c_pos.1 >= pos.1 - 1 && c_pos.1 <= pos.1 + 1,
            Direction::W => c_pos.1 == pos.1 - 1 && c_pos.0 >= pos.0 - 1 && c_pos.0 <= pos.0 + 1,
        })
        .collect_vec()
}

fn round(map: &mut BTreeMap2D<Tile>, proposed_dirs: &mut VecDeque<Direction>) -> usize {
    let mut proposed = BTreeMap::new();

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
                };
                let current_proposed: &mut Vec<(isize, isize)> = proposed.entry(proposed_pos).or_default();
                current_proposed.push(*pos);
                break;
            }
        }
    }

    // second phase of round:
    // move elves around based on their propositions
    let mut move_count = 0;
    for (proposed_pos, propositioners) in proposed.iter() {
        if propositioners.len() != 1 {
            continue;
        }

        move_count += 1;
        move_elf(&propositioners[0], proposed_pos, map);
    }

    rotate_proposed(proposed_dirs);
    move_count
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
    let h = (aabb.1 .0 - aabb.0 .0).abs() + 1;
    let w = (aabb.1 .1 - aabb.0 .1).abs() + 1;
    let total = h * w;

    total as usize - elves
}

fn map_to_string(map: &BTreeMap2D<Tile>) -> String {
    let aabb = get_bounding_rectangle(map);
    let mut buff = String::new();
    for y in aabb.0 .0..=aabb.1 .0 {
        for x in aabb.0 .1..=aabb.1 .1 {
            let el = map.get(&(y, x)).unwrap_or(&Tile::Empty);
            match el {
                Tile::Empty => buff.push('.'),
                Tile::Elf => buff.push('#'),
            }
        }
        buff.push('\n');
    }

    buff
}

#[allow(unused)]
fn print_map(map: &BTreeMap2D<Tile>) {
    print!("{}", map_to_string(map));
}

fn part1(input: &str) {
    let mut map = parse(input);
    let elves = map
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::Elf))
        .count();
    let mut proposed = VecDeque::from(STARTING_DIRECTIONS);
    for _ in 0..10 {
        round(&mut map, &mut proposed);
    }

    let aabb = get_bounding_rectangle(&map);
    let empty = count_empty(aabb, elves);

    println!("Part 1: {empty}");
}

fn part2(input: &str) {
    let mut map = parse(input);
    let mut proposed = VecDeque::from(STARTING_DIRECTIONS);
    for i in 1.. {
        let move_count = round(&mut map, &mut proposed);
        if move_count == 0 {
            println!("Part 2: {i}");
            break;
        }
    }
}

fn main() {
    let input = read_to_string("./data/input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_works() {
        let input = "##
#.
..
##";
        let mut map = parse(input);
        let mut proposed = VecDeque::from(STARTING_DIRECTIONS);
        round(&mut map, &mut proposed);
        round(&mut map, &mut proposed);
        let expected = ".##.
#...
...#
....
.#..
";

        assert_eq!(map_to_string(&map), expected);
    }

    #[test]
    fn filter_neighbours_works() {
        let neighbours = vec![
            (&(0, 0), &Tile::Elf),
            (&(0, 1), &Tile::Empty),
            (&(0, 2), &Tile::Empty),
            (&(1, 0), &Tile::Empty),
            (&(1, 2), &Tile::Empty),
            (&(2, 0), &Tile::Empty),
            (&(2, 1), &Tile::Empty),
            (&(2, 2), &Tile::Elf),
        ];

        assert_eq!(
            filter_neighbours(&(1, 1), &Direction::N, &neighbours),
            vec![
                &(&(0, 0), &Tile::Elf),
                &(&(0, 1), &Tile::Empty),
                &(&(0, 2), &Tile::Empty),
            ]
        );

        assert_eq!(
            filter_neighbours(&(1, 1), &Direction::E, &neighbours),
            vec![
                &(&(0, 2), &Tile::Empty),
                &(&(1, 2), &Tile::Empty),
                &(&(2, 2), &Tile::Elf),
            ]
        );

        assert_eq!(
            filter_neighbours(&(1, 1), &Direction::W, &neighbours),
            vec![
                &(&(0, 0), &Tile::Elf),
                &(&(1, 0), &Tile::Empty),
                &(&(2, 0), &Tile::Empty),
            ]
        );
    }
}

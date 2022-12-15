use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, BTreeSet},
    fs::read_to_string,
};
use itertools::Itertools;

type Coord = (isize, isize);

#[derive(Debug, PartialEq)]
struct Sensor {
    distance: u32,
    beacon: Coord,
}

fn parse_line(line: &str) -> (Coord, Sensor) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#
        )
        .unwrap();
    }

    let captures = RE.captures(line).unwrap();
    let get_capture = |i| captures.get(i).unwrap().as_str().parse::<isize>().unwrap();

    let pos = (get_capture(1), get_capture(2));
    let beacon = (get_capture(3), get_capture(4));

    (
        pos,
        Sensor {
            distance: manhattan_distance(&pos, &beacon),
            beacon,
        },
    )
}

fn manhattan_distance(a: &Coord, b: &Coord) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn part1(input: &str, target_row: isize) -> usize {
    let iter = input.lines().map(|line| parse_line(line));
    let mut sensors = HashMap::new();
    let mut beacons = HashSet::new();
    for (coord, sensor) in iter {
        beacons.insert(sensor.beacon);
        sensors.insert(coord, sensor);
    }

    let ranges = sensors.iter().filter(|(coords, sensor)| {
        let diff = (coords.1 - target_row).abs();
        (diff as u32) <= sensor.distance
    }).map(|(coords, sensor)| {
        let r_pos = (coords.0, target_row);
        let md_diff = sensor.distance - manhattan_distance(coords, &r_pos);
        let row_width = 2 * md_diff + 1;
        let min_x = r_pos.0 - (row_width as isize / 2);
        let max_x = r_pos.0 + (row_width as isize / 2);
        min_x..=max_x
    }).collect_vec();

    // this is lazy and wastes space
    // we could merge the ranges as much as possible
    // then get their individual counts and sum them
    let mut row_points = HashSet::new();
    for range in ranges {
        for x in range {
            row_points.insert(x);
        }
    }

    let beacon_count = beacons.iter().filter(|beacon| beacon.1 == target_row).count();

    row_points.len() - beacon_count
}

// fn part2(input: &str, min: isize, max: isize) {

// }

fn main() {
    let input = read_to_string("./data/input.txt").unwrap();
    let p1_res = part1(&input, 2000000);
    println!("{}", p1_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &'static str = include_str!("../data/demo.txt");

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            (
                (2, 18),
                Sensor {
                    beacon: (-2, 15),
                    distance: 7,
                }
            )
        );
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(DEMO, 10), 26);
    }
}

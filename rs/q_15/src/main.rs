use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive, cmp::Ordering,
};

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

fn parse_input(input: &str) -> (HashMap<Coord, Sensor>, HashSet<Coord>) {
    let iter = input.lines().map(|line| parse_line(line));
    let mut sensors = HashMap::new();
    let mut beacons = HashSet::new();
    for (coord, sensor) in iter {
        beacons.insert(sensor.beacon);
        sensors.insert(coord, sensor);
    }

    (sensors, beacons)
}

fn get_row_ranges(
    sensors: &HashMap<Coord, Sensor>,
    target_row: isize,
) -> Vec<RangeInclusive<isize>> {
    let mut ranges = sensors
        .iter()
        .filter(|(coords, sensor)| {
            let diff = (coords.1 - target_row).abs();
            (diff as u32) <= sensor.distance
        })
        .map(|(coords, sensor)| {
            let r_pos = (coords.0, target_row);
            let md_diff = sensor.distance - manhattan_distance(coords, &r_pos);
            let row_width = 2 * md_diff + 1;
            let min_x = r_pos.0 - (row_width as isize / 2);
            let max_x = r_pos.0 + (row_width as isize / 2);
            min_x..=max_x
        })
        .collect_vec();

        ranges.sort_by(|a, b| {
            let diff = a.start() - b.start();
            if diff == 0 {
                return Ordering::Equal;
            }

            if diff > 0 {
                return Ordering::Greater;
            }

            Ordering::Less
        });

        ranges
}

fn part1(input: &str, target_row: isize) -> usize {
    let (sensors, beacons) = parse_input(input);
    let ranges = get_row_ranges(&sensors, target_row);

    let count: isize = merge_all_ranges(&ranges)
        .iter()
        .map(|range| (range.start() - range.end()).abs() + 1)
        .sum();

    let beacon_count = beacons
        .iter()
        .filter(|beacon| beacon.1 == target_row)
        .count();

    count as usize - beacon_count
}

fn range_intersects<T: PartialOrd>(first: &RangeInclusive<T>, second: &RangeInclusive<T>) -> bool {
    (first.start() <= second.end() && first.end() >= second.end())
        || (first.end() >= second.start() && second.end() >= first.end())
}

fn merge_ranges<T: Ord + Copy>(
    first: &RangeInclusive<T>,
    second: &RangeInclusive<T>,
) -> RangeInclusive<T> {
    *first.start().min(second.start())..=*first.end().max(second.end())
}

fn merge_all_ranges(ranges: &Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    if ranges.len() <= 1 {
        return ranges.clone();
    }

    let mut res = vec![ranges[0].clone()];

    for i in 1..ranges.len() {
        let last = res.last_mut().unwrap();
        if range_intersects(last, &ranges[i]) {
            *last = merge_ranges(last, &ranges[i]);
        } else {
            res.push(ranges[i].clone());
        }
    }

    res
}

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
    fn range_includes_works() {
        assert!(range_intersects(&(1..=4), &(1..=3)));
        assert!(range_intersects(&(1..=4), &(1..=4)));
        assert!(range_intersects(&(2..=6), &(3..=6)));
        assert!(range_intersects(&(2..=6), &(3..=5)));

        assert!(range_intersects(&(1..=4), &(0..=1)));
        assert!(range_intersects(&(1..=4), &(3..=5)));

        assert!(!range_intersects(&(1..=4), &(5..=6)));
        assert!(!range_intersects(&(3..=5), &(0..=2)));
    }

    #[test]
    fn merge_ranges_works() {
        assert_eq!(merge_ranges(&(1..=3), &(2..=5)), 1..=5);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(DEMO, 10), 26);
    }
}

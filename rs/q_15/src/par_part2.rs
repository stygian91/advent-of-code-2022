use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::{row_search_beacon, Coord, Sensor};

pub fn part2_par(sensors: &HashMap<Coord, Sensor>, beacons: &HashSet<Coord>, max: isize) -> usize {
    let beacon = par_get_beacon_pos(sensors, beacons, max).unwrap();

    beacon.0 as usize * 4_000_000 + beacon.1 as usize
}

fn par_get_beacon_pos(
    sensors: &HashMap<Coord, Sensor>,
    beacons: &HashSet<Coord>,
    search_max: isize,
) -> Option<Coord> {
    (0..=search_max)
        .into_par_iter()
        .find_map_any(|i| row_search_beacon(sensors, beacons, i, search_max))
}

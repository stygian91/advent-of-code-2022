use std::collections::{HashMap, HashSet};

use crate::{Coord, HeightMap};

#[derive(Debug)]
pub struct Graph {
    pub heights: Vec<Vec<u8>>,
}

impl Graph {
    pub fn new(heightmap: HeightMap) -> Self {
        Self { heights: heightmap.heights }
    }

    pub fn shortest(&mut self, source: &Coord) -> HashMap<Coord, u64> {
        let mut dist = HashMap::new();
        // let mut prev: HashMap<Coord, Option<u64>> = HashMap::new();
        let mut q = HashSet::new();

        for (i, row) in self.heights.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let coord = Coord(i, j);
                q.insert(coord.clone());
                dist.insert(coord.clone(), u64::MAX);
                // prev.insert(coord.clone(), None);
            }
        }

        *dist.get_mut(source).unwrap() = 0;

        while !q.is_empty() {
            let u = lowest_distance(&dist, &q).unwrap();
            q.remove(&u);

            let neighbours = get_neighbours(&u, &self.heights);
            let neighbours = neighbours
                .iter()
                .filter(|nbr| q.contains(*nbr))
                .collect::<Vec<_>>();

            for v in neighbours {
                let u_dist = dist.get(&u).unwrap();
                let alt = u_dist + 1;
                let v_mut = dist.get_mut(v).unwrap();
                if alt < *v_mut {
                    *v_mut = alt;
                    // *prev.get_mut(v).unwrap() = Some();
                }
            }
        }

        dist
    }
}

fn lowest_distance(dist: &HashMap<Coord, u64>, q: &HashSet<Coord>) -> Option<Coord> {
    let mut min = None;

    for coord in q {
        let value = dist.get(coord).unwrap();

        if min.is_none() {
            min = Some(coord.clone());
        }

        let m = min.unwrap();
        if dist.get(&m).unwrap() > value {
            min = Some(coord.clone())
        } else {
            min = Some(m);
        }

        // min = match min {
        //     Some(m) => {
        //         if dist.get(&m).unwrap() > value {
        //             Some(coord.clone())
        //         } else {
        //             Some(m)
        //         }
        //     }
        //     None => Some(coord.clone()),
        // };
    }

    min
}

fn is_valid_neighbour(current: u8, neighbour: u8) -> bool {
    neighbour == current || (neighbour > 0 && neighbour - 1 == current) || neighbour + 1 == current
}

fn get_neighbours(pos: &Coord, heights: &Vec<Vec<u8>>) -> Vec<Coord> {
    let mut neighbours = vec![];
    let el = heights[pos.0][pos.1];

    if pos.0 > 0 {
        let top = heights[pos.0 - 1][pos.1];
        if is_valid_neighbour(el, top) {
            neighbours.push(Coord(pos.0 - 1, pos.1));
        }
    }

    if pos.1 > 0 {
        let left = heights[pos.0][pos.1 - 1];
        if is_valid_neighbour(el, left) {
            neighbours.push(Coord(pos.0, pos.1 - 1));
        }
    }

    if let Some(right) = heights[pos.0].get(pos.1 + 1) {
        if is_valid_neighbour(el, *right) {
            neighbours.push(Coord(pos.0, pos.1 + 1));
        }
    }

    if let Some(bottom) = heights.get(pos.0 + 1).and_then(|r| r.get(pos.1)) {
        if is_valid_neighbour(el, *bottom) {
            neighbours.push(Coord(pos.0 + 1, pos.1));
        }
    }

    neighbours
}

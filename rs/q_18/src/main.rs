use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

type Coord = (isize, isize, isize);

struct BoundingBox {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl BoundingBox {
    pub fn new(cubes: &HashSet<Coord>) -> Self {
        Self {
            min_x: cubes.iter().fold(isize::MAX, |acc, c| acc.min(c.0)) - 1,
            max_x: cubes.iter().fold(isize::MIN, |acc, c| acc.max(c.0)) + 1,
            min_y: cubes.iter().fold(isize::MAX, |acc, c| acc.min(c.1)) - 1,
            max_y: cubes.iter().fold(isize::MIN, |acc, c| acc.max(c.1)) + 1,
            min_z: cubes.iter().fold(isize::MAX, |acc, c| acc.min(c.2)) - 1,
            max_z: cubes.iter().fold(isize::MIN, |acc, c| acc.max(c.2)) + 1,
        }
    }

    pub fn is_inside(&self, pos: &Coord) -> bool {
        pos.0 >= self.min_x
            && pos.0 <= self.max_x
            && pos.1 >= self.min_y
            && pos.1 <= self.max_y
            && pos.2 >= self.min_z
            && pos.2 <= self.max_z
    }

    pub fn outside(&self, cubes: &HashSet<Coord>) -> HashSet<Coord> {
        let mut res = HashSet::new();
        let mut queue = Vec::from([(self.min_x, self.min_y, self.min_z)]);

        while let Some(p) = queue.pop() {
            if !cubes.contains(&p) && !res.contains(&p) && self.is_inside(&p) {
                res.insert(p);
                get_neighbours(&p).iter().for_each(|n| queue.push(*n));
            }
        }

        res
    }
}

fn parse_coord(input: &str) -> Coord {
    input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn sides_touching(pos: &Coord, cubes: &HashSet<Coord>) -> isize {
    get_neighbours(pos)
        .iter()
        .filter(|c| cubes.contains(c))
        .count() as isize
}

fn get_neighbours(pos: &Coord) -> Vec<Coord> {
    vec![
        (pos.0 - 1, pos.1, pos.2),
        (pos.0 + 1, pos.1, pos.2),
        (pos.0, pos.1 - 1, pos.2),
        (pos.0, pos.1 + 1, pos.2),
        (pos.0, pos.1, pos.2 - 1),
        (pos.0, pos.1, pos.2 + 1),
    ]
}

fn part1(input: &str) {
    let cubes = input.lines().map(parse_coord).collect::<HashSet<Coord>>();
    let res: isize = cubes.iter().map(|p| 6 - sides_touching(p, &cubes)).sum();
    println!("Part 1: {}", res);
}

fn part2(input: &str) {
    let cubes = input.lines().map(parse_coord).collect::<HashSet<Coord>>();
    let outside = BoundingBox::new(&cubes).outside(&cubes);
    let res: isize = cubes.iter().map(|c| sides_touching(c, &outside)).sum();
    println!("Part 2: {}", res);
}

fn main() {
    let content = read_to_string("./data/input.txt").unwrap();
    part1(&content);
    part2(&content);
}

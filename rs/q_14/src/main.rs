use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

type Coord = (i32, i32);

const DEFAULT_START: Coord = (500, 0);

enum Tile {
    Rock,
    Sand,
}

struct Scene {
    tiles: HashMap<Coord, Tile>,
    current: Coord,
    max_y: i32,
}

impl Scene {
    pub fn new(input: &str) -> Self {
        let mut scene = Self {
            tiles: HashMap::new(),
            current: (500, 0),
            max_y: i32::MIN,
        };

        for line in input.split('\n') {
            let rock_lines = parse_line(line);
            for rock_line in rock_lines {
                scene.add_rocks(&rock_line);
            }
        }

        scene.update_max_y();

        scene
    }

    pub fn get_sand_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Sand))
            .count()
    }

    pub fn reset_current(&mut self) {
        self.current = DEFAULT_START;
    }

    pub fn add_sand(&mut self, coords: Coord) {
        self.tiles.insert(coords, Tile::Sand);
    }

    fn update_max_y(&mut self) {
        let mut max = i32::MIN;

        for (coords, tile) in self.tiles.iter() {
            if let Tile::Sand = *tile {
                continue;
            }

            if coords.1 > max {
                max = coords.1;
            }
        }

        self.max_y = max;
    }

    fn add_rocks(&mut self, rock_line: &(Coord, Coord)) {
        if is_horizontal(rock_line) {
            let minx = rock_line.0 .0.min(rock_line.1 .0);
            let maxx = rock_line.0 .0.max(rock_line.1 .0);

            for i in minx..=maxx {
                self.tiles.insert((i, rock_line.0 .1), Tile::Rock);
            }
        } else {
            let miny = rock_line.0 .1.min(rock_line.1 .1);
            let maxy = rock_line.0 .1.max(rock_line.1 .1);

            for i in miny..=maxy {
                self.tiles.insert((rock_line.0 .0, i), Tile::Rock);
            }
        }
    }

    pub fn tick(&mut self) {
        match self.try_move() {
            Some(next) => {
                self.current = next;
            }
            None => {
                self.tiles.insert(self.current, Tile::Sand);
                self.add_sand(self.current);
                self.reset_current();
            }
        };
    }

    fn try_move(&mut self) -> Option<Coord> {
        let mut next = (self.current.0, self.current.1 + 1);
        if !self.tiles.contains_key(&next) {
            return Some(next);
        }

        next = (self.current.0 - 1, self.current.1 + 1);
        if !self.tiles.contains_key(&next) {
            return Some(next);
        }

        next = (self.current.0 + 1, self.current.1 + 1);
        if !self.tiles.contains_key(&next) {
            return Some(next);
        }

        None
    }
}

fn is_horizontal(rock_line: &(Coord, Coord)) -> bool {
    rock_line.0 .1 == rock_line.1 .1
}

fn parse_line(line: &str) -> Vec<(Coord, Coord)> {
    line.split(" -> ")
        .map(|coords| {
            coords
                .split(',')
                .map(|coord| coord.parse::<i32>().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .tuple_windows::<(Coord, Coord)>()
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let mut scene = Scene::new(input);
    let mut done = false;

    while !done {
        scene.tick();
        if scene.current.1 > scene.max_y {
            done = true;
        }
    }

    scene.get_sand_count()
}

fn part2(input: &str) -> usize {
    let mut scene = Scene::new(input);
    let mut done = false;

    while !done {
        if scene.tiles.contains_key(&DEFAULT_START) {
            done = true;
        }

        scene.tick();

        if scene.current.1 == scene.max_y + 2 {
            scene.add_sand((scene.current.0, scene.current.1 - 1));
            scene.reset_current();
        }
    }

    scene.get_sand_count()
}

fn main() {
    let input = read_to_string("./data/input.txt").unwrap();
    let part1_res = part1(&input);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&input);
    println!("Part 2: {}", part2_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!(part1(DEMO), 24);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(DEMO), 93)
    }
}

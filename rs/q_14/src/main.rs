#![allow(unused)]

use std::{collections::HashSet, fs::{read_to_string, File}};
use itertools::Itertools;

type Coord = (i32, i32);

const DEMO: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
const DEFAULT_START: Coord = (500, 0);

#[derive(Debug, PartialEq)]
struct RockLine {
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
struct Scene {
    rocks: Vec<RockLine>,
    sands: HashSet<Coord>,
    start: Coord,
    current: Coord,
    done: bool,
    max_y: i32,
}

impl Scene {
    pub fn new(input: &str) -> Self {
        let mut rocks = vec![];

        for line in input.split('\n') {
            rocks.extend(parse_line(line));
        }

        let max_y = rocks.iter().map(|rock| rock.get_max_y()).max().unwrap();

        Self {
            rocks,
            sands: HashSet::new(),
            start: DEFAULT_START,
            current: (500, 0),
            done: false,
            max_y,
        }
    }

    pub fn tick(&mut self) {
        match self.try_move() {
            Some(next) => {
                self.current = next;
                if self.current.1 > self.max_y {
                    self.done = true;
                }
            }
            None => {
                self.sands.insert(self.current);
                self.current = DEFAULT_START;
            }
        };
    }

    fn try_move(&mut self) -> Option<Coord> {
        let mut next = (self.current.0, self.current.1 + 1);
        if self.is_free_space(&next) {
            return Some(next);
        }

        next = (self.current.0 - 1, self.current.1 + 1);
        if self.is_free_space(&next) {
            return Some(next);
        }

        next = (self.current.0 + 1, self.current.1 + 1);
        if self.is_free_space(&next) {
            return Some(next);
        }

        None
    }

    fn is_free_space(&self, pos: &Coord) -> bool {
        !self.rocks_contain(pos) && !self.sands.contains(pos)
    }

    fn rocks_contain(&self, point: &Coord) -> bool {
        for rock in &self.rocks {
            if rock.contains(&point) {
                return true;
            }
        }

        false
    }
}

impl RockLine {
    pub fn contains(&self, point: &Coord) -> bool {
        if self.is_horizontal() {
            let minx = self.start.0.min(self.end.0);
            let maxx = self.start.0.max(self.end.0);
            return point.1 == self.start.1 && (minx..=maxx).contains(&point.0);
        }

        let miny = self.start.1.min(self.end.1);
        let maxy = self.start.1.max(self.end.1);
        point.0 == self.start.0 && (miny..=maxy).contains(&point.1)
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    pub fn get_max_y(&self) -> i32 {
        if self.is_horizontal() {
            return self.start.1;
        }

        self.start.1.max(self.end.1)
    }
}

fn parse_line(line: &str) -> Vec<RockLine> {
    line.split(" -> ")
        .map(|coords| {
            coords
                .split(',')
                .map(|coord| coord.parse::<i32>().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .tuple_windows::<(Coord, Coord)>()
        .map(|(start, end)| RockLine { start, end })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let mut scene = Scene::new(input);

    while !scene.done {
        scene.tick();
    }

    scene.sands.len()
}
fn main() {
    let content = read_to_string("./data/input.txt").unwrap();
    // let guard = pprof::ProfilerGuardBuilder::default()
    //     .frequency(1000)
    //     .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    //     .build()
    //     .unwrap();
    let part1_res = part1(&content);
    // if let Ok(report) = guard.report().build() {
    //     let file = File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // }
    println!("Part 1: {}", part1_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let lines = DEMO.split('\n').collect_vec();

        assert_eq!(
            parse_line(lines[0]),
            vec![
                RockLine {
                    start: (498, 4),
                    end: (498, 6)
                },
                RockLine {
                    start: (498, 6),
                    end: (496, 6)
                },
            ]
        );

        assert_eq!(
            parse_line(lines[1]),
            vec![
                RockLine {
                    start: (503, 4),
                    end: (502, 4)
                },
                RockLine {
                    start: (502, 4),
                    end: (502, 9)
                },
                RockLine {
                    start: (502, 9),
                    end: (494, 9),
                },
            ]
        );
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(DEMO), 24);
    }
}

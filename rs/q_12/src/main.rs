use std::{fs::read_to_string, path::Path};

use crate::dijkstra::Graph;

mod dijkstra;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord(pub usize, pub usize);

#[derive(Debug)]
pub struct HeightMap {
    pub heights: Vec<Vec<u8>>,
    pub start: Coord,
    pub end: Coord,
}

fn char_to_height(ch: char) -> u8 {
    match ch {
        'S' => 0,
        'E' => 25,
        c => c as u8 - 'a' as u8,
    }
}

fn parse_map(path: &str) -> HeightMap {
    let contents = read_to_string(Path::new(path)).unwrap();
    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);
    let mut heights = vec![];

    for (i, line) in contents.lines().enumerate() {
        let mut row = vec![];

        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Coord(i, j);
            } else if ch == 'E' {
                end = Coord(i, j);
            }

            row.push(char_to_height(ch));
        }

        heights.push(row);
    }

    HeightMap {
        heights,
        start,
        end,
    }
}

fn part1(path: &str) -> u64 {
    let map = parse_map(path);
    let start = map.start.clone();
    let end = map.end.clone();
    let mut graph = Graph::new(map);
    let distances = graph.shortest(&start);
    // println!("{:#?}", distances);
    *distances.get(&end).unwrap()
}

fn main() {
    // let part1_res = part1("./data/demo.txt");
    let part1_res = part1("./data/input.txt");
    println!("part 1: {:#?}", part1_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("./data/demo.txt"), 31);
    }
}

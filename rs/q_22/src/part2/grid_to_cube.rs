use std::collections::{HashMap, HashSet};

use super::cube::FaceIdentity::*;
use super::cube::{Cube, Face, FaceIdentity};

fn transform(face_size: usize, grid: &Vec<Vec<char>>) -> Cube {
    let mut faces: HashMap<FaceIdentity, Face> = HashMap::new();
    let mut face_coords = vec![];

    for (y, row) in grid.iter().step_by(face_size).enumerate() {
        for (x, ch) in row.iter().step_by(face_size).enumerate() {
            if *ch == ' ' {
                continue;
            }

            face_coords.push((x / face_size, y / face_size));
        }
    }

    let face = Face {
        identity: Front,
        grid: get_grid_range(&(0, 0), face_size, grid),
    };
    faces.insert(Front, face);

    let mut visited = HashSet::new();
    let mut to_visit = HashSet::new();

    visited.insert(&face_coords[0]);
    for coord in face_coords.iter().skip(1) {
        to_visit.insert(coord);
    }

    while !to_visit.is_empty() {
        for &candidate in to_visit.iter() {
            
        }
    }

    Cube { faces }
}

fn are_neighbours(pos: &(usize, usize), pos2: &(usize, usize)) -> bool {
    let delta_x = pos.0.max(pos2.0) - pos.0.min(pos2.0);
    let delta_y = pos.1.max(pos2.1) - pos.1.min(pos2.1);

    delta_x + delta_y == 1
}

fn get_grid_range(pos: &(usize, usize), face_size: usize, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::with_capacity(face_size);

    for y in pos.1 * face_size..(pos.1 + 1) * face_size {
        let mut row = Vec::with_capacity(face_size);
        for x in pos.0 * face_size..(pos.0 + 1) * face_size {
            row.push(grid[y][x]);
        }
        res.push(row);
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::part2::parse::parse_grid;

    use super::*;

    #[test]
    fn get_grid_range_works() {
        let input = include_str!("./../../data/demo.txt");
        let mut iter = input.split("\n\n");
        let grid_lines = iter.next().unwrap();
        let (_, grid) = parse_grid(grid_lines);
        let sub_grid = get_grid_range(&(1, 1), 4, &grid);
        let expected = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
        ];

        assert_eq!(sub_grid, expected);
    }

    #[test]
    fn are_neighbours_works() {
        assert_eq!(are_neighbours(&(0,1), &(1,0)), false);
        assert_eq!(are_neighbours(&(0,2), &(2,0)), false);
        assert_eq!(are_neighbours(&(0,1), &(0,3)), false);
        assert_eq!(are_neighbours(&(0,0), &(0,0)), false);
        assert_eq!(are_neighbours(&(1,1), &(1,0)), true);
        assert_eq!(are_neighbours(&(2,1), &(2,0)), true);
        assert_eq!(are_neighbours(&(1,2), &(1,3)), true);
        assert_eq!(are_neighbours(&(4,5), &(5,5)), true);
    }
}

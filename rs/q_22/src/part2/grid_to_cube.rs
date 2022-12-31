use std::collections::{HashMap, HashSet};

use super::cube::{FaceIdentity::*, AxisDirection};
use super::cube::{Cube, Face, FaceIdentity};
use super::cube::AxisDirection::*;

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

    let mut visited = HashMap::new();
    let mut to_visit = HashSet::new();

    visited.insert(&face_coords[0], (Front, 0usize));
    for coord in face_coords.iter().skip(1) {
        to_visit.insert(coord);
    }

    while !to_visit.is_empty() {
        let mut found = false;

        for &candidate in to_visit.iter() {
            if found {
                break;
            }

            for (visited_coord, (ident, rots)) in visited.iter() {
                if are_neighbours(candidate, visited_coord) {
                    let new_visited = (Back, 0);
                    visited.insert(candidate, new_visited);
                    found = true;
                    break;
                }
            }
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

// assumes pos and pos2 are neighbours
fn determine_direction(pos: &(usize, usize), pos2: &(usize, usize)) -> Option<AxisDirection> {
    if pos2.0 > pos.0 {
        return Some(XPositive);
    }

    if pos2.0 < pos.0 {
        return Some(XNegative);
    }

    if pos2.1 > pos.1 {
        return Some(YPositive);
    }

    if pos2.1 < pos.1 {
        return Some(YNegative);
    }

    None
}

fn rotate_direction(dir: &AxisDirection, rots: usize) -> AxisDirection {
    let adjusted_rots = rots % 4;

    let dirs = [YNegative, XPositive, YPositive, XNegative];
    let dir_idx = dirs.iter().position(|curr| *curr == *dir).unwrap();
    let dir_idx = (dir_idx + adjusted_rots) % 4;

    dirs[dir_idx].clone()
}

// fn rotate_range_once(range: &Vec<Vec<char>>) -> Vec<Vec<char>> {

// }

// fn rotate_range(range: &Vec<Vec<char>>, rots: usize) -> Vec<Vec<char>> {
//     let mut temp_range = range.clone();
// }

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

    #[test]
    fn determine_direction_works() {
        assert_eq!(determine_direction(&(1,1), &(2,1)).unwrap(), XPositive);
        assert_eq!(determine_direction(&(1,1), &(0,1)).unwrap(), XNegative);
        assert_eq!(determine_direction(&(1,1), &(1,2)).unwrap(), YPositive);
        assert_eq!(determine_direction(&(1,1), &(1,0)).unwrap(), YNegative);
        assert!(determine_direction(&(1,1), &(1,1)).is_none());
    }

    #[test]
    fn rotate_direction_works() {
        assert_eq!(rotate_direction(&YNegative, 0), YNegative);
        assert_eq!(rotate_direction(&YNegative, 1), XPositive);
        assert_eq!(rotate_direction(&YNegative, 2), YPositive);
        assert_eq!(rotate_direction(&YNegative, 3), XNegative);
        assert_eq!(rotate_direction(&YNegative, 4), YNegative);
    }
}

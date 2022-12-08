use std::{fs::read_to_string, path::Path};

type Trees = Vec<Vec<u8>>;

fn is_visible(i: usize, j: usize, trees: &Trees) -> bool {
    let tree = trees[i][j];

    let mut left_visible = true;
    for x in 0..j {
        let current = trees[i][x];
        if current >= tree {
            left_visible = false;
            break;
        }
    }

    let mut right_visible = true;
    for x in j+1..trees[i].len() {
        let current = trees[i][x];
        if current >= tree {
            right_visible = false;
            break;
        }
    }

    let mut top_visible = true;
    for y in 0..i {
        let current = trees[y][j];
        if current >= tree {
            top_visible = false;
            break;
        }
    }

    let mut bottom_visible = true;
    for y in i+1..trees.len() {
        let current = trees[y][j];
        if current >= tree {
            bottom_visible = false;
            break;
        }
    }

    left_visible || right_visible || top_visible || bottom_visible
}

fn scenic_view(i: usize, j: usize, trees: &Trees) -> u32 {
    let tree = trees[i][j];

    let mut left_visible = 0;
    for x in (0..j).rev() {
        let current = trees[i][x];
        if current >= tree {
            left_visible += 1;
            break;
        }

        left_visible += 1;
    }

    let mut right_visible = 0;
    for x in j+1..trees[i].len() {
        let current = trees[i][x];
        if current >= tree {
            right_visible += 1;
            break;
        }

        right_visible += 1;
    }

    let mut top_visible = 0;
    for y in (0..i).rev() {
        let current = trees[y][j];
        if current >= tree {
            top_visible += 1;
            break;
        }

        top_visible += 1;
    }

    let mut bottom_visible = 0;
    for y in i+1..trees.len() {
        let current = trees[y][j];
        if current >= tree {
            bottom_visible += 1;
            break;
        }

        bottom_visible += 1;
    }

    left_visible * right_visible * top_visible * bottom_visible
}

fn part1(trees: &Trees) -> u32 {
    let mut visible = 0;

    for (i, row) in trees.iter().enumerate() {
        let is_outer_row = i == 0 || i == trees.len() - 1;
        for (j, _) in row.iter().enumerate() {
            let is_outer_col = j == 0 || j == row.len() - 1;
            if is_outer_row || is_outer_col || is_visible(i, j, trees) {
                visible += 1;
            }
        }
    }

    visible
}

fn part2(trees: &Trees) -> u32 {
    let mut max = 0;

    for (i, row) in trees.iter().enumerate() {
        if i == 0 || i == trees.len() - 1 {
            continue;
        }

        for (j, _) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                continue;
            }

            let current = scenic_view(i, j, trees);
            if current > max {
                max = current;
            }
        }
    }

    max
}

fn parse(path: &str) -> Trees {
    read_to_string(&Path::new(path))
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Trees>()
}

fn main() {
    let trees = parse("./data/data.txt");
    let part1_result = part1(&trees);
    println!("part 1: {}", part1_result);
    let part2_result = part2(&trees);
    println!("part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let trees = parse("./data/demo.txt");
        let result = part1(&trees);
        assert_eq!(21, result);
    }

    #[test]
    fn scenic_view_works() {
        let trees = parse("./data/demo.txt");
        let result = scenic_view(3, 2, &trees);
        assert_eq!(8, result);
    }

    #[test]
    fn part2_works() {
        let trees = parse("./data/demo.txt");
        let result = part2(&trees);
        assert_eq!(8, result);
    }
}

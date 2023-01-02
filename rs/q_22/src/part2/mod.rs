use crate::part2::parse::{parse_grid, print_grid, pad_grid};

mod cube;
mod grid_to_cube;
mod instruction;
mod parse;
mod transition;

pub fn part2(input: &str) {
    let mut iter = input.split("\n\n");
    let grid_lines = iter.next().unwrap();
    // let mut instructions = iter.next().unwrap();
    let (max_len, mut grid) = parse_grid(grid_lines);
    pad_grid(max_len, &mut grid);
    let grid_str = print_grid(&grid);
    println!("{}", grid_str);
}

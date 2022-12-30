use std::fs::read_to_string;

use crate::board::{parse_instructions, Board};

pub fn part1() {
    let content = read_to_string("./data/input.txt").unwrap();
    let mut content_iter = content.split("\n\n");
    let board = content_iter.next().unwrap();
    let instructions = content_iter.next().unwrap();

    let mut board = Board::from_str(board);
    let instructions = parse_instructions(instructions);

    for instruction in instructions {
        board.do_instruction(&instruction);
    }

    let final_pos = board.get_position();
    let dir_mod = match board.get_orientation() {
        crate::board::Orientation::Right => 0,
        crate::board::Orientation::Bottom => 1,
        crate::board::Orientation::Left => 2,
        crate::board::Orientation::Top => 3,
    };

    let res = 1000 * (final_pos.0 + 1) + 4 * (final_pos.1 + 1) + dir_mod;
    println!("{:#?}", res);
}

use std::{collections::BTreeMap, fs::read_to_string};

use Direction::*;
use Instruction::*;

#[derive(Debug)]
struct Board {
    tiles: BTreeMap<(usize, usize), Tile>,
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
    Void,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug)]
enum Direction {
    Counterclockwise,
    Clockwise,
}

impl Board {
    pub fn from_str(input: &str) -> Self {
        let mut tiles = BTreeMap::new();
        input.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, ch)| {
                tiles.insert((i, j), Tile::from_char(ch));
            })
        });

        Self { tiles }
    }
}

impl Tile {
    pub fn from_char(ch: char) -> Self {
        match ch {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("Invalid tile char."),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut number_buffer = String::new();

    let mut iter = input.chars().peekable();
    while let Some(ch) = iter.next() {
        if ch.is_ascii_digit() {
            number_buffer.push(ch);
            if let Some(next_ch) = iter.peek() {
                if !next_ch.is_ascii_digit() {
                    let number = number_buffer.parse::<usize>().unwrap();
                    instructions.push(Move(number));
                }
            }
        } else if ch == 'R' {
            instructions.push(Turn(Clockwise));
        } else {
            instructions.push(Turn(Counterclockwise));
        }
    }

    instructions
}

pub fn part1() {
    let content = read_to_string("./data/demo.txt").unwrap();
    let mut content_iter = content.split("\n\n");
    let board = content_iter.next().unwrap();
    let instructions = content_iter.next().unwrap();

    let board = Board::from_str(board);
    let instructions = parse_instructions(instructions);

    println!("{:#?}", board);
    println!("{:#?}", instructions);
}

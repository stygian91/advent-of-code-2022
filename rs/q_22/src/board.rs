use std::{collections::BTreeMap, ops::RangeInclusive};

use Direction::*;
use Instruction::*;
use Orientation::*;
use Tile::*;

// (y, x)
type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Right,
    Bottom,
    Left,
    Top,
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Open,
    Wall,
    Void,
}

#[derive(Debug)]
pub enum Instruction {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug)]
pub enum Direction {
    Counterclockwise,
    Clockwise,
}

impl Tile {
    pub fn from_char(ch: char) -> Self {
        match ch {
            ' ' => Void,
            '.' => Open,
            '#' => Wall,
            _ => panic!("Invalid tile char."),
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
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
                    number_buffer = String::new();
                }
            } else {
                let number = number_buffer.parse::<usize>().unwrap();
                instructions.push(Move(number));
                number_buffer = String::new();
            }
        } else if ch == 'R' {
            instructions.push(Turn(Clockwise));
        } else {
            instructions.push(Turn(Counterclockwise));
        }
    }

    instructions
}

#[derive(Debug)]
pub struct Board {
    tiles: BTreeMap<Coord, Tile>,
    start: (usize, usize),
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    position: Coord,
    orientation: Orientation,
}

impl Board {
    pub fn from_str(input: &str) -> Self {
        let mut res = Self {
            tiles: BTreeMap::new(),
            x_range: 0..=usize::MAX,
            y_range: 0..=usize::MAX,
            start: (0, 0),
            position: (0, 0),
            orientation: Right,
        };

        let mut max_x = 0;
        let mut max_y = 0;
        let mut start = false;

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                let tile = Tile::from_char(ch);
                match tile {
                    Open | Wall => {
                        if !start && y == 0 {
                            start = true;
                            res.start = (y, x);
                            res.position = (y, x);
                        }
                    }
                    Void => (),
                }

                if y > max_y {
                    max_y = y;
                }
                if x > max_x {
                    max_x = x;
                }

                if let Void = &tile {
                    return;
                }

                res.tiles.insert((y, x), tile);
            })
        });

        res.x_range = 0..=max_x;
        res.y_range = 0..=max_y;
        res
    }

    pub fn walk(&mut self, steps: usize) {
        let iter = BoardIterator::new(self, steps).enumerate();
        let mut current_pos = self.position;

        for (_, (pos, tile)) in iter {
            match tile {
                Open => {
                    current_pos = pos;
                }
                Wall => break,
                Void => unreachable!("Tile should not be void."),
            }
        }

        self.position = current_pos;
    }

    pub fn do_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Move(steps) => self.walk(*steps),
            Turn(direction) => {
                self.orientation = match direction {
                    Counterclockwise => match self.orientation {
                        Right => Top,
                        Bottom => Right,
                        Left => Bottom,
                        Top => Left,
                    },
                    Clockwise => match self.orientation {
                        Right => Bottom,
                        Bottom => Left,
                        Left => Top,
                        Top => Right,
                    },
                };
            }
        }
    }

    pub fn get(&self, pos: &Coord) -> Tile {
        self.tiles
            .get(pos)
            .map(|t| *t)
            .or_else(|| Some(Tile::Void))
            .unwrap()
    }

    pub fn first_non_void(
        &self,
        row_or_col: usize,
        backward: bool,
        horizontal: bool,
    ) -> Option<Coord> {
        let iter: Box<dyn Iterator<Item = usize>> = match backward {
            true => Box::new(self.x_range.clone().rev()),
            false => Box::new(self.x_range.clone()),
        };

        for i in iter {
            let pos = match horizontal {
                true => (row_or_col, i),
                false => (i, row_or_col),
            };

            match self.get(&pos) {
                Open | Wall => return Some(pos),
                Void => (),
            }
        }

        None
    }

    pub fn get_position(&self) -> Coord {
        self.position
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    range_iter: Box<dyn ExactSizeIterator<Item = isize>>,
    current: usize,
}

impl<'a> BoardIterator<'a> {
    pub fn new(board: &'a Board, steps: usize) -> Self {
        Self {
            board,
            range_iter: Self::new_range(&board.position, &board.orientation, steps),
            current: 0,
        }
    }

    pub fn new_range(
        position: &Coord,
        orientation: &Orientation,
        steps: usize,
    ) -> Box<dyn ExactSizeIterator<Item = isize>> {
        let pos_isize = (position.0 as isize, position.1 as isize);
        let steps_isize = steps as isize;
        let range_iter: Box<dyn ExactSizeIterator<Item = isize>> = match orientation {
            Right => Box::new(pos_isize.1 + 1..pos_isize.1 + steps_isize + 1),
            Bottom => Box::new(pos_isize.0 + 1..pos_isize.0 + steps_isize + 1),
            Left => Box::new((pos_isize.1 - steps_isize..pos_isize.1).rev()),
            Top => Box::new((pos_isize.0 - steps_isize..pos_isize.0).rev()),
        };

        range_iter
    }

    fn handle_void<F>(&mut self, cb: F) -> Option<((usize, usize), Tile)>
    where
        F: Fn() -> (usize, bool, bool),
    {
        let args = cb();
        let new_pos = self.board.first_non_void(args.0, args.1, args.2).unwrap();
        if self.range_iter.len() == 0 {
            return Some((new_pos, self.board.get(&new_pos)));
        }

        self.range_iter = Self::new_range(
            &new_pos,
            &self.board.orientation,
            self.range_iter.len() - (self.current - 1) + 1,
        );
        self.current = 0;

        Some((new_pos, self.board.get(&new_pos)))
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = ((usize, usize), Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.range_iter.next();
        if let None = next {
            return None;
        }
        let i = next.unwrap();
        self.current += 1;

        let pos = match self.board.orientation {
            Right | Left => (self.board.position.0 as isize, i),
            Bottom | Top => (i, self.board.position.1 as isize),
        };

        if pos.0 < 0 {
            return self.handle_void(|| (pos.1 as usize, true, false));
        } else if pos.1 < 0 {
            return self.handle_void(|| (pos.0 as usize, true, true));
        }

        let pos_usize = (pos.0 as usize, pos.1 as usize);
        let tmp = self.board.get(&pos_usize);
        match tmp {
            Open | Wall => {
                return Some((pos_usize, tmp));
            }
            Void => (),
        }

        match self.board.orientation {
            Right => self.handle_void(|| (pos_usize.0, false, true)),
            Bottom => self.handle_void(|| (pos_usize.1, false, false)),
            Left => self.handle_void(|| (pos_usize.0, true, true)),
            Top => self.handle_void(|| (pos_usize.1, true, false)),
        }
    }
}

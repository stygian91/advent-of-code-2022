#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Counterclockwise,
    Clockwise,
}

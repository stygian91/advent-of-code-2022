use super::instruction::*;
use super::instruction::Direction::*;
use super::instruction::Instruction::*;

pub fn parse_grid(input: &str) -> (usize, Vec<Vec<char>>) {
    let mut res = vec![];
    let mut max_len = 0;

    for line in input.lines() {
        if line.len() > max_len {
            max_len = line.len();
        }
        let mut row = vec![];

        for ch in line.chars() {
            row.push(ch);
        }

        res.push(row);
    }

    (max_len, res)
}

pub fn pad_grid(max_len: usize, grid: &mut Vec<Vec<char>>) {
    for row in grid.iter_mut() {
        while row.len() < max_len {
            row.push(' ');
        }
    }
}

pub fn print_grid(grid: &Vec<Vec<char>>) -> String {
    let mut res = String::new();

    for row in grid.iter() {
        for ch in row.iter() {
            res.push(*ch);
        }

        res.push('\n');
    }

    res.pop();

    res
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let expected = include_str!("./../../data/test/demo-expected.txt");
        let input = include_str!("./../../data/demo.txt");
        let mut iter = input.split("\n\n");
        let grid_lines = iter.next().unwrap();
        let (max_len, mut grid) = parse_grid(grid_lines);
        pad_grid(max_len, &mut grid);
        let grid_str = print_grid(&grid);
        assert_eq!(grid_str, expected);
    }

    #[test]
    fn parse_instructions_works() {
        let input = include_str!("./../../data/demo.txt");
        let mut iter = input.split("\n\n");
        let instruction_input = iter.nth(1).unwrap();
        let instructions = parse_instructions(instruction_input);
        let expected = vec![
            Move(10),
            Turn(Clockwise),
            Move(5),
            Turn(Counterclockwise),
            Move(5),
            Turn(Clockwise),
            Move(10),
            Turn(Counterclockwise),
            Move(4),
            Turn(Clockwise),
            Move(5),
            Turn(Counterclockwise),
            Move(5),
        ];
        assert_eq!(instructions, expected);
    }
}

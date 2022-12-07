use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn from_str(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let get_capture = |i| captures.get(i).unwrap().as_str().parse::<usize>().unwrap();

        Self {
            amount: get_capture(1),
            from: get_capture(2) - 1,
            to: get_capture(3) - 1,
        }
    }
}

fn parse_file(parts: &Vec<&str>) -> (Vec<Vec<char>>, Vec<Move>) {
    let stacks = parse_stacks(parts[0]);
    let moves = parse_moves(parts[1]);
    (stacks, moves)
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in input.lines() {
        moves.push(Move::from_str(line));
    }
    moves
}

fn init_until(n: usize, stacks: &mut Vec<Vec<char>>) {
    for _ in stacks.len()..=n {
        stacks.push(Vec::new());
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let mut iter = input.lines().peekable();

    while let Some(line) = iter.next() {
        if iter.peek().is_none() {
            continue;
        }

        for (stack_idx, ch_idx) in (1..line.len()).step_by(4).enumerate() {
            let ch = line.chars().nth(ch_idx).unwrap();
            if ch == ' ' {
                continue;
            }

            if stack_idx >= stacks.len() {
                init_until(stack_idx, &mut stacks);
            }
            stacks[stack_idx].push(ch);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn apply_move(stacks: &mut Vec<Vec<char>>, mv: &Move) {
    for _ in 0..mv.amount {
        let val = stacks[mv.from].pop().unwrap();
        stacks[mv.to].push(val);
    }
}

fn apply_move_v2(stacks: &mut Vec<Vec<char>>, mv: &Move) {
    let len = stacks[mv.from].len();
    let slice = stacks[mv.from].split_off(len - mv.amount);
    stacks[mv.to].extend(slice);
}

fn get_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut res = String::with_capacity(stacks.len());

    for stack in stacks.iter() {
        let last = stack.last().unwrap();
        res.push(*last);
    }

    res
}

fn process_moves<F>(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>, f: F)
where
    F: Fn(&mut Vec<Vec<char>>, &Move) -> (),
{
    for mv in moves {
        f(stacks, mv);
    }

    println!("{}", get_tops(&stacks));
}

fn part1(parts: &Vec<&str>) {
    let (mut stacks, moves) = parse_file(parts);
    process_moves(&mut stacks, &moves, apply_move);
}

fn part2(parts: &Vec<&str>) {
    let (mut stacks, moves) = parse_file(parts);
    process_moves(&mut stacks, &moves, apply_move_v2);
}

fn main() {
    let contents = read_to_string(&Path::new("./data/input.txt")).unwrap();
    let parts = contents.split("\n\n").collect::<Vec<_>>();

    part1(&parts);
    part2(&parts);
}

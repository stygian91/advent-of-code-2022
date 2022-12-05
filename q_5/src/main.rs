use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::Path};

#[derive(Debug)]
struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn from_str(input: &str) -> Self {
        lazy_static! {
            // move 5 from 8 to 2
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let captures = RE.captures(input).unwrap();
        let amount = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        Self { amount, from, to }
    }
}

fn parse_file(path: &str) -> (HashMap<usize, Vec<char>>, Vec<Move>) {
    let contents = read_to_string(&Path::new(path)).unwrap();
    let parts = contents.split("\n\n").collect::<Vec<_>>();
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

fn parse_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::new();
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

            let stack = stacks.entry(stack_idx).or_insert(Vec::new());
            stack.push(ch);
        }
    }

    for (i, stack) in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn apply_move(stacks: &mut HashMap<usize, Vec<char>>, mv: &Move) {
    for _ in 0..mv.amount {
        let val;
        {
            val = stacks.get_mut(&mv.from).unwrap().pop().unwrap();
        }
        stacks.get_mut(&mv.to).unwrap().push(val);
    }
}

fn apply_move_v2(stacks: &mut HashMap<usize, Vec<char>>, mv: &Move) {
    let val;
    {
        let from = stacks.get_mut(&mv.from).unwrap();
        val = from.split_off(from.len() - mv.amount)
    }
    stacks.get_mut(&mv.to).unwrap().extend(val);
}

fn get_tops(stacks: &HashMap<usize, Vec<char>>) -> String {
    let mut res = String::new();
    for i in 0..stacks.len() {
        let stack = stacks.get(&i).unwrap();
        let last = stack.last().unwrap();
        res.push(*last);
    }
    res
}

#[allow(unused)]
fn part1() {
    let (mut stacks, moves) = parse_file("./data/input.txt");
    for mv in moves {
        apply_move(&mut stacks, &mv);
    }

    println!("{}", get_tops(&stacks));
}

fn part2() {
    let (mut stacks, moves) = parse_file("./data/input.txt");
    for mv in moves {
        apply_move_v2(&mut stacks, &mv);
    }

    println!("{}", get_tops(&stacks));
}

fn main() {
    part2();
}

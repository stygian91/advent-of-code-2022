use core::panic;

use crate::{parse_opponent, GameResult, Hand, calculate_score};

#[derive(Debug)]
struct Round {
    opponent: Hand,
    result: GameResult,
}

fn parse_result(input: &str) -> GameResult {
    match input {
        "X" => GameResult::Loss,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Invalid game result"),
    }
}

impl Round {
    pub fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        Self {
            opponent: parse_opponent(parts[0]),
            result: parse_result(parts[1]),
        }
    }

    pub fn get_player(&self) -> Hand {
        match self.result {
            GameResult::Win => self.opponent.get_corresponding_hand(&GameResult::Loss),
            GameResult::Loss => self.opponent.get_corresponding_hand(&GameResult::Win),
            GameResult::Draw => self.opponent.get_corresponding_hand(&GameResult::Draw),
        }
    }

    pub fn get_points(&self) -> u32 {
        calculate_score(&self.get_player(), &self.result)
    }
}

pub fn part2(input: &str) {
    let points: u32 = input.lines()
        .map(|line| Round::from_str(line).get_points())
        .sum();

    println!("{}", points);
}

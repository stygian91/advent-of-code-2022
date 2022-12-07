use std::{path::Path, fs::read_to_string};

use part1::part1;
use part2::part2;

mod part1;
mod part2;

#[derive(Debug, Clone, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn get_corresponding_hand(&self, result: &GameResult) -> Hand {
        match result {
            GameResult::Loss => match self {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            GameResult::Draw => self.clone(),
            GameResult::Win => match self {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Draw,
    Loss,
}

pub fn parse_opponent(input: &str) -> Hand {
    match input {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Invalid opponent hand"),
    }
}

pub fn calculate_score(player: &Hand, result: &GameResult) -> u32 {
    let hand_score: u32 = match player {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };
    let result_score: u32 = match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
    };

    hand_score + result_score
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = read_to_string(&path).unwrap();

    part1(&input);
    part2(&input);
}

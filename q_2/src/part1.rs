use crate::{Hand, GameResult, parse_opponent, calculate_score};

#[derive(Debug)]
struct Round {
    opponent: Hand,
    player: Hand,
}

impl Round {
    pub fn from_str(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<&str>>();
        let opponent = parse_opponent(parts[0]);
        let player = match parts[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Invalid player hand"),
        };
        Round { opponent, player }
    }

    fn get_result(&self) -> GameResult {
        let winning_hand = self.player.get_corresponding_hand(&GameResult::Win);
        let losing_hand = self.player.get_corresponding_hand(&GameResult::Loss);
        if self.opponent == winning_hand {
            return GameResult::Win;
        }

        if self.opponent == losing_hand {
            return GameResult::Loss;
        }

        GameResult::Draw
    }

    fn get_points(&self) -> u32 {
        calculate_score(&self.player, &self.get_result())
    }
}

pub fn part1(input: &str) {
    let points: u32 = input.lines()
        .map(|line| Round::from_str(line).get_points())
        .sum();

    println!("{}", points);
}

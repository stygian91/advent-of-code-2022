use std::collections::VecDeque;

use Direction::*;

const STARTING_DIRECTIONS: [Direction; 4] = [N, S, E, W];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

pub fn rotate_proposed(proposed: &mut VecDeque<Direction>) {
    let curr = proposed.pop_front().unwrap();
    proposed.push_back(curr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_works() {
        let mut proposed_directions = VecDeque::from(STARTING_DIRECTIONS);
        rotate_proposed(&mut proposed_directions);
        assert_eq!(proposed_directions, VecDeque::from([S, E, W, N]));
    }
}
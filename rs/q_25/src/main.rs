use std::fs::read_to_string;

use convert::number_to_snafu;
use parse::parse_number;

mod parse;
mod convert;

fn sum_snafu(input: &str) -> isize {
    input.lines().map(|line| parse_number(line).unwrap()).sum()
}

fn part1(input: &str) -> String {
    let sum = sum_snafu(input);
    number_to_snafu(sum)
}

fn main() {
    let input = read_to_string("./data/input.txt").unwrap();
    let res1 = part1(&input);
    println!("Part 1: {}", res1);
}

#[cfg(test)]
mod tests {
    const DEMO: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    use super::*;

    #[test]
    fn sum_snafu_works() {
        assert_eq!(sum_snafu(DEMO), 4890);
    }

    #[test]
    fn part1_works() {
        let input = read_to_string("./data/demo.txt").unwrap();
        let res = part1(&input);
        assert_eq!(res, "2=-1=0");
    }
}

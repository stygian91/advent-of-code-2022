#![allow(unused)]

use std::{collections::VecDeque, fs::read_to_string, path::Path};

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,
    inspected: u32,
}

impl Monkey {
    pub fn from_str(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();

        Self {
            items: Self::parse_items(lines[1]),
            operation: Operation::from_str(lines[2]),
            test: Test::from_lines(&lines[3..=5]),
            inspected: 0,
        }
    }

    pub fn throw_all(&mut self, decrease_stress: bool) -> Vec<(usize, u128)> {
        let mut res = vec![];

        while let Some(item) = self.get_next() {
            let mut _item = self.operation.execute(&item);
            if decrease_stress {
                _item = _item / 3;
            }
            res.push((self.test.execute(&_item), _item));
            self.inspected += 1;
        }

        res
    }

    pub fn receive(&mut self, item: u128) {
        self.items.push_back(item);
    }

    fn get_next(&mut self) -> Option<u128> {
        self.items.pop_front()
    }

    fn parse_items(line: &str) -> VecDeque<u128> {
        let parts = line.split(": ").collect::<Vec<&str>>();
        parts[1]
            .split(", ")
            .map(|num| num.parse::<u128>().unwrap())
            .collect::<VecDeque<_>>()
    }
}

#[derive(Debug)]
struct Operation {
    // The left operand is always `old`
    right: Operand,
    operator: Operator,
}

impl Operation {
    pub fn from_str(line: &str) -> Self {
        let parts = line
            .split("old ")
            .nth(1)
            .map(|part| part.split(' '))
            .unwrap()
            .collect::<Vec<&str>>();

        let sign = parts[0].chars().nth(0).unwrap();
        let operator = match sign {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Invalid operator"),
        };

        let right = match parts[1] {
            "old" => Operand::Old,
            (val) => Operand::Literal(val.parse::<u128>().unwrap()),
        };

        Self { operator, right }
    }

    pub fn execute(&self, old: &u128) -> u128 {
        let right = match &self.right {
            Operand::Old => old,
            Operand::Literal(val) => &val,
        };

        match self.operator {
            Operator::Add => old + right,
            Operator::Multiply => old * right,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Literal(u128),
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Test {
    divident: u128,
    passes: usize,
    fails: usize,
}

impl Test {
    pub fn from_lines(lines: &[&str]) -> Self {
        let divident = lines[0]
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<u128>()
            .unwrap();

        Self {
            divident,
            passes: Self::parse_throw(lines[1]),
            fails: Self::parse_throw(lines[2]),
        }
    }

    pub fn execute(&self, value: &u128) -> usize {
        match value % &self.divident == 0 {
            true => self.passes,
            false => self.fails,
        }
    }

    fn parse_throw(line: &str) -> usize {
        line.split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }
}

fn round(monkeys: &mut Vec<Monkey>, decrease_stress: bool) {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].throw_all(decrease_stress);

        for throw in throws {
            monkeys[throw.0].receive(throw.1);
        }
    }
}

fn parse_monkeys(path: &str) -> Vec<Monkey> {
    let content = read_to_string(&Path::new(path)).unwrap();
    let line_groups = content.split("\n\n").collect::<Vec<&str>>();
    line_groups
        .iter()
        .map(|group| Monkey::from_str(group))
        .collect::<Vec<_>>()
}

fn part1(monkeys: &mut Vec<Monkey>) -> u32 {
    for i in 0..20 {
        round(monkeys, true);
    }

    let mut inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();

    inspected.sort_by(|a, b| b.cmp(a));

    inspected[0] * inspected[1]
}

fn part2(monkeys: &mut Vec<Monkey>) -> u32 {
    for i in 0..10_000 {
        round(monkeys, false);
    }

    let mut inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();

    inspected.sort_by(|a, b| b.cmp(a));

    inspected[0] * inspected[1]
}

fn main() {
    let mut monkeys = parse_monkeys("./data/input.txt");

    let part1_res = part1(&mut monkeys);
    println!("part 1: {}", part1_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_items(monkeys: &Vec<Monkey>, expected: &[Vec<u128>]) {
        let mut actual = vec![];

        for monkey in monkeys {
            actual.push(monkey.items.clone());
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn round_works() {
        let mut monkeys = parse_monkeys("./data/demo.txt");

        round(&mut monkeys, true);

        test_items(
            &monkeys,
            &[
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![],
            ],
        );
    }

    #[test]
    fn part1_works() {
        let mut monkeys = parse_monkeys("./data/demo.txt");

        let monkey_business = part1(&mut monkeys);
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let mut monkeys = parse_monkeys("./data/demo.txt");

        let monkey_business = part2(&mut monkeys);
        assert_eq!(monkey_business, 2713310158);
    }
}

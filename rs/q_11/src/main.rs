use std::{collections::VecDeque, fs::read_to_string, path::Path};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,
    inspected: u128,
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

    pub fn throw_all<F>(&mut self, modify: F) -> Vec<(usize, u128)>
    where
        F: Fn(u128) -> u128,
    {
        let mut res = vec![];

        while let Some(item) = self.get_next() {
            let mut _item = self.operation.execute(item);
            _item = modify(_item);
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

#[derive(Debug, Clone)]
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

        let sign = parts[0].chars().next().unwrap();
        let operator = match sign {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Invalid operator"),
        };

        let right = match parts[1] {
            "old" => Operand::Old,
            val => Operand::Literal(val.parse::<u128>().unwrap()),
        };

        Self { operator, right }
    }

    pub fn execute(&self, old: u128) -> u128 {
        let right = match self.right {
            Operand::Old => old,
            Operand::Literal(val) => val,
        };

        match self.operator {
            Operator::Add => old + right,
            Operator::Multiply => old * right,
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Literal(u128),
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
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
        match value % self.divident == 0 {
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

fn round<F>(monkeys: &mut Vec<Monkey>, modify: F) where F: Fn(u128) -> u128 {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].throw_all(&modify);

        for throw in throws {
            monkeys[throw.0].receive(throw.1);
        }
    }
}

fn parse_monkeys(path: &str) -> Vec<Monkey> {
    let content = read_to_string(Path::new(path)).unwrap();
    let line_groups = content.split("\n\n").collect::<Vec<&str>>();
    line_groups
        .iter()
        .map(|group| Monkey::from_str(group))
        .collect::<Vec<_>>()
}

fn part1_worry(worry: u128) -> u128 {
    worry / 3
}

fn part1(monkeys: &mut Vec<Monkey>) -> u128 {
    for _ in 0..20 {
        round(monkeys, part1_worry);
    }

    let mut inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();

    inspected.sort_by(|a, b| b.cmp(a));

    inspected[0] * inspected[1]
}

fn part2(monkeys: &mut Vec<Monkey>) -> u128 {
    let modulo = monkeys.iter().map(|monkey| monkey.test.divident).product::<u128>();

    for _ in 0..10_000 {
        round(monkeys, |item| item % modulo);
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

    let part1_res = part1(&mut monkeys.clone());
    println!("part 1: {}", part1_res);

    let part2_res = part2(&mut monkeys);
    println!("part 2: {}", part2_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut monkeys = parse_monkeys("./data/demo.txt");

        let monkey_business = part1(&mut monkeys);
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn part2_works() {
        let mut monkeys = parse_monkeys("./data/demo.txt");

        let monkey_business = part2(&mut monkeys);
        assert_eq!(monkey_business, 2713310158);
    }
}

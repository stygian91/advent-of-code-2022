use std::{collections::HashMap, fs::read_to_string};

use Operator::{Add, Div, Mul, Sub};

#[derive(Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    operator: Operator,
}

impl Operation {
    pub fn new(input: &str) -> Self {
        Self {
            lhs: input[0..4].to_owned(),
            rhs: input[7..].to_owned(),
            operator: Operator::new(input.chars().nth(5).unwrap()),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn new(ch: char) -> Self {
        match ch {
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Literal(u64),
    Operation(Operation),
}

fn eval(name: &str, map: &HashMap<String, Expression>) -> u64 {
    let value = map.get(name).unwrap();
    match value {
        Expression::Literal(v) => *v,
        Expression::Operation(operation) => match operation.operator {
            Add => eval(&operation.lhs, map) + eval(&operation.rhs, map),
            Sub => eval(&operation.lhs, map) - eval(&operation.rhs, map),
            Mul => eval(&operation.lhs, map) * eval(&operation.rhs, map),
            Div => eval(&operation.lhs, map) / eval(&operation.rhs, map),
        },
    }
}

fn parse(input: &str) -> HashMap<String, Expression> {
    let mut map = HashMap::new();

    input.lines().for_each(|line| {
        let mut iter = line.split(": ");
        let name = iter.next().unwrap();
        let value = iter.next().unwrap();
        let value = if let Ok(number) = value.parse::<u64>() {
            Expression::Literal(number)
        } else {
            Expression::Operation(Operation::new(value))
        };
        map.insert(name.to_owned(), value);
    });

    map
}

pub fn part1() {
    let input = read_to_string("./data/input.txt").unwrap();
    let map = parse(&input);

    println!("Part 1 result: {}", eval("root", &map));
}

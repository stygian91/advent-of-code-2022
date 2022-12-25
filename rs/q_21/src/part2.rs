// Original code by: JuniorBirdman1115 on reddit https://www.reddit.com/user/JuniorBirdman1115/
// From this comment: https://www.reddit.com/r/adventofcode/comments/zrav4h/comment/j1bymnm/
// Refactored to remove some duplication and fixed cargo clippy warnings
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Expression {
    Integer(u64),
    Plus(String, String),
    Minus(String, String),
    Times(String, String),
    Divide(String, String),
    Equals(String, String),
    Unknown,
}

use Expression::*;

pub type SymbolTable = HashMap<String, Expression>;

fn read_input(filename: &str) -> String {
    read_to_string(filename)
        .unwrap_or_else(|error| panic!("read_input(): error reading {}: {:?}", filename, error))
}

fn parse_input(contents: &str) -> SymbolTable {
    let int_re = Regex::new(r"([a-z]+): (\d+)").unwrap();
    let exp_re = Regex::new(r"([a-z]+): ([a-z]+) (\+|\-|\*|/) ([a-z]+)").unwrap();
    let mut symtab = SymbolTable::new();
    contents.lines().enumerate().for_each(|(i, line)| {
        if int_re.is_match(line) {
            let caps = int_re.captures(line).unwrap();
            let name = caps[1].to_string();
            let val = caps[2].parse::<u64>().unwrap();
            symtab.insert(name, Expression::Integer(val));
        } else if exp_re.is_match(line) {
            let caps = exp_re.captures(line).unwrap();
            let name = caps[1].to_string();
            let operand1 = caps[2].to_string();
            let operand2 = caps[4].to_string();
            let op = &caps[3];
            match op {
                "+" => symtab.insert(name, Plus(operand1, operand2)),
                "-" => symtab.insert(name, Minus(operand1, operand2)),
                "*" => symtab.insert(name, Times(operand1, operand2)),
                "/" => symtab.insert(name, Divide(operand1, operand2)),
                _ => unreachable!("Unreachable code in parse_input()"),
            };
        } else {
            panic!("parse_input(): parse error at line {}: '{}'", i + 1, line);
        }
    });

    symtab
}

fn modify(symtab: &mut SymbolTable) {
    let root = String::from("root");
    let root_exp = symtab.get(&root).unwrap();
    match root_exp {
        Plus(op1, op2)
        | Minus(op1, op2)
        | Times(op1, op2)
        | Divide(op1, op2) => {
            let new_exp = Equals(op1.clone(), op2.clone());
            symtab.insert(root, new_exp);
        }
        _ => {
            panic!("modify(): bug - expected +, -, *, or / in root node!");
        }
    }
    symtab.insert("humn".to_owned(), Unknown);
}

fn eval(name: &str, symtab: &SymbolTable) -> u64 {
    let exp = symtab.get(name).unwrap();
    match exp {
        Expression::Integer(n) => *n,
        Expression::Plus(op1, op2) => eval(op1, symtab) + eval(op2, symtab),
        Expression::Minus(op1, op2) => eval(op1, symtab) - eval(op2, symtab),
        Expression::Times(op1, op2) => eval(op1, symtab) * eval(op2, symtab),
        Expression::Divide(op1, op2) => eval(op1, symtab) / eval(op2, symtab),
        _ => 0,
    }
}

fn can_eval(name: &str, symtab: &SymbolTable) -> bool {
    let exp = symtab.get(name).unwrap();
    match exp {
        Expression::Integer(_) => true,
        Expression::Unknown => false,
        Expression::Plus(op1, op2)
        | Expression::Minus(op1, op2)
        | Expression::Times(op1, op2)
        | Expression::Divide(op1, op2) => can_eval(op1, symtab) & can_eval(op2, symtab),
        Expression::Equals(_, _) => unreachable!("Unreachable in can_eval()"),
    }
}

fn solve_exp<F, G>(op1: &str, op2: &str, val: u64, symtab: &SymbolTable, l_cb: &F, r_cb: &G) -> u64
where
    F: Fn(u64, u64) -> u64,
    G: Fn(u64, u64) -> u64,
{
    let eval_left = can_eval(op1, symtab);
    let eval_right = can_eval(op2, symtab);
    if eval_left {
        let n = eval(op1, symtab);
        let new_val = l_cb(val, n);
        recursive_solve(op2, new_val, symtab)
    } else if eval_right {
        let n = eval(op2, symtab);
        let new_val = r_cb(val, n);
        recursive_solve(op1, new_val, symtab)
    } else {
        panic!("recursive_solve(): bug - neither lhs and rhs can eval!");
    }
}

fn recursive_solve(name: &str, val: u64, symtab: &SymbolTable) -> u64 {
    let exp = symtab.get(name).unwrap();

    match exp {
        Integer(n) => *n,
        Plus(op1, op2) => solve_exp(op1, op2, val, symtab, &|a, b| a - b, &|a, b| a - b),
        Minus(op1, op2) => solve_exp(op1, op2, val, symtab, &|a, b| b - a, &|a, b| a + b),
        Times(op1, op2) => solve_exp(op1, op2, val, symtab, &|a, b| a / b, &|a, b| a / b),
        Divide(op1, op2) => solve_exp(op1, op2, val, symtab, &|a, b| b / a, &|a, b| a * b),
        Unknown => val,
        Equals(_, _) => panic!("recursive_solve(): bug - equals exp found in subtree!"),
    }
}

fn solve(name: &str, symtab: &SymbolTable) -> u64 {
    let exp = symtab.get(name).unwrap();
    if let Expression::Equals(op1, op2) = exp {
        let eval_left = can_eval(op1, symtab);
        let eval_right = can_eval(op2, symtab);
        if eval_left {
            let val = eval(op1, symtab);
            recursive_solve(op2, val, symtab)
        } else if eval_right {
            let val = eval(op2, symtab);
            recursive_solve(op1, val, symtab)
        } else {
            panic!("solve(): bug - neither lhs nor rhs can eval!")
        }
    } else {
        panic!("solve(): expected 'equals' expression for {}!", name);
    }
}

pub fn part2() {
    let contents = read_input("./data/input.txt");
    let mut symtab = parse_input(&contents);
    modify(&mut symtab);
    let result = solve("root", &symtab);
    println!("Part 2 result: {}", result);
}

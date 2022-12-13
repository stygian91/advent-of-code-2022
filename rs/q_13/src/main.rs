use std::{cmp::Ordering, fs::read_to_string};

use serde_json::{from_str, Number, Value};

fn parse(path: &str) -> Vec<Vec<Value>> {
    let content = read_to_string(path).unwrap();
    let pairs = content
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n')
                .map(|line| from_str::<Value>(line).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    return pairs;
}

fn compare(a: &Value, b: &Value) -> Ordering {
    if let Value::Number(a) = a {
        if let Value::Number(b) = b {
            return compare_nums(a, b);
        }
    }

    let a_list = match a {
        Value::Number(_) => vec![a.clone()],
        Value::Array(list) => list.clone(),
        _ => panic!("Invalid type"),
    };

    let b_list = match b {
        Value::Number(_) => vec![b.clone()],
        Value::Array(list) => list.clone(),
        _ => panic!("Invalid type"),
    };

    compare_lists(&a_list, &b_list)
}

fn compare_lists(a: &Vec<Value>, b: &Vec<Value>) -> Ordering {
    let max_len = a.len().max(b.len());

    for i in 0..max_len {
        let a_el = a.get(i);
        let b_el = b.get(i);

        if a_el.is_none() {
            return Ordering::Less;
        }

        if b_el.is_none() {
            return Ordering::Greater;
        }

        let a_el = a_el.unwrap();
        let b_el = b_el.unwrap();
        let cmp = compare(a_el, b_el);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    Ordering::Equal
}

fn compare_nums(a: &Number, b: &Number) -> Ordering {
    let a_num = a.as_u64().unwrap();
    let b_num = b.as_u64().unwrap();

    return a_num.cmp(&b_num);
}

fn part1(path: &str) -> usize {
    let pairs = parse(path);
    let mut sum = 0;

    for (i, pair) in pairs.iter().enumerate() {
        let cmp = compare(&pair[0], &pair[1]);
        if cmp == Ordering::Equal || cmp == Ordering::Less {
            sum += i + 1;
        }
    }

    sum
}

fn part2(path: &str) -> usize {
    let pairs = parse(path);
    let mut sorted = vec![];

    for pair in pairs.iter() {
        sorted.push(pair[0].clone());
        sorted.push(pair[1].clone());
    }

    let div1 = Value::Array(vec![Value::Array(vec![Value::from(2)])]);
    let div2 = Value::Array(vec![Value::Array(vec![Value::from(6)])]);

    sorted.push(div1.clone());
    sorted.push(div2.clone());

    sorted.sort_by(compare);

    let mut div1_idx = 0;
    let mut div2_idx = 0;

    for (i, el) in sorted.iter().enumerate() {
        let div1_cmp = compare(el, &div1);
        let div2_cmp = compare(el, &div2);

        if div1_cmp == Ordering::Equal {
            div1_idx = i;
        }

        if div2_cmp == Ordering::Equal {
            div2_idx = i;
        }
    }

    (div1_idx + 1) * (div2_idx + 1)
}

fn main() {
    let part1_res = part1("./data/input.txt");
    println!("part 1: {}", part1_res);

    let part2_res = part2("./data/input.txt");
    println!("part 2: {}", part2_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let part1_res = part1("./data/demo.txt");
        assert_eq!(part1_res, 13);
    }

    #[test]
    fn part2_works() {
        let part2_res = part2("./data/demo.txt");
        assert_eq!(part2_res, 140);
    }
}

use std::{fs::read_to_string, path::Path};

fn get_item_priority(letter: char) -> u8 {
    if letter.is_ascii_lowercase() {
        return letter as u8 - 'a' as u8 + 1;
    }

    if letter.is_ascii_uppercase() {
        return letter as u8 - 'A' as u8 + 27;
    }

    panic!("Invalid character");
}

fn find_common_letter(str1: &str, str2: &str) -> Option<char> {
    for ch in str1.chars() {
        if str2.contains(ch) {
            return Some(ch);
        }
    }

    None
}

fn part1(lines: &Vec<&str>) {
    let sum: u32 = lines
        .iter()
        .map(|line| {
            if line.len() % 2 == 1 {
                panic!("Number of items not divisible by 2");
            }

            let half = line.len() / 2;
            (&line[0..half], &line[half..])
        })
        .map(|parts| find_common_letter(parts.0, parts.1).unwrap())
        .map(|ch| get_item_priority(ch) as u32)
        .sum();

    println!("{:#?}", sum);
}

fn find_common_letter_in_group(group: &[&str]) -> Option<char> {
    for ch in group[0].chars() {
        if group[1].contains(ch) && group[2].contains(ch) {
            return Some(ch);
        }
    }

    None
}

fn part2(lines: &Vec<&str>) {
    let sum: u32 = lines
        .chunks(3)
        .map(|chunk| find_common_letter_in_group(chunk).unwrap())
        .map(|ch| get_item_priority(ch) as u32)
        .sum();

    println!("{:#?}", sum);
}

fn main() {
    let path = Path::new("./data/input.txt");
    let contents = read_to_string(&path).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    part1(&lines);
    part2(&lines);
}

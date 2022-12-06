use std::{fs::read_to_string, path::Path};

fn has_duplicates(slice: &[char]) -> bool {
    let mut v = Vec::from(slice);
    let len = v.len();

    v.sort();
    v.dedup();

    v.len() < len
}

fn get_first_unique_window(n: usize, input: &String) -> Option<usize> {
    let chars = input.chars().collect::<Vec<char>>();

    for (i, window) in chars.windows(n).enumerate() {
        if !has_duplicates(window) {
            return Some(i + n);
        }
    }

    None
}

fn main() {
    let contents = read_to_string(&Path::new("./data/input.txt")).unwrap();
    let start_of_packet = get_first_unique_window(4, &contents).unwrap();
    let start_of_message = get_first_unique_window(14, &contents).unwrap();
    println!("part 1: {:#?}", start_of_packet);
    println!("part 2: {:#?}", start_of_message);
}

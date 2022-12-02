use std::{fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("./data/input.txt");
    let input = read_to_string(&path).unwrap();
    let mut calories = input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    // descending sort
    calories.sort_by(|a, b| b.cmp(a));

    // part 1
    println!("{:#?}", calories[0]);
    // part 2
    println!("{:#?}", calories[0] + calories[1] + calories[2]);
}

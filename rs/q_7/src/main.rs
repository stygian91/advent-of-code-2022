#![allow(unused)]

mod common;
mod part1;

use std::{rc::Rc, cell::RefCell, fs::read_to_string, path::Path};

use common::*;
use part1::part1;

fn main() {
    let contents = read_to_string(&Path::new("./data/input.txt")).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut root = File::new_rc(File::new("/", true));
    parse(root.clone(), 1, &lines);
    update_dir_sizes(root.clone());
    let part1_res = part1(root.clone());
    println!("{:#?}", part1_res);
}

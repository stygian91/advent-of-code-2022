#![allow(unused)]

mod common;
mod part1;

use std::{rc::Rc, cell::RefCell, fs::read_to_string, path::Path};

use common::*;

fn main() {
    let contents = read_to_string(&Path::new("./data/demo.txt")).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut root = File::new_rc(File::new("/", true));
    parse(root.clone(), 1, &lines);
    let file_tree = root.borrow().print("-");
    println!("{}", file_tree);
}

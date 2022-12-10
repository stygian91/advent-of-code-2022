use std::fmt::Write;
use std::{cell::RefCell, rc::Rc};

pub type RcFile = Rc<RefCell<File>>;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
    pub size: Option<usize>,
    pub parent: Option<RcFile>,
    pub children: Vec<RcFile>,
}

impl File {
    pub fn new_rc(file: File) -> RcFile {
        Rc::new(RefCell::new(file))
    }

    pub fn new(name: &str, is_dir: bool) -> Self {
        Self {
            name: name.to_owned(),
            is_dir,
            size: None,
            parent: None,
            children: vec![],
        }
    }

    pub fn print(&self, sep: &str) -> String {
        let mut res = String::new();
        writeln!(
            res,
            "{}{} ({})",
            sep,
            self.name,
            if self.is_dir { "dir" } else { "file" }
        );

        for child in self.children.iter() {
            let mut new_sep = String::from(sep);
            new_sep.push_str("-");
            res.push_str(&child.borrow().print(&new_sep));
        }

        res
    }

    pub fn get_parent(&self) -> Option<RcFile> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, parent: RcFile) {
        self.parent = Some(parent);
    }

    pub fn add_child(&mut self, child: RcFile) {
        self.children.push(child);
    }
}

pub fn parse_ls(dir: RcFile, start_idx: usize, lines: &[&str]) -> usize {
    let mut i = start_idx + 1;
    let mut iter = lines.iter().skip(i);
    while let Some(line) = iter.next() {
        if line.starts_with('$') {
            return i;
        }

        i += 1;

        let parts: Vec<&str> = line.split(' ').collect();
        let name = parts[1];
        let is_dir = parts[0] == "dir";
        let file = File::new_rc(File::new(name, is_dir));
        dir.borrow_mut().add_child(file.clone());
        file.borrow_mut().set_parent(dir.clone());
    }

    i
}

pub fn parse_cd(cwd: RcFile, line: &str) -> RcFile {
    let mut parts = line.split(' ');
    let name = parts.nth(2).unwrap();
    if name == ".." {
        return cwd.borrow().get_parent().unwrap();
    }

    for child in cwd.borrow().children.iter() {
        if child.borrow().name == name {
            return child.clone();
        }
    }

    unreachable!("Couldn't find dir to cd into.");
}

pub fn parse(cwd: RcFile, idx: usize, lines: &[&str]) {
    if idx >= lines.len() {
        return;
    }

    let line = lines[idx];
    if line.starts_with("$ cd") {
        let next = parse_cd(cwd, line);
        return parse(next, idx + 1, lines);
    }

    if (line.starts_with("$ ls")) {
        let end_idx = parse_ls(cwd.clone(), idx, lines);
        return parse(cwd, end_idx, lines);
    }

    unreachable!("Trying to parse wrong line.");
}

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use super::*;

    #[test]
    fn parse_ls_works() {
        let contents = read_to_string(&Path::new("./data/demo.txt")).unwrap();
        let lines = contents.lines().collect::<Vec<&str>>();

        let mut root = File::new_rc(File::new("/", true));
        let next_idx = parse_ls(root.clone(), 1, &lines);
        assert_eq!(6, next_idx);
        assert_eq!(4, root.borrow().children.len());
    }
}

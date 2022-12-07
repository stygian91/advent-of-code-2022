use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct File {
    name: String,
    is_dir: bool,
}

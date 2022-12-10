use super::common::*;

const MAX_SIZE: usize = 100000;

pub fn part1(cwd: RcFile) -> usize {
    let mut accumulator = 0;

    walk(cwd.clone(), &mut |file| {
        let f_borrow = file.borrow();
        if !f_borrow.is_dir {
            return;
        }

        let size = f_borrow.size.unwrap();
        if size <= MAX_SIZE {
            accumulator += size;
        }
    });

    accumulator
}

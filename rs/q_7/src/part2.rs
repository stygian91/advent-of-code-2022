use crate::common::*;

const FREE_NEEDED: usize = 30000000;
const TOTAL_SPACE: usize = 70000000;
const MAX_USED: usize = TOTAL_SPACE - FREE_NEEDED;

pub fn part2(root: RcFile) -> usize {
    let root_size = root.borrow().size.unwrap();
    let to_be_freed = root_size - MAX_USED;
    let mut current_req_min = root_size;

    walk(root.clone(), &mut |file| {
        let f_borrow = file.borrow();
        if !f_borrow.is_dir {
            return;
        }

        let size = f_borrow.size.unwrap();
        if (size < current_req_min && size >= to_be_freed) {
          current_req_min = size;
        }
    });

    current_req_min
}
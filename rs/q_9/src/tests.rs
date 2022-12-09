use super::*;

#[test]
fn part1_works() {
    let moves = parse_moves("./data/demo.txt");

    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
        tail_positions: BTreeSet::new(),
    };

    let res = part1(&moves, &mut rope);
    assert_eq!(res, 13);
}

use super::*;

#[test]
fn part1_works() {
    let moves = parse_moves("./data/demo.txt");
    let res = part1(&moves);
    assert_eq!(res, 13);
}

#[test]
fn part2_works() {
    let moves = parse_moves("./data/demo.txt");
    let res = part2(&moves);
    assert_eq!(res, 1);
}

#[test]
fn part2_advanced_works() {
    let moves = parse_moves("./data/demo2.txt");
    let res = part2(&moves);
    assert_eq!(res, 36);
}
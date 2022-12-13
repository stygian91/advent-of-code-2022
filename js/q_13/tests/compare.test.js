const compare = require('../compare');

describe('test compare', () => {
  test('numbers', () => {
    expect(compare(1, 1)).toEqual(0);
    expect(compare(3, 5)).toEqual(-1);
    expect(compare(5, 3)).toEqual(1);
  });

  test('lists', () => {
    const a = [1, 1, 3, 1, 1];
    const b = [1, 1, 5, 1, 1];
    expect(compare(a, b)).toEqual(-1);
  });

  test('mixed', () => {
    const a = [[1],[2,3,4]];
    const b =[[1],4];
    expect(compare(a, b)).toEqual(-1);
  });

  test('mixed wrong order', () => {
    const a = [9];
    const b = [[8,7,6]];
    expect(compare(a, b)).toEqual(1);
  });

  test('different lengths', () => {
    const a = [[4,4],4,4];
    const b = [[4,4],4,4,4];
    expect(compare(a, b)).toEqual(-1);
  });

  test('different lengths wrong order', () => {
    const a = [7,7,7,7];
    const b = [7,7,7];
    expect(compare(a, b)).toEqual(1);
  });

  test('left empty', () => {
    const a = [];
    const b = [3];
    expect(compare(a, b)).toEqual(-1);
  });

  test('empty wrong order', () => {
    const a = [[[]]];
    const b = [[]];
    expect(compare(a, b)).toEqual(1);
  });

  test('bigger lists', () => {
    const a = [1,[2,[3,[4,[5,6,7]]]],8,9];
    const b = [1,[2,[3,[4,[5,6,0]]]],8,9];
    expect(compare(a, b)).toEqual(1);
  });
});

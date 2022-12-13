const part1 = require('../part1');

test('part 1 works with demo data', () => {
  const res = part1('./data/demo.txt');
  expect(res).toEqual(13);
});
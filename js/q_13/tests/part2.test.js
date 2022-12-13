const part2 = require('../part2');

test('part2', () => {
  const result = part2("./data/demo.txt");
  expect(result).toEqual(140);
});

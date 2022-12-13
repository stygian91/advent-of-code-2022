const part1 = require('./part1');
const part2 = require('./part2');

const main = () => {
  const p1_result = part1("./data/input.txt");
  console.log('part 1: ' + p1_result);

  const p2_result = part2("./data/input.txt");
  console.log('part 2: ' + p2_result);
};

main();

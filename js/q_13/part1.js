const parse = require('./parse');
const compare = require('./compare');

const part1 = (path) => {
  const pairs = parse(path);
  let sum = 0;

  for (let i = 0; i < pairs.length; i++) {
    const pair = pairs[i];
    if (compare(pair[0], pair[1]) <= 0) {
      sum += i + 1;
    }
  }

  return sum;
};

module.exports = part1;

const parse = require('./parse');
const compare = require('./compare');

const div1 = [[2]];
const div2 = [[6]];

const part2 = (path) => {
  const pairs = parse(path);
  const result = [];

  pairs.forEach((pair) => {
    result.push(pair[0]);
    result.push(pair[1]);
  });

  result.push(div1);
  result.push(div2);

  const sorted = result.sort(compare);
  let div1_idx = 0,
    div2_idx = 0;

  sorted.forEach((el, idx) => {
    const div1_cmp = compare(el, div1);
    const div2_cmp = compare(el, div2);

    if (div1_cmp === 0) {
      div1_idx = idx;
    }

    if (div2_cmp === 0) {
      div2_idx = idx;
    }
  });

  return (div1_idx + 1) * (div2_idx + 1);
}

module.exports = part2;

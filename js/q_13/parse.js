const { readFileSync } = require('fs');

const parse = (path) => readFileSync(path, { encoding: 'utf8' })
  .split('\n\n')
  .map(pair => pair
    .split('\n')
    .map(parseLine)
  );

const parseLine = (line) => JSON.parse(line);

module.exports = parse;

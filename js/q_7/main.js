const { readFileSync } = require('fs');

const mkdir = ({ name, parent = null }) => ({
  name,
  is_dir: true,
  children: [],
  parent,
  size: null,
});

const touch = ({ name, size, parent = null, }) => ({
  name,
  is_dir: false,
  parent,
  size,
});

const parseLsEntry = (line) => {
  let parts = line.split(' ');
  if (parts[0] === 'dir') {
    return mkdir({ name: parts[1] });
  }

  return touch({
    name: parts[1],
    size: parseInt(parts[0]),
  });
};

const parseLs = (cwd, startIdx, lines) => {
  let i;
  let files = [];

  for (i = startIdx + 1; i < lines.length; i++) {
    const line = lines[i];
    if (line.charAt(0) === "$") {
      break;
    }

    const entry = {
      ...parseLsEntry(line),
      parent: cwd,
    };

    files.push(entry);
  }

  return [i, files];
};

const parseCd = (cwd, line) => {
  let parts = line.split(' ');
  let name = parts[2];
  if (name === '..') {
    return cwd.parent;
  }

  return cwd.children.find(child => child.name === name);
};

const parse = (cwd, idx, lines) => {
  if (idx >= lines.length) {
    return;
  }

  const line = lines[idx];
  if (line.startsWith('$ cd')) {
    let next = parseCd(cwd, line);
    return parse(next, idx + 1, lines);
  }

  if (line.startsWith('$ ls')) {
    let [endIdx, files] = parseLs(cwd, idx, lines);
    cwd.children = files;
    return parse(cwd, endIdx, lines);
  }

  throw new Error('Unreachable');
};

const walk = (cwd, callback) => {
  if (!cwd.children) {
    return;
  }

  cwd.children.forEach((child) => {
    if (child.is_dir) {
      walk(child, callback);
    }

    callback(child);
  });
};

const updateSizes = (cwd) => {
  let size = 0;

  for (let i = 0; i < cwd.children.length; i++) {
    const child = cwd.children[i];
    if (child.is_dir) {
      updateSizes(child);
    }

    size += child.size;
  }

  cwd.size = size;
};

const part1_sum = (cwd) => {
  const max_size = 100000;
  let accumulator = 0;

  walk(cwd, (file) => {
    if (!file.is_dir) {
      return;
    }

    if (file.size <= max_size) {
      accumulator += file.size;
    }
  });

  return accumulator;
};

const part1 = (root) => {
  const acc = { value: 0 };
  const sum = part1_sum(root, acc);
  console.log(sum);
};

const part2 = (root) => {
  const free_needed = 30000000;
  const total_space = 70000000;
  const max_used = total_space - free_needed;
  const to_be_freed = root.size - max_used;

  let current_req_min = root.size;
  walk(root, (file) => {
    if (!file.is_dir) {
      return;
    }

    if (file.size < current_req_min && file.size >= to_be_freed) {
      current_req_min = file.size;
    }
  });

  console.log(current_req_min);
};

const main = () => {
  const lines = readFileSync('./input.txt', { encoding: 'utf-8' }).split("\n");
  // const lines = readFileSync('./demo.txt', { encoding: 'utf-8' }).split("\n");
  const root = mkdir({ name: '/' });
  parse(root, 1, lines);
  updateSizes(root);
  part1(root);
  part2(root);
};

main();

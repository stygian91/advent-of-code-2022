const compare = (a, b) => {
  const a_type = typeof a;
  const b_type = typeof b;
  if (a_type === 'number' && b_type === 'number') {
    const diff = a - b;
    return diff > 0 ? 1 : diff < 0 ? -1 : 0;
  }

  const a_list = a_type === 'number' ? [a] : a;
  const b_list = b_type === 'number' ? [b] : b;
  return compareLists(a_list, b_list);
};

const compareLists = (a, b) => {
  const max_len = Math.max(a.length, b.length);

  for (let i = 0; i < max_len; i++) {
    const a_el = a[i];
    const b_el = b[i];
    if (typeof a_el === 'undefined') {
      return -1;
    }

    if (typeof b_el === 'undefined') {
      return 1;
    }

    const cmp = compare(a_el, b_el);
    if (cmp !== 0) {
      return cmp;
    }
  }

  return 0;
}

module.exports = compare;

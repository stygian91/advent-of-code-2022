use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

pub type Coord = (isize, isize);

#[derive(Debug)]
pub struct BTreeMap2D<T> {
    elements: BTreeMap<Coord, T>,
}

impl<T> BTreeMap2D<T> {
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
        }
    }

    pub fn range(&self, ne: &Coord) -> impl Iterator<Item = (&Coord, &T)> {
        self.elements
            .range((Included(ne), Included(&(ne.0, ne.1 + 2))))
            .chain(
                self.elements
                    .range((Included(&(ne.0 + 1, ne.1)), Included(&(ne.0 + 1, ne.1 + 2)))),
            )
            .chain(
                self.elements
                    .range((Included(&(ne.0 + 2, ne.1)), Included(&(ne.0 + 2, ne.1 + 2)))),
            )
    }

    pub fn neighbours(&self, pos: Coord) -> impl Iterator<Item = (&Coord, &T)> {
        let ne = (pos.0 - 1, pos.1 - 1);
        self.range(&ne)
            .filter(move |(curr_pos, _)| curr_pos.0 != pos.0 || curr_pos.1 != pos.1)
    }

    pub fn insert(&mut self, key: Coord, value: T) -> Option<T> {
        self.elements.insert(key, value)
    }

    pub fn get(&self, key: &Coord) -> Option<&T> {
        self.elements.get(key)
    }

    pub fn get_mut(&mut self, key: &Coord) -> Option<&mut T> {
        self.elements.get_mut(key)
    }

    pub fn entry(&mut self, key: Coord) -> Entry<(isize, isize), T> {
        self.elements.entry(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_works() {
        let mut elves = BTreeMap::new();
        elves.insert((0, 0), 0);
        elves.insert((0, 1), 1);
        elves.insert((0, 2), 2);
        elves.insert((1, 0), 3);
        elves.insert((1, 1), 4);
        elves.insert((1, 2), 5);
        elves.insert((2, 0), 6);
        elves.insert((2, 1), 7);
        elves.insert((2, 2), 8);
        // ----------
        elves.insert((0, -1), 9);
        elves.insert((2, 3), 10);
        elves.insert((1, -1), 11);
        let field = BTreeMap2D { elements: elves };
        let neighbours = field
            .neighbours((1, 1))
            .map(|pair| (pair.0.to_owned(), pair.1.to_owned()))
            .collect::<Vec<(Coord, usize)>>();

        let expected = [
            ((0, 0), 0),
            ((0, 1), 1),
            ((0, 2), 2),
            ((1, 0), 3),
            ((1, 2), 5),
            ((2, 0), 6),
            ((2, 1), 7),
            ((2, 2), 8),
        ];

        assert_eq!(neighbours, expected);
    }
}

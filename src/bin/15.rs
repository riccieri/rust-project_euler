/* Problem 15: Lattice Paths
 *
 * Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
 * there are exactly 6 routes to the bottom right corner.
 *
 * How many such routes are there through a 20×20 grid? */

#![allow(unstable)]
use std::collections::BTreeMap;

fn main() {
    let result = ways_to_reach(20, 20, &mut BTreeMap::new());
    println!("{}", result);
}

fn ways_to_reach(row: usize, column: usize, cache: &mut BTreeMap<(usize, usize), usize>) -> usize {
    return match cache.get(&(row, column)) {
        Some(&value) => value,

        None => {
            let value = compute(row, column, cache);
            cache.insert((row, column), value);
            value
        }
    };
}

fn compute(row: usize, column: usize, cache: &mut BTreeMap<(usize, usize), usize>) -> usize {
    match (row, column) {
        (_, 0)        => 1,
        (0, _)        => 1,
        (row, column) => ways_to_reach(row - 1, column, cache) + ways_to_reach(row, column - 1, cache)
    }
}

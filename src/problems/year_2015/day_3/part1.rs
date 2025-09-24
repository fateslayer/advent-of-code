use super::common::move_position;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    visited.insert(position);

    for c in input.chars() {
        position = move_position(position, c);
        visited.insert(position);
    }

    visited.len()
}

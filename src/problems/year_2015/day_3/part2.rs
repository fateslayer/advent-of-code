use super::common::move_position;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    visited.insert(santa_pos);

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa_pos = move_position(santa_pos, c);
            visited.insert(santa_pos);
        } else {
            robo_pos = move_position(robo_pos, c);
            visited.insert(robo_pos);
        }
    }

    visited.len()
}

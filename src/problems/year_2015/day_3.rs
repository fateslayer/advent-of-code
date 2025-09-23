// Problem: https://adventofcode.com/2015/day/3

use std::collections::HashSet;

pub fn solve(input: &str) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    visited.insert(position);

    for c in input.chars() {
        position = move_position(position, c);
        visited.insert(position);
    }

    visited.len()
}

fn solve_part2(input: &str) -> usize {
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

fn move_position((x, y): (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => (x, y + 1),
        '>' => (x + 1, y),
        'v' => (x, y - 1),
        '<' => (x - 1, y),
        _ => (x, y),
    }
}
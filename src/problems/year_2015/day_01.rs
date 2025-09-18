// Problem: https://adventofcode.com/2015/day/1

use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/2015/01.txt").unwrap();

    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => panic!("Unsupported char"),
        }

        if count == -1 {
            println!("Basement: {}", i + 1);
        }
    }

    println!("Count: {}", count);
}
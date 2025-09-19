// Problem: https://adventofcode.com/2015/day/2

use std::fs;
use itertools::Itertools;

pub fn solve() {
    let input = fs::read_to_string("inputs/2015/02.txt").unwrap();

    let lines = input.lines();

    let mut total: i32 = 0;

    for line in lines {
        let arr: Vec<i32> = line.split("x").map(|s| s.parse().unwrap()).sorted().collect();

        let x = arr.get(0).unwrap();
        let y = arr.get(1).unwrap();
        let z = arr.get(2).unwrap();

        let xy = x * y;
        let yz = y * z;
        let xz = x * z;

        let min = xy.min(yz.min(xz));
        let area = (2 * xy) + (2 * yz) + (2 * xz);
        let ribbon = (x * 2) + (y * 2);
        let bow = x * y * z;

        total += ribbon + bow;
    }

    println!("{}", total);
}
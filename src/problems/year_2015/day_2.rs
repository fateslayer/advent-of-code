// Problem: https://adventofcode.com/2015/day/2

use itertools::Itertools;

pub fn solve(input: &str) {
    let lines = input.lines();

    let mut p1_total: i32 = 0;
    let mut p2_total: i32 = 0;

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

        p1_total += min + area;

        let ribbon = (x * 2) + (y * 2);
        let bow = x * y * z;

        p2_total += ribbon + bow;
    }

    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);
}
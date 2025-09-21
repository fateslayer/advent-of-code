// Problem: https://adventofcode.com/2015/day/1

pub fn solve(input: String) {
    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }

        // Part 2
        if count == -1 {
            println!("Basement: {}", i + 1);
            return;
        }
    }

    println!("Count: {}", count);
}
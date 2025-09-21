// Problem: https://adventofcode.com/2015/day/1

pub fn solve(input: String) {
    let mut count = 0;
    let mut basement: Option<usize> = None;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }

        if count == -1 && basement.is_none() {
            basement = Some(i + 1);
        }
    }
    
    println!("Part 1: {}", count);
    println!("Part 2: {:?}", basement);
}
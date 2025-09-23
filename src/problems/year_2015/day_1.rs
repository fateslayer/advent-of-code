// Problem: https://adventofcode.com/2015/day/1

pub fn solve(input: &str) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2.unwrap());
}

fn solve_part1(input: &str) -> i32 {
    let mut count = 0;

    for c in input.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }
    }

    count
}

fn solve_part2(input: &str) -> Option<usize> {
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
            break;
        }
    }

    basement
}
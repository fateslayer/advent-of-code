use std::collections::HashSet;

pub fn solve(input: &str) {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut position = (0, 0);
    map.insert(position);

    for c in input.chars().step_by(2) {
        match c {
            '^' => position.1 += 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '<' => position.0 -= 1,
            _ => continue,
        }

        map.insert(position);
    }

    position = (0, 0);

    for c in input.chars().skip(1).step_by(2) {
        match c {
            '^' => position.1 += 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '<' => position.0 -= 1,
            _ => continue,
        }

        map.insert(position);
    }

    println!("{}", map.len());
}
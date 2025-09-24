pub fn solve(input: &str) -> usize {
    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }

        if count == -1 {
            return i + 1;
        }
    }

    panic!("Never reached the basement!")
}

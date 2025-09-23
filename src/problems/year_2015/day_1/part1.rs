pub fn solve(input: &str) -> i32 {
    input.chars().fold(0, |count, c| {
        match c {
            '(' => count + 1,
            ')' => count - 1,
            _ => count,
        }
    })
}
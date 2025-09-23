pub fn move_position((x, y): (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => (x, y + 1),
        '>' => (x + 1, y),
        'v' => (x, y - 1),
        '<' => (x - 1, y),
        _ => (x, y),
    }
}
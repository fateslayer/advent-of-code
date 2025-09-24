pub fn parse_dimensions(line: &str) -> [i32; 3] {
    line.split("x")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

use super::common::parse_dimensions;

pub fn solve(input: &str) -> i32 {
    input.lines().fold(0, |total, line| {
        let mut dimensions = parse_dimensions(line);
        dimensions.sort();

        let [x, y, z] = dimensions;

        let ribbon = (x * 2) + (y * 2);
        let bow = x * y * z;

        total + ribbon + bow
    })
}

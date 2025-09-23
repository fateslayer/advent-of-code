use super::common::parse_dimensions;

pub fn solve(input: &str) -> i32 {
    input.lines().fold(0, |total, line| {
        let [x, y, z] = parse_dimensions(line);

        let xy = x * y;
        let yz = y * z;
        let xz = x * z;

        let min = xy.min(yz.min(xz));
        let area = (2 * xy) + (2 * yz) + (2 * xz);

        total + min + area
    })
}
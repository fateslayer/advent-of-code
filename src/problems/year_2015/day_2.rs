// Problem: https://adventofcode.com/2015/day/2

pub fn solve(input: &str) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_dimensions(line: &str) -> [i32; 3] {
    line.split("x").map(|s| s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()
}

fn solve_part1(input: &str) -> i32 {
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

fn solve_part2(input: &str) -> i32 {
    input.lines().fold(0, |total, line| {
        let mut dimensions = parse_dimensions(line);
        dimensions.sort();

        let [x, y, z] = dimensions;

        let ribbon = (x * 2) + (y * 2);
        let bow = x * y * z;

        total + ribbon + bow
    })
}
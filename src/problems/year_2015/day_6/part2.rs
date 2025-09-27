use fancy_regex::Regex;

pub fn solve(input: &str) -> usize {
    const SIZE: usize = 1000;
    let mut grid = [[0usize; SIZE]; SIZE];
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    for line in input.lines() {
        if let Ok(Some(captures)) = re.captures(line) {
            let action = &captures[1];
            let x1: usize = captures[2].parse().unwrap();
            let y1: usize = captures[3].parse().unwrap();
            let x2: usize = captures[4].parse().unwrap();
            let y2: usize = captures[5].parse().unwrap();

            for i in x1..=x2 {
                for j in y1..=y2 {
                    match action {
                        "turn on" => grid[i][j] += 1,
                        "turn off" => {
                            if grid[i][j] > 0 {
                                grid[i][j] -= 1
                            }
                        }
                        "toggle" => grid[i][j] += 2,
                        _ => panic!("unknown"),
                    }
                }
            }
        }
    }

    grid.iter().flatten().sum()
}

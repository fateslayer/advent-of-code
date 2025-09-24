mod problems;

use problems::year_2015;
use std::fs;

pub fn run_solution(day: u8, year: u16) {
    let path = format!("src/problems/year_{}/day_{}/input.txt", year, day);
    let result = fs::read_to_string(path);

    match result {
        Ok(input) => match (year, day) {
            (2015, 1) => year_2015::day_1::solve(&input),
            (2015, 2) => year_2015::day_2::solve(&input),
            (2015, 3) => year_2015::day_3::solve(&input),
            _ => panic!("Solution not found for year {} day {}", year, day),
        },
        Err(e) => panic!("Failed to read input file: {}", e),
    }
}

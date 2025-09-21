mod problems;

use std::fs;
use problems::year_2015;

pub fn run_solution(day: u8, year: u16, input_path: &str) {    
    let result = fs::read_to_string(input_path);

    match result {
        Ok(input) => {
            match (year, day) {
                (2015, 1) => year_2015::day_1::solve(&input),
                (2015, 2) => year_2015::day_2::solve(&input),
                _ => panic!("Solution not found for year {} day {}", year, day)
            }
        },
        Err(e) => panic!("Failed to read input file: {}", e)
    }
}
mod problems;

use std::fs;
use problems::year_2015;

pub fn run_solution(input_path: String, year: u16, day: u8) {
    if !std::path::Path::new(&input_path).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_path);
        std::process::exit(1);
    }
    
    let result = fs::read_to_string(input_path);

    match result {
        Ok(input) => {
            match (year, day) {
                (2015, 1) => year_2015::day_01::solve(input),
                (2015, 2) => year_2015::day_02::solve(input),
                _ => panic!("Solution not found for year {} day {}", year, day)
            }
        },
        Err(e) => panic!("Failed to read input file: {}", e)
    }
}
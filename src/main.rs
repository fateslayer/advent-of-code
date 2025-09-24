use advent_of_code::run_solution;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let year: u16;
    let day: u8;

    if let Some(day_str) = args.get(1) {
        day = day_str.parse().unwrap();
    } else {
        day = 1;
    }

    if let Some(year_str) = args.get(2) {
        year = year_str.parse().unwrap();
    } else {
        year = 2015;
    }

    run_solution(day, year);
}

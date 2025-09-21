use std::env;
use advent_of_code::run_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path: String;
    let year: u16;
    let day: u8;

    if let Some(path_str) = args.get(1) {
        input_path = path_str.to_string();
    } else {
        input_path = String::from("inputs/test.txt");
    }

    if let Some(year_str) = args.get(2) {
        year = year_str.parse().unwrap();
    } else {
        year = 2015;
    }

    if let Some(day_str) = args.get(3) {
        day = day_str.parse().unwrap();
    } else {
        day = 1;
    }

    run_solution(input_path, year, day);
}
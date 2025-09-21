use std::env;
use advent_of_code::run_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("Usage: {} <input_path> <year> <day>", args[0]);
    }

    let input_path: String = args[1].parse().unwrap();
    let year: u16 = args[2].parse().unwrap();
    let day: u8 = args[3].parse().unwrap();

    run_solution(input_path, year, day);
}
use std::env;
use advent_of_code::run_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: {} <year> <day>", args[0]);
    }

    let year: u16 = args[1].parse().unwrap();
    let day: u8 = args[2].parse().unwrap();

    run_solution(year, day);
}
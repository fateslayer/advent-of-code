pub mod problems;

pub use problems::year_2015;

pub fn run_solution(year: u16, day: u8) {
    match (year, day) {
        (2015, 1) => year_2015::day_01::solve(),
        (2015, 2) => year_2015::day_02::solve(),
        _ => panic!("Solution not found for year {} day {}", year, day)
    }
}
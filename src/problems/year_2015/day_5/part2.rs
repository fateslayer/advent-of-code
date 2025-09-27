use fancy_regex::Regex;

pub fn solve(input: &str) -> i32 {
    let mut count = 0;
    let consecutive = Regex::new(r"([a-zA-Z]{2}).*?\1").unwrap();
    let repeated = Regex::new(r"([a-zA-Z]).\1").unwrap();

    for line in input.lines() {
        if consecutive.is_match(line).unwrap() && repeated.is_match(line).unwrap() {
            count += 1;
        }
    }

    count
}

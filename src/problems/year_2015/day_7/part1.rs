use fancy_regex::Regex;
use std::collections::HashMap;

fn get_io_map(input: &str) -> HashMap<String, String> {
    let input_re = Regex::new(r"(.+) -> ([a-z]+)").unwrap();

    let mut map = HashMap::new();

    for line in input.lines() {
        if let Ok(Some(captures)) = input_re.captures(line) {
            let input = captures[1].to_string();
            let wire = captures[2].to_string();

            map.insert(wire, input);
        }
    }

    map
}

fn find_value(
    signal: &str,
    map: &HashMap<String, String>,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(&cached) = cache.get(signal) {
        return cached;
    }

    if let Ok(num) = signal.parse::<u32>() {
        return num;
    }

    let input = map.get(signal).unwrap();
    let result: u32;

    let num_re = Regex::new(r"^\d+$").unwrap();
    let string_re = Regex::new(r"^[a-z]+$").unwrap();
    let not_re = Regex::new(r"^NOT ([a-z]+)$").unwrap();
    let or_re = Regex::new(r"^(.+) OR (.+)$").unwrap();
    let and_re = Regex::new(r"^(.+) AND (.+)$").unwrap();
    let lshift_re = Regex::new(r"^(.+) LSHIFT (.+)$").unwrap();
    let rshift_re = Regex::new(r"^(.+) RSHIFT (.+)$").unwrap();

    if let Ok(Some(captures)) = num_re.captures(input) {
        let value = &captures[0];

        result = value.parse().unwrap();
    } else if let Ok(Some(captures)) = string_re.captures(input) {
        let value = &captures[0];

        result = find_value(value, map, cache);
    } else if let Ok(Some(captures)) = not_re.captures(input) {
        let value = &captures[1];

        result = (!find_value(value, map, cache)) & 0xFFFF;
    } else if let Ok(Some(captures)) = or_re.captures(input) {
        let left = &captures[1];
        let right = &captures[2];

        result = find_value(left, map, cache) | find_value(right, map, cache);
    } else if let Ok(Some(captures)) = and_re.captures(input) {
        let left = &captures[1];
        let right = &captures[2];

        result = find_value(left, map, cache) & find_value(right, map, cache);
    } else if let Ok(Some(captures)) = lshift_re.captures(input) {
        let left = &captures[1];
        let right = &captures[2];

        result = find_value(left, map, cache) << find_value(right, map, cache);
    } else if let Ok(Some(captures)) = rshift_re.captures(input) {
        let left = &captures[1];
        let right = &captures[2];

        result = find_value(left, map, cache) >> find_value(right, map, cache);
    } else {
        panic!("unsupported input: {}", input);
    }

    cache.insert(signal.to_string(), result);

    result
}

pub fn solve(input: &str) -> u32 {
    let map = get_io_map(input);
    let mut cache: HashMap<String, u32> = HashMap::new();
    find_value("a", &map, &mut cache)
}

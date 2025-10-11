use super::common::{Instruction, parse_instruction};
use fancy_regex::Regex;
use std::collections::HashMap;

fn get_io_map(input: &str) -> HashMap<String, Instruction> {
    let input_re = Regex::new(r"(.+) -> ([a-z]+)").unwrap();

    let mut map = HashMap::new();

    for line in input.lines() {
        if let Ok(Some(captures)) = input_re.captures(line) {
            let input = captures[1].to_string();
            let wire = captures[2].to_string();
            let instruction = parse_instruction(&input);

            map.insert(wire, instruction);
        }
    }

    map
}

fn find_value(
    signal: &str,
    map: &HashMap<String, Instruction>,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(&cached) = cache.get(signal) {
        return cached;
    }

    if let Ok(num) = signal.parse::<u32>() {
        return num;
    }

    let input = map.get(signal).unwrap();

    let result: u32 = match input {
        Instruction::Value(value) => *value,
        Instruction::Wire(value) => find_value(&value, map, cache),
        Instruction::Not(value) => (!find_value(&value, map, cache)) & 0xFFFF,
        Instruction::And(left, right) => {
            find_value(&left, map, cache) & find_value(&right, map, cache)
        }
        Instruction::Or(left, right) => {
            find_value(&left, map, cache) | find_value(&right, map, cache)
        }
        Instruction::LShift(left, right) => {
            find_value(&left, map, cache) << find_value(&right, map, cache)
        }
        Instruction::RShift(left, right) => {
            find_value(&left, map, cache) >> find_value(&right, map, cache)
        }
    };

    cache.insert(signal.to_string(), result);

    result
}

pub fn solve(input: &str) -> u32 {
    let map = get_io_map(input);
    let mut cache: HashMap<String, u32> = HashMap::new();
    find_value("a", &map, &mut cache)
}

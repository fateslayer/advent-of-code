use super::common::{Instruction, get_instruction_map};
use std::collections::HashMap;

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
    let map = get_instruction_map(input);
    let mut cache: HashMap<String, u32> = HashMap::new();
    find_value("a", &map, &mut cache)
}

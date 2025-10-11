use super::common::{Instruction, find_value, get_instruction_map};
use super::part1;
use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let previous_value = part1::solve(input);
    let mut map = get_instruction_map(input);

    map.insert("b".to_string(), Instruction::Value(previous_value));

    let mut cache: HashMap<String, u32> = HashMap::new();

    find_value("a", &map, &mut cache)
}

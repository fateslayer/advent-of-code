use super::common::{find_value, get_instruction_map};
use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let map = get_instruction_map(input);
    let mut cache: HashMap<String, u32> = HashMap::new();
    find_value("a", &map, &mut cache)
}

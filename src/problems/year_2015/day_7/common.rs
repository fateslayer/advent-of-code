use std::collections::HashMap;

use regex::Regex;

use once_cell::sync::Lazy;

static INPUT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.+) -> ([a-z]+)").unwrap());
static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());
static WIRE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z]+$").unwrap());
static NOT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^NOT ([a-z]+)$").unwrap());
static OR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+) OR (.+)$").unwrap());
static AND_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+) AND (.+)$").unwrap());
static LSHIFT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+) LSHIFT (.+)$").unwrap());
static RSHIFT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+) RSHIFT (.+)$").unwrap());

pub enum Instruction {
    Value(u32),
    Wire(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, String),
    RShift(String, String),
}

fn parse_instruction(input: &str) -> Instruction {
    if let Some(captures) = NUM_RE.captures(input) {
        let value = &captures[0];
        let value = value.parse().unwrap();
        Instruction::Value(value)
    } else if let Some(captures) = WIRE_RE.captures(input) {
        let value = captures[0].to_string();
        Instruction::Wire(value)
    } else if let Some(captures) = NOT_RE.captures(input) {
        let value = captures[1].to_string();
        Instruction::Not(value)
    } else if let Some(captures) = AND_RE.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::And(left, right)
    } else if let Some(captures) = OR_RE.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::Or(left, right)
    } else if let Some(captures) = LSHIFT_RE.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::LShift(left, right)
    } else if let Some(captures) = RSHIFT_RE.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::RShift(left, right)
    } else {
        panic!("unsupported input: {}", input);
    }
}

pub fn get_instruction_map(input: &str) -> HashMap<String, Instruction> {
    let mut map = HashMap::new();

    for line in input.lines() {
        if let Some(captures) = INPUT_RE.captures(line) {
            let input = captures[1].to_string();
            let wire = captures[2].to_string();
            let instruction = parse_instruction(&input);

            map.insert(wire, instruction);
        }
    }

    map
}

pub fn find_value(
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

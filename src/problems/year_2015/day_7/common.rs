use fancy_regex::Regex;

pub enum Instruction {
    Value(u32),
    Wire(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, String),
    RShift(String, String),
}

pub fn parse_instruction(input: &str) -> Instruction {
    let num_re = Regex::new(r"^\d+$").unwrap();
    let string_re = Regex::new(r"^[a-z]+$").unwrap();
    let not_re = Regex::new(r"^NOT ([a-z]+)$").unwrap();
    let or_re = Regex::new(r"^(.+) OR (.+)$").unwrap();
    let and_re = Regex::new(r"^(.+) AND (.+)$").unwrap();
    let lshift_re = Regex::new(r"^(.+) LSHIFT (.+)$").unwrap();
    let rshift_re = Regex::new(r"^(.+) RSHIFT (.+)$").unwrap();

    if let Ok(Some(captures)) = num_re.captures(input) {
        let value = &captures[0];
        let value = value.parse().unwrap();
        Instruction::Value(value)
    } else if let Ok(Some(captures)) = string_re.captures(input) {
        let value = captures[0].to_string();
        Instruction::Wire(value)
    } else if let Ok(Some(captures)) = not_re.captures(input) {
        let value = captures[1].to_string();
        Instruction::Not(value)
    } else if let Ok(Some(captures)) = and_re.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::And(left, right)
    } else if let Ok(Some(captures)) = or_re.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::Or(left, right)
    } else if let Ok(Some(captures)) = lshift_re.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::LShift(left, right)
    } else if let Ok(Some(captures)) = rshift_re.captures(input) {
        let left = captures[1].to_string();
        let right = captures[2].to_string();

        Instruction::RShift(left, right)
    } else {
        panic!("unsupported input: {}", input);
    }
}

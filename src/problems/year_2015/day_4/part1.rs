use md5;

pub fn solve(input: &str) -> i32 {
    let mut num = 1;
    let pattern = "00000";

    loop {
        let digest = md5::compute(format!("{}{}", input, num));

        if format!("{:x}", digest).starts_with(pattern) {
            break;
        }

        num += 1;
    }

    num
}

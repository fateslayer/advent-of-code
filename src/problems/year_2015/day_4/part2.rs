use md5;

pub fn solve(input: &str) -> i32 {
    let mut num = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, num));

        if format!("{:x}", digest).starts_with("000000") {
            break;
        }

        num += 1;
    }

    num
}

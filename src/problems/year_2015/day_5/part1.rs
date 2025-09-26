pub fn solve(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        let mut vowels = 0;
        let mut duplicate = false;
        let mut invalid = false;

        for (i, c) in line.chars().enumerate() {
            if "aeiou".contains(c) {
                vowels += 1;
            }

            if !duplicate && i > 0 && line.chars().nth(i - 1).unwrap() == c {
                duplicate = true;
            }

            if i > 0 {
                match &line[i - 1..=i] {
                    "ab" | "cd" | "pq" | "xy" => invalid = true,
                    _ => (),
                }
            }
        }

        if vowels >= 3 && duplicate && !invalid {
            count += 1;
        }
    }

    count
}

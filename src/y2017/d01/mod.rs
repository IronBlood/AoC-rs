pub fn run(input: &str) {
    println!("{}", get_captcha(input, 1));
    println!("{}", get_captcha(input, 2));
}

fn get_captcha(data: &str, part: i32) -> u32 {
    let mut sum: u32 = 0;
    let len = data.len();
    let half = len / 2;
    let bytes = data.as_bytes();
    let step = if part == 1 { 1 } else { half };

    for i in 0..len {
        let j = (i + step) % len;
        if bytes[i] == bytes[j] {
            let digit = (bytes[i] - b'0') as u32;
            sum += digit;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let testcases = [("1122", 3), ("1111", 4), ("91212129", 9)];
        for (data, expected) in testcases {
            assert_eq!(get_captcha(data, 1), expected);
        }
    }

    #[test]
    fn part2() {
        let testcases = [
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];
        for (data, expected) in testcases {
            assert_eq!(get_captcha(data, 2), expected);
        }
    }
}

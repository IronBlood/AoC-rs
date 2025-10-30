fn evaluate(data: &str, part: u8) -> u32 {
    let mut score = 0;
    let mut curr_depth = 0;
    let mut is_in_garbage = false;
    let mut is_canceled = false;
    let mut garbage_count = 0;

    for c in data.bytes() {
        if is_canceled {
            is_canceled = false;
            continue;
        }

        if c == b'!' {
            is_canceled = true;
            continue;
        }

        if !is_in_garbage && c == b'<' {
            is_in_garbage = true;
            continue;
        }

        if is_in_garbage {
            if c == b'>' {
                is_in_garbage = false;
            } else {
                garbage_count += 1;
            }
            continue;
        }

        if c == b'{' {
            curr_depth += 1;
            score += curr_depth;
        }

        if c == b'}' {
            curr_depth -= 1;
        }
    }

    if part == 1 { score } else { garbage_count }
}

pub fn run(input: &str) {
    println!("{}", evaluate(input, 1));
    println!("{}", evaluate(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let testcases = [
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];
        for (data, res) in testcases {
            assert_eq!(evaluate(data, 1), res);
        }
    }

    #[test]
    fn test2() {
        let testcases = [
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ];
        for (data, res) in testcases {
            assert_eq!(evaluate(data, 2), res);
        }
    }
}

use std::collections::HashSet;

fn count_valid_passphrases(data: &str, part: u8) -> u32 {
    data.lines().fold(0u32, |acc, line| {
        // ChatGPT
        let arr = line.split_whitespace();
        if part == 1 {
            let mut set = HashSet::new();
            for part in arr {
                if set.contains(part) {
                    return acc;
                }
                set.insert(part);
            }
        } else {
            let mut set = HashSet::new();
            for part in arr {
                let mut freq: [u8; 26] = [0; 26];
                for b in part.as_bytes() {
                    let idx = b - b'a';
                    freq[idx as usize] += 1;
                }
                if set.contains(&freq) {
                    return acc;
                }
                set.insert(freq);
            }
        };
        acc + 1
    })
}

pub fn run(input: &str) {
    println!("{}", count_valid_passphrases(input, 1));
    println!("{}", count_valid_passphrases(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let testcases = vec![
            ("aa bb cc dd ee", 1),
            ("aa bb cc dd aa", 0),
            ("aa bb cc dd aaa", 1),
        ];
        for (str, actual) in testcases {
            assert_eq!(count_valid_passphrases(str, 1), actual);
        }
    }

    #[test]
    fn part2() {
        let testcases = [
            ("abcde fghij", 1),
            ("abcde xyz ecdab", 0),
            ("a ab abc abd abf abj", 1),
            ("iiii oiii ooii oooi oooo", 1),
            ("oiii ioii iioi iiio", 0),
        ];
        for (str, actual) in testcases {
            assert_eq!(count_valid_passphrases(str, 2), actual);
        }
    }
}

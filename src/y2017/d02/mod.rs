use std::cmp::{max, min};

use regex::Regex;

fn checksum(data: &str, part: u8) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    data.lines().fold(0u32, |acc, line| {
        if part == 1 {
            let mut _max: u32 = 0;
            let mut _min: u32 = u32::MAX;

            re.find_iter(line).for_each(|m| {
                // m.as_str() fixed by ChatGPT
                let num: u32 = m.as_str().parse().expect("not a valid number");
                _min = min(_min, num);
                _max = max(_max, num);
            });
            acc + _max - _min
        } else {
            let mut nums: Vec<u32> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().expect("not a valid number"))
                .collect();

            nums.sort();

            let mut row = 0;
            // 'outer, fixed by ChatGPT
            'outer: for i in 0..nums.len() - 1 {
                for j in i + 1..nums.len() {
                    if nums[j] % nums[i] == 0 {
                        row = nums[j] / nums[i];
                        break 'outer;
                    }
                }
            }

            acc + row
        }
    })
}

pub fn run(input: &str) {
    println!("{}", checksum(input, 1));
    println!("{}", checksum(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(checksum(s, 1), 18);
    }

    #[test]
    fn part2() {
        let s = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(checksum(s, 2), 9);
    }
}

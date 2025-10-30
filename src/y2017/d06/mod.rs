use std::{collections::HashSet, str::FromStr};

fn dispatch(nums: &mut [u32]) {
    let mut max_idx = 0;
    let mut max = nums[0];
    for i in 1..nums.len() {
        if max < nums[i] {
            max_idx = i;
            max = nums[i];
        }
    }

    let count = max as usize % nums.len();
    let x = ((max as usize + nums.len() - 1) / nums.len()) as u32;
    nums[max_idx] = 0;
    for i in 0..nums.len() {
        let idx = (max_idx + 1 + i) % nums.len();
        if count == 0 {
            nums[idx] += x;
        } else {
            nums[idx] += if i < count { x } else { x - 1 };
        }
    }
}

fn count_cycles(data: &str, part: u8) -> u32 {
    let mut nums: Vec<u32> = data
        .split_whitespace()
        .map(|part| u32::from_str(part).expect("not a valid number"))
        .collect();

    let mut seen: HashSet<Vec<u32>> = HashSet::new();
    // ChatGPT
    let repeated: Vec<u32>;
    loop {
        if seen.contains(&nums) {
            repeated = nums.clone();
            break;
        }
        seen.insert(nums.clone());
        dispatch(&mut nums);
    }

    if part == 1 {
        return seen.len() as u32;
    }

    let mut count = 0;
    loop {
        count += 1;
        dispatch(&mut nums);
        if nums == repeated {
            break;
        }
    }
    count
}

pub fn run(input: &str) {
    println!("{}", count_cycles(input, 1));
    println!("{}", count_cycles(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "0 2 7 0";
        assert_eq!(count_cycles(data, 1), 5);
        assert_eq!(count_cycles(data, 2), 4);
    }
}

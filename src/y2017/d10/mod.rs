fn swap<T>(arr: &mut [T], mut start_idx: usize, count: usize) {
    if count == 0 {
        return;
    }
    let len = arr.len();
    let mut end_idx = (start_idx + count - 1) % len;
    let swaps = (count + 1) / 2;
    for _ in 0..swaps {
        // ChatGPT
        arr.swap(start_idx, end_idx);

        start_idx = (start_idx + 1) % len;
        end_idx = if end_idx == 0 { len - 1 } else { end_idx - 1 };
    }
}

fn get_checksum(data: &str, size: usize) -> usize {
    let mut nums: Vec<_> = (0..size).collect();
    let ins: Vec<usize> = data.split(",").map(|x| x.parse().unwrap()).collect();

    let mut idx = 0;
    for (i, &el) in ins.iter().enumerate() {
        swap(&mut nums, idx, el);
        idx = (idx + i + el) % size;
    }

    nums[0] * nums[1]
}

fn hex_nums(nums: &[u8]) -> String {
    // polished by ChatGPT
    let hex = b"0123456789abcdef";
    let mut builder = String::with_capacity(nums.len() * 2);

    for &x in nums {
        builder.push(hex[(x >> 4) as usize] as char);
        builder.push(hex[(x & 0xf) as usize] as char);
    }

    builder
}

// refacted by ChatGPT
fn get_dense_hash(nums: &[u8]) -> Vec<u8> {
    nums.chunks_exact(16)
        .map(|ch| ch.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .collect()
}

fn hash(data: &str) -> String {
    let mut lengths = data.as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut nums: Vec<u8> = (0..=255).collect();
    let mut idx = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &x in &lengths {
            let k = x as usize;
            swap(&mut nums, idx, k);
            idx = (idx + skip + k) % 256;
            skip += 1;
        }
    }
    hex_nums(&get_dense_hash(&nums))
}

pub fn run(input: &str) {
    println!("{}", get_checksum(input, 256));
    println!("{}", hash(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_checksum("3,4,1,5", 5), 12);
    }

    #[test]
    fn test2() {
        let testcases = [
            ["", "a2582a3a0e66e6e86e3812dcb672a272"],
            ["AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"],
            ["1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"],
            ["1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"],
        ];
        for tc in testcases {
            assert_eq!(hash(tc[0]), tc[1]);
        }
    }
}

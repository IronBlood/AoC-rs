fn find_pairs(data: &str, part: u8) -> u32 {
    let mut it = data.lines().map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("not a valid number")
    });
    // ChatGPT
    let (mut a, mut b) = (it.next().unwrap(), it.next().unwrap());

    const F_A: u64 = 16807;
    const F_B: u64 = 48271;
    const MOD: u64 = 0x7fffffff;

    let mut count = 0;

    if part == 1 {
        for _ in 0..40_000_000 {
            // ChatGPT
            a = ((a as u64 * F_A) % MOD) as u32;
            b = ((b as u64 * F_B) % MOD) as u32;
            if (a & 0xffff) == (b & 0xffff) {
                count += 1;
            }
        }
    } else {
        for _ in 0..5_000_000 {
            loop {
                a = ((a as u64 * F_A) % MOD) as u32;
                if (a & 0x3) == 0 {
                    break;
                }
            }
            loop {
                b = ((b as u64 * F_B) % MOD) as u32;
                if (b & 0x7) == 0 {
                    break;
                }
            }
            if (a & 0xffff) == (b & 0xffff) {
                count += 1;
            }
        }
    }

    count
}

pub fn run(input: &str) {
    println!("{}", find_pairs(input, 1));
    println!("{}", find_pairs(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "Generator A starts with 65\nGenerator B starts with 8921";
        assert_eq!(find_pairs(data, 1), 588);
        assert_eq!(find_pairs(data, 2), 309);
    }
}

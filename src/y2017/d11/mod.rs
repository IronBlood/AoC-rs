fn get_dist(s: i32, q: i32, r: i32) -> u32 {
    ((s.abs() + q.abs() + r.abs()) / 2) as u32
}

fn count_steps(data: &str, part: u8) -> u32 {
    let mut q = 0i32;
    let mut r = 0i32;
    let mut s: i32;
    let mut max = 0u32;
    let mut dist = 0u32;

    for d in data.split(",") {
        match d {
            "n" => r -= 1,
            "s" => r += 1,
            "ne" => {
                q += 1;
                r -= 1;
            }
            "sw" => {
                q -= 1;
                r += 1;
            }
            "se" => q += 1,
            "nw" => q -= 1,
            _ => panic!("invalid direction: {d}"),
        }
        s = -q - r;
        dist = get_dist(s, q, r);
        max = max.max(dist);
    }

    match part {
        1 => dist,
        2 => max,
        _ => panic!("invalid part: {part}"),
    }
}

pub fn run(input: &str) {
    println!("{}", count_steps(input, 1));
    println!("{}", count_steps(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let testcases = [
            ("ne,ne,ne", 3),
            ("ne,ne,sw,sw", 0),
            ("ne,ne,s,s", 2),
            ("se,sw,se,sw,sw", 3),
        ];
        for (data, res) in testcases {
            assert_eq!(count_steps(data, 1), res);
        }
    }
}

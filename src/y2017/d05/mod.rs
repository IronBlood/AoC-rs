fn count_steps(data: &str, part: u8) -> u32 {
    let mut instructions: Vec<i32> = data
        .lines()
        .map(|line| line.parse::<i32>().expect("not a number"))
        .collect();

    let mut i = 0usize;
    let mut count = 0;
    loop {
        if i as usize >= instructions.len() {
            break;
        }
        let next_idx = instructions[i] + i as i32;
        if next_idx < 0 {
            panic!("invalid index");
        }
        if part == 1 || instructions[i] < 3 {
            instructions[i] += 1;
        } else {
            instructions[i] -= 1;
        }
        count += 1;
        i = next_idx as usize;
    }
    count
}

pub fn run(input: &str) {
    println!("{}", count_steps(input, 1));
    println!("{}", count_steps(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "0\n3\n0\n1\n-3";
        assert_eq!(count_steps(data, 1), 5);
        assert_eq!(count_steps(data, 2), 10);
    }
}

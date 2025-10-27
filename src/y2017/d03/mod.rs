fn get_steps(data: u32) -> u32 {
    if data == 1 {
        return 0;
    }

    let mut d = 3;
    while d * d < data {
        d += 2;
    }

    let mut begin = d * d;
    loop {
        begin -= d - 1;
        if begin <= data {
            break;
        }
    }
    let center = begin + (d - 1) / 2;
    // ChatGPT
    let delta = data.abs_diff(center);
    delta + (d - 1) / 2
}

fn first_larger_value(data: u32) -> u32 {
    if data == 1 {
        return 2;
    }
    if data < 4 {
        return 4;
    }

    let mut d = 3;
    while d * d < data {
        d += 2;
    }

    let d = d as usize;

    let mut grid: Vec<Vec<i32>> = vec![vec![-1; d as usize]; d];
    let mut x: isize = (d / 2) as isize;
    let mut y = x;
    grid[x as usize][y as usize] = 1;
    y += 1;
    grid[x as usize][y as usize] = 1;
    x -= 1;

    // ChatGPT
    let directions: [[isize; 2]; 4] = [
        [-1, 0], // N
        [0, -1], // W
        [1, 0],  // S
        [0, 1],  // E
    ];
    let mut dir_idx: usize = 0;

    let in_grid = |x: isize, y: isize| x >= 0 && y >= 0 && (x as usize) < d && (y as usize) < d;

    loop {
        let mut count = 0;
        let mut curr: u32 = 0;
        for a in x - 1..=x + 1 {
            for b in y - 1..=y + 1 {
                if a == x && b == y {
                    continue;
                }
                if !in_grid(a, b) {
                    continue;
                }
                let val = grid[a as usize][b as usize];
                if val == -1 {
                    continue;
                }

                curr += val as u32;
                count += 1;
            }
        }

        if curr > data as u32 {
            return curr;
        }

        grid[x as usize][y as usize] = curr as i32;
        if count == 2 {
            dir_idx = (dir_idx + 1) % 4;
        }
        x += directions[dir_idx][0];
        y += directions[dir_idx][1];
    }
}

pub fn run(input: &str) {
    let data = input.parse::<u32>().expect("not a valid number");
    println!("{}", get_steps(data));
    println!("{}", first_larger_value(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let testcases = vec![(1, 0), (12, 3), (23, 2), (25, 4), (1024, 31)];
        for (data, expected) in testcases {
            assert_eq!(get_steps(data), expected);
        }
    }

    #[test]
    fn part2() {
        let testcases = vec![(3, 4), (6, 10), (7, 10), (330, 351)];
        for (data, expected) in testcases {
            assert_eq!(first_larger_value(data), expected);
        }
    }
}

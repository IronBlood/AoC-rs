use std::{collections::VecDeque, fmt::Write};

use super::d10::hash;

fn byte_to_num(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => 10 + (b - b'a'),
        b'A'..=b'A' => 10 + (b - b'A'),
        _ => unreachable!("invalid hex"),
    }
}

fn count_squares(data: &str) -> u32 {
    // ChatGPT
    let pop: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];
    // ChatGPT
    let mut key = String::with_capacity(data.len() + 4);

    (0..128)
        .map(|i| {
            key.clear();
            write!(&mut key, "{data}-{i}").unwrap();
            let str = hash(&key);

            str.bytes()
                .map(|b| {
                    let nib = byte_to_num(b) as usize;
                    pop[nib] as u32
                })
                .sum::<u32>()
        })
        .sum()
}

const DIRECTIONS: [i32; 5] = [0, 1, 0, -1, 0];
type Coord = (i32, i32);

fn in_grid<T>(grid: &mut [Vec<T>], x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    return x < grid.len() && y < grid[0].len();
}

fn bfs_flood(grid: &mut [Vec<u8>], x: usize, y: usize) {
    // safe for this situation
    let x = x as i32;
    let y = y as i32;
    let mut queue: VecDeque<Coord> = VecDeque::new();
    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if !in_grid(grid, x, y) {
            continue;
        }

        let x = x as usize;
        let y = y as usize;
        if grid[x][y] == 0 {
            continue;
        }

        grid[x][y] = 0;

        let x = x as i32;
        let y = y as i32;
        for i in 0..4 {
            queue.push_back((x + DIRECTIONS[i], y + DIRECTIONS[i + 1]));
        }
    }
}

fn count_regions(data: &str) -> u32 {
    let mut key = String::with_capacity(data.len() + 4);
    let mut disk: Vec<Vec<u8>> = (0..128)
        .map(|idx| {
            key.clear();
            write!(&mut key, "{data}-{idx}").unwrap();
            let mut bits: Vec<u8> = Vec::new();
            for b in hash(&key).bytes() {
                let n = byte_to_num(b);
                bits.push((n >> 3) & 1);
                bits.push((n >> 2) & 1);
                bits.push((n >> 1) & 1);
                bits.push(n & 1);
            }
            bits
        })
        .collect();

    let mut ground_id = 0;
    for i in 0..disk.len() {
        for j in 0..disk[0].len() {
            if disk[i][j] == 1 {
                bfs_flood(&mut disk, i, j);
                ground_id += 1;
            }
        }
    }
    ground_id
}

pub fn run(input: &str) {
    println!("{}", count_squares(input));
    println!("{}", count_regions(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "flqrgnkx";
        assert_eq!(count_squares(data), 8108);
        assert_eq!(count_regions(data), 1242);
    }
}

#[derive(Copy, Clone)]
enum State {
    /// clean
    C,
    /// weakened
    W,
    /// infected
    I,
    /// flagged
    F,
}

struct Carrier {
    pos: [i32; 2],
    dir: u8,
}

fn in_grid<T>(grid: &[Vec<T>], x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

fn enlarge(grid: Vec<Vec<State>>) -> Vec<Vec<State>> {
    let size = grid.len();
    let mut next = vec![vec![State::C; size * 3]; size * 3];

    for i in 0..size {
        next[i + size][size..2 * size].copy_from_slice(&grid[i]);
    }

    next
}

fn count_infection(data: &str, part: u8, iterations: usize) -> u32 {
    let mut grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| {
                    if c == b'#' {
                        if part == 1 { State::W } else { State::I }
                    } else {
                        State::C
                    }
                })
                .collect()
        })
        .collect();

    let mut count = 0;
    let mut carrier = Carrier {
        pos: [(grid.len() >> 1) as i32, (grid[0].len() >> 1) as i32],
        dir: 0,
    };

    let directions = [
        [-1, 0], // N
        [0, 1],  // E
        [1, 0],  // S
        [0, -1], // W
    ];

    for _ in 0..iterations {
        if !in_grid(&grid, carrier.pos[0], carrier.pos[1]) {
            let size = grid.len();
            grid = enlarge(grid);
            carrier.pos[0] += size as i32;
            carrier.pos[1] += size as i32;
        }

        let x = carrier.pos[0] as usize;
        let y = carrier.pos[1] as usize;

        if part == 1 {
            match grid[x][y] {
                State::W => {
                    carrier.dir += 1;
                    carrier.dir %= 4;
                    grid[x][y] = State::C;
                }
                State::C => {
                    carrier.dir += 3;
                    carrier.dir %= 4;
                    grid[x][y] = State::W;
                    count += 1;
                }
                _ => panic!("invalid state for part 1"),
            }
        } else {
            match grid[x][y] {
                State::I => {
                    carrier.dir += 1;
                    carrier.dir %= 4;
                    grid[x][y] = State::F;
                }
                State::C => {
                    carrier.dir += 3;
                    carrier.dir %= 4;
                    grid[x][y] = State::W;
                }
                State::F => {
                    carrier.dir += 2;
                    carrier.dir %= 4;
                    grid[x][y] = State::C;
                }
                State::W => {
                    grid[x][y] = State::I;
                    count += 1;
                }
            }
        }

        let dir = &directions[carrier.dir as usize];
        carrier.pos[0] += dir[0];
        carrier.pos[1] += dir[1];
    }

    count
}

pub fn run(input: &str) {
    println!("{}", count_infection(input, 1, 10000));
    println!("{}", count_infection(input, 2, 10000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "..#\n#..\n...";

        let testcases = [
            [1, 1, 1],
            [1, 2, 1],
            [1, 7, 5],
            [1, 10000, 5587],
            [2, 100, 26],
            [2, 10000000, 2511944],
        ];

        for tc in testcases {
            assert_eq!(count_infection(data, tc[0] as u8, tc[1]), tc[2] as u32);
        }
    }
}

fn get_start_pos(grid: &Vec<&[u8]>) -> (usize, usize) {
    for j in 0..grid[0].len() {
        if grid[0][j] == b'|' {
            return (0, j);
        }
    }
    unreachable!("invalid input");
}

#[derive(Debug)]
struct Pointer {
    pos: [i32; 2],
    dir: [i32; 2],
}

fn get_letters(data: &str) -> (String, u32) {
    let grid: Vec<_> = data.lines().map(|line| line.as_bytes()).collect();
    let (start_x, start_y) = get_start_pos(&grid);
    let mut letters = String::new();
    let mut steps = 0;
    let mut ptr = Pointer {
        pos: [start_x as i32, start_y as i32],
        dir: [1, 0],
    };

    let in_grid = |x: i32, y: i32| {
        x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
    };

    loop {
        if !in_grid(ptr.pos[0], ptr.pos[1]) {
            break;
        }
        let ch = grid[ptr.pos[0] as usize][ptr.pos[1] as usize];
        if ch == b' ' {
            break;
        }

        steps += 1;

        if ch == b'+' {
            let candidate_dirs = if ptr.dir[0] == 0 {
                [[-1, 0], [1, 0]]
            } else {
                [[0, -1], [0, 1]]
            };

            for [dx, dy] in candidate_dirs {
                let nx = ptr.pos[0] + dx;
                let ny = ptr.pos[1] + dy;
                if in_grid(nx, ny) && grid[nx as usize][ny as usize] != b' ' {
                    ptr.pos[0] = nx;
                    ptr.pos[1] = ny;
                    ptr.dir[0] = dx;
                    ptr.dir[1] = dy;
                    break;
                }
            }
            continue;
        }

        if ch.is_ascii_lowercase() || ch.is_ascii_uppercase() {
            letters.push(ch as char);
        }

        ptr.pos[0] += ptr.dir[0];
        ptr.pos[1] += ptr.dir[1];
    }

    (letters, steps)
}

pub fn run(input: &str) {
    let (letters, steps) = get_letters(input);
    println!("{}", letters);
    println!("{}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut data = String::new();
        data.push_str("     |          \n");
        data.push_str("     |  +--+    \n");
        data.push_str("     A  |  C    \n");
        data.push_str(" F---|----E|--+ \n");
        data.push_str("     |  |  |  D \n");
        data.push_str("     +B-+  +--+ ");

        let (l, s) = get_letters(&data);
        assert_eq!(l, "ABCDEF");
        assert_eq!(s, 38);
    }
}

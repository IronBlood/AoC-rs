use std::collections::HashMap;

use crate::lib::matrix::{RotateOrientation, flip_w_tile, rotate_tile};

fn get_size<T>(grid: &[Vec<T>]) -> usize {
    let x = grid.len();
    debug_assert!(grid.iter().all(|row| row.len() == x));
    x
}

fn convert_string_to_tile(s: &str) -> Vec<Vec<u8>> {
    s.split('/').map(|p| p.as_bytes().to_vec()).collect()
}

fn convert_tile_to_string(grid: &[Vec<u8>], x: usize, y: usize, step: usize) -> String {
    assert!(step == 2 || step == 3, "invalid step");
    let mut out = String::with_capacity(step * step + (step - 1));
    for i in 0..step {
        if i > 0 {
            out.push('/');
        }
        for j in 0..step {
            out.push(grid[x + i][y + j] as char);
        }
    }
    out
}

fn enhance(
    grid: Vec<Vec<u8>>,
    step: usize,
    convert_book: &HashMap<String, Vec<Vec<u8>>>,
) -> Vec<Vec<u8>> {
    let n = get_size(&grid) / step;
    let ns = step + 1;
    let mut next_grid = vec![vec![grid[0][0]; ns * n]; ns * n];

    for i in 0..n {
        for j in 0..n {
            let replacement = convert_book
                .get(&convert_tile_to_string(&grid, i * step, j * step, step))
                .expect("no replacement found");
            for u in 0..ns {
                // ChatGPT: fast row copy
                next_grid[i * ns + u][j * ns..j * ns + ns].copy_from_slice(&replacement[u]);
            }
        }
    }

    next_grid
}

fn count_on(data: &str, iterations: usize) -> usize {
    let mut convert_book = HashMap::new();
    for line in data.lines() {
        let (str_from, str_to) = line.split_once(" => ").expect("invalid input");
        let grid_to = convert_string_to_tile(str_to);
        convert_book.insert(str_from.to_string(), grid_to.clone());

        // TODO can be simplified
        let grid_from = convert_string_to_tile(str_from);
        for ro in [
            RotateOrientation::C90,
            RotateOrientation::C180,
            RotateOrientation::C270,
        ] {
            let grid_from = rotate_tile(&grid_from, ro);
            convert_book.insert(
                convert_tile_to_string(&grid_from, 0, 0, get_size(&grid_from)),
                grid_to.clone(),
            );
        }

        let grid_from = flip_w_tile(&grid_from);
        convert_book.insert(
            convert_tile_to_string(&grid_from, 0, 0, get_size(&grid_from)),
            grid_to.clone(),
        );
        for ro in [
            RotateOrientation::C90,
            RotateOrientation::C180,
            RotateOrientation::C270,
        ] {
            let grid_from = rotate_tile(&grid_from, ro);
            convert_book.insert(
                convert_tile_to_string(&grid_from, 0, 0, get_size(&grid_from)),
                grid_to.clone(),
            );
        }
    }

    let mut grid = vec![
        vec![b'.', b'#', b'.'],
        vec![b'.', b'.', b'#'],
        vec![b'#', b'#', b'#'],
    ];

    for _ in 0..iterations {
        let size = get_size(&grid);
        if size % 2 == 0 {
            grid = enhance(grid, 2, &convert_book);
        } else {
            grid = enhance(grid, 3, &convert_book);
        }
    }

    grid.iter().fold(0usize, |acc, row| {
        acc + row
            .iter()
            .fold(0usize, |a, c| if *c == b'#' { a + 1 } else { a })
    })
}

pub fn run(input: &str) {
    println!("{}", count_on(input, 5));
    println!("{}", count_on(input, 18));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
        assert_eq!(count_on(data, 2), 12);
    }
}

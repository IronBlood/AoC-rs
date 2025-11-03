#[derive(Copy, Clone, PartialEq)]
pub enum RotateOrientation {
    /// CLOCKWISE 90
    C90,
    /// CLOCKWISE 180
    C180,
    /// CLOCKWISE 270
    C270,
}

pub fn rotate_tile<T: Copy>(tile: &[Vec<T>], ro: RotateOrientation) -> Vec<Vec<T>> {
    let h = tile.len();
    let w = tile[0].len();

    debug_assert!(tile.iter().all(|row| row.len() == w));

    let mut new_tile: Vec<Vec<T>>;
    let init = tile[0][0];

    match ro {
        RotateOrientation::C90 => {
            new_tile = vec![vec![init; h]; w];
            for i in 0..h {
                for j in 0..w {
                    new_tile[j][h - 1 - i] = tile[i][j];
                }
            }
        }
        RotateOrientation::C180 => {
            new_tile = vec![vec![init; w]; h];
            for i in 0..h {
                for j in 0..w {
                    new_tile[h - 1 - i][w - 1 - j] = tile[i][j];
                }
            }
        }
        RotateOrientation::C270 => {
            new_tile = vec![vec![init; h]; w];
            for i in 0..h {
                for j in 0..w {
                    new_tile[w - 1 - j][i] = tile[i][j];
                }
            }
        }
    }

    new_tile
}

pub fn flip_w_tile<T: Copy>(tile: &[Vec<T>]) -> Vec<Vec<T>> {
    tile.iter()
        .map(|row| row.iter().copied().rev().collect())
        .collect()
}

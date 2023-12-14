#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;

    Ok(io::BufReader::new(file)
        .lines()
        .collect::<std::io::Result<Vec<String>>>()?)
}

pub fn transpose<T: Copy>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = vec.len();
    let cols = vec[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| vec[row][col]).collect())
        .collect()
}

pub fn rotate<T: Copy>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = vec.len();
    let cols = vec[0].len();

    let mut rotated: Vec<_> = Vec::new();

    for col in 0..cols {
        rotated.push(Vec::new());
        for row in (0..rows).rev() {
            rotated[col].push(vec[row][col]);
        }
    }

    rotated
}

#[cfg(test)]
mod test {
    use super::{rotate, transpose};

    #[test]
    fn test_transpose() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let v = transpose(&v);
        let expected = vec![vec![1, 3], vec![2, 4]];

        for (g, e) in v.iter().zip(expected.iter()) {
            assert_eq!(g, e)
        }
    }

    #[test]
    fn test_rotate() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v = rotate(&v);
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        for (g, e) in v.iter().zip(expected.iter()) {
            assert_eq!(g, e)
        }
    }
}

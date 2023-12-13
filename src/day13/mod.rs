#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day13 {}

impl Solution for Day13 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        let mut acc: i32 = 0;
        for pat in read(path)? {
            let transposed = transpose(&pat);

            let row: i32 = (0..pat.len() - 1)
                .map(|i| i as i32)
                .find(|i| reflects_part_one(&pat, *i))
                .unwrap_or(-1);
            acc += (row + 1) * 100;

            let col: i32 = (0..pat[0].len() - 1)
                .map(|i| i as i32)
                .find(|i| reflects_part_one(&transposed, *i))
                .unwrap_or(-1);
            acc += col + 1;
        }

        println!("{acc}");

        Ok(())
    }

    fn second(&self, path: &str) -> anyhow::Result<()> {
        let mut acc: i32 = 0;
        for pat in read(path)?.into_iter() {
            let transposed = transpose(&pat);

            let row: i32 = (0..pat.len() - 1)
                .map(|i| i as i32)
                .find(|i| reflects_part_two(&pat, *i))
                .unwrap_or(-1);
            acc += (row + 1) * 100;

            let col: i32 = (0..pat[0].len() - 1)
                .map(|i| i as i32)
                .find(|i| reflects_part_two(&transposed, *i))
                .unwrap_or(-1);
            acc += col + 1;
        }

        println!("{acc}");

        Ok(())
    }
}

fn transpose(pat: &Pattern) -> Pattern {
    let m: usize = pat.len();
    let n: usize = pat[0].len();

    let mut transposed: Pattern = vec![vec!['.'; m]; n];

    for (y, row) in pat.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            transposed[x][y] = val
        }
    }

    transposed
}

fn reflects_part_one(pat: &Pattern, i: i32) -> bool {
    for col in 0..pat[0].len() {
        for row in 0..pat.len() {
            let second_row = i * 2 + 1 - row as i32;
            if (second_row < 0) || (second_row >= row as i32) {
                continue;
            }

            if pat[row][col] != pat[second_row as usize][col] {
                return false;
            }
        }
    }

    true
}

fn reflects_part_two(pat: &Pattern, i: i32) -> bool {
    let mut acc: usize = 0;

    for col in 0..pat[0].len() {
        for row in 0..pat.len() {
            let second_row = i * 2 + 1 - row as i32;
            if (second_row < 0) || (second_row >= row as i32) {
                continue;
            }

            if pat[row][col] != pat[second_row as usize][col] {
                acc += 1
            }
        }
    }

    acc == 1
}

type Pattern = Vec<Vec<char>>;

fn read(path: &str) -> Result<Vec<Pattern>> {
    let contents = std::fs::read_to_string(path)?;
    let patterns = contents
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .collect_vec();

    Ok(patterns)
}

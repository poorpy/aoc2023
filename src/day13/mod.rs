#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day13 {}

impl Solution for Day13 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        let mut acc: usize = 0;
        for pat in read(path)? {
            let ver = vertical(&pat);
            let hor = horizontal(&pat);
            acc += ver + 100 * hor;
        }

        println!("{acc}");

        Ok(())
    }

    fn second(&self, path: &str) -> anyhow::Result<()> {
        unimplemented!("{path}")
    }
}

fn vertical(pat: &Pattern) -> usize {
    for pivot in 1..pat[0].len() {
        let mirrors = pat
            .iter()
            .map(|row| {
                let left = row[0..pivot].iter().rev();
                let right = &row[pivot..];
                left.zip(right).map(|(a, b)| a == b).all(|x| x)
            })
            .all(|x| x);

        if mirrors {
            return pivot;
        }
    }

    0
}

fn horizontal(pat: &Pattern) -> usize {
    for pivot in 1..pat.len() {
        let top = pat[0..pivot].iter().rev();
        let bottom = &pat[pivot..];

        if top.zip(bottom).map(|(a, b)| a == b).all(|x| x) {
            return pivot;
        }
    }

    0
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

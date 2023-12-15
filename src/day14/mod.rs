#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

use crate::{solution::Solution, util};

pub struct Day14 {}

impl Solution for Day14 {
    fn first(&self, path: &str) -> Result<()> {
        let platform = read(path)?;
        let platform = util::rotate(&platform);
        let load = calculate(&platform);

        println!("{load}");

        Ok(())
    }

    fn second(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}

fn calculate(platform: &Platform) -> usize {
    platform
        .iter()
        .map(|row| {
            let mut acc: usize = 0;
            let mut rank: usize = row.len();
            for (index, stone) in row.iter().enumerate().rev() {
                match stone {
                    Rock::Round => {
                        acc += rank;
                        rank -= 1;
                    }
                    Rock::Cube => {
                        rank = index;
                    }
                    Rock::None => continue,
                }
            }
            acc
        })
        .sum()
}

enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
    None,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => Self::None,
        }
    }
}

type Platform = Vec<Vec<Rock>>;

fn read(path: &str) -> Result<Platform> {
    let mut platform: Platform = Vec::new();

    for line in util::read(path)? {
        let row = line.chars().map(Rock::from).collect_vec();
        platform.push(row);
    }

    Ok(platform)
}

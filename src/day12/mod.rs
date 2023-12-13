#![allow(dead_code)]

use crate::{solution::Solution, util};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day12 {}

impl Solution for Day12 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(_path: &str) -> Result<()> {
    Ok(())
}

fn second(_path: &str) -> Result<()> {
    Ok(())
}

fn solve(row: &Row) -> usize {
    let mut intervals: Vec<(usize, Spring)> = Vec::new();

    let mut current = row.springs[0];
    let mut count = 0;
    for &spring in &row.springs {
        if spring == current {
            count += 1;
            continue;
        }

        intervals.push((count, current));
        current = spring;
        count = 1;
    }

    0
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Working,
            '#' => Self::Broken,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    records: Vec<usize>,
}

fn read(path: &str) -> Result<Vec<Row>> {
    let mut res = Vec::new();
    for line in util::read(path)? {
        let (springs, records) = line
            .split_once(' ')
            .ok_or(anyhow!("malformed line: {line}"))?;

        let springs = springs.chars().map(Spring::from).collect_vec();
        let records = records
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        res.push(Row { springs, records })
    }

    Ok(res)
}

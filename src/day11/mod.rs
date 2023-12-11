#![allow(dead_code)]

use std::cmp::{max, min};
use std::collections::HashSet;

use crate::{solution::Solution, util};
use anyhow::Result;
use itertools::Itertools;

#[cfg(test)]
mod tests;

pub struct Day11 {}

impl Solution for Day11 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let map = expand(read(path)?);
    let galaxies = galaxies(&map);

    let mut bests: Vec<usize> = Vec::new();

    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            let y = max(galaxies[i].0, galaxies[j].0) - min(galaxies[i].0, galaxies[j].0);
            let x = max(galaxies[i].1, galaxies[j].1) - min(galaxies[i].1, galaxies[j].1);
            bests.push(x + y);
        }
    }

    println!("{}", bests.iter().sum::<usize>());

    Ok(())
}

fn second(path: &str) -> Result<()> {
    println!("{path}");
    Ok(())
}

fn galaxies(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(y, x)| map[y][x] == '#')
        .collect_vec()
}

fn expand(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rows: HashSet<usize> = HashSet::new();
    for (id, row) in map.iter().enumerate() {
        if row.iter().filter(|&&c| c == '#').count() == 0 {
            rows.insert(id);
        }
    }

    let mut columns: HashSet<usize> = HashSet::new();
    'cl: for column in 0..map[0].len() {
        for row in map.iter() {
            if row[column] == '#' {
                continue 'cl;
            }
        }

        columns.insert(column);
    }

    let mut expanded: Vec<Vec<char>> = Vec::new();
    for (row_id, row) in map.into_iter().enumerate() {
        expanded.push(Vec::new());

        for (col_id, &col) in row.iter().enumerate() {
            if columns.contains(&col_id) {
                expanded.last_mut().unwrap().push(col); // push additional copy
            }
            expanded.last_mut().unwrap().push(col);
        }

        if rows.contains(&row_id) {
            expanded.push(expanded.last().unwrap().clone());
        }
    }

    expanded
}

fn read(path: &str) -> Result<Vec<Vec<char>>> {
    let lines = util::read(path)?;
    Ok(lines.iter().map(|l| l.chars().collect_vec()).collect_vec())
}

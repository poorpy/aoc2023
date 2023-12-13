use crate::{solution::Solution, util};
use anyhow::Result;
use itertools::Itertools;
use std::cmp::{max, min};

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
    let map = read(path)?;
    let rows = empty_rows(&map);
    let cols = empty_cols(&map);
    let galaxies = galaxies(&map)
        .iter()
        .map(|&(y, x)| {
            let y_inc = rows.iter().filter(|&&r| r < y).count();
            let x_inc = cols.iter().filter(|&&c| c < x).count();

            (
                (y + 2 * y_inc).saturating_sub(y_inc),
                (x + 2 * x_inc).saturating_sub(x_inc),
            )
        })
        .collect_vec();

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
    let map = read(path)?;
    let rows = empty_rows(&map);
    let cols = empty_cols(&map);
    let galaxies = galaxies(&map)
        .iter()
        .map(|&(y, x)| {
            let y_inc = rows.iter().filter(|&&r| r < y).count();
            let x_inc = cols.iter().filter(|&&c| c < x).count();

            (
                (y + 1_000_000 * y_inc).saturating_sub(y_inc),
                (x + 1_000_000 * x_inc).saturating_sub(x_inc),
            )
        })
        .collect_vec();

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

fn galaxies(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(y, x)| map[y][x] == '#')
        .collect_vec()
}

fn empty_rows(map: &[Vec<char>]) -> Vec<usize> {
    let mut rows: Vec<usize> = Vec::new();
    for (id, row) in map.iter().enumerate() {
        if row.iter().filter(|&&c| c == '#').count() == 0 {
            rows.push(id);
        }
    }

    rows
}

fn empty_cols(map: &[Vec<char>]) -> Vec<usize> {
    let mut columns: Vec<usize> = Vec::new();
    'cl: for column in 0..map[0].len() {
        for row in map.iter() {
            if row[column] == '#' {
                continue 'cl;
            }
        }

        columns.push(column);
    }

    columns
}

fn read(path: &str) -> Result<Vec<Vec<char>>> {
    let lines = util::read(path)?;
    Ok(lines.iter().map(|l| l.chars().collect_vec()).collect_vec())
}

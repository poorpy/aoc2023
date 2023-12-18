#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::{solution::Solution, util};

use self::dig::DigRow;

mod dig;

pub struct Day18 {}

impl Solution for Day18 {
    fn first(&self, path: &str) -> Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let mut digs: Vec<DigRow> = Vec::new();
    for line in util::read(path)? {
        digs.push(DigRow::try_from(line.as_str())?);
    }

    let points = points(&digs);
    let enclosed = enclosed(&points);
    let area = enclosed as usize + (points.len() / 2) + 1;

    println!("{area}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let mut digs: Vec<DigRow> = Vec::new();
    for line in util::read(path)? {
        digs.push(DigRow::try_from(line.as_str())?.second());
    }

    let points = points(&digs);
    let enclosed = enclosed(&points);
    let area = enclosed as usize + (points.len() / 2) + 1;

    println!("{area}");

    Ok(())
}

fn enclosed(points: &[Point]) -> isize {
    let area: isize = points
        .iter()
        .zip(points[1..].iter())
        .map(|(a, b)| {
            let y = a.row + b.row;
            let x = a.col - b.col;

            x * y
        })
        .sum::<isize>()
        .abs();

    area / 2
}

#[derive(Debug, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

fn points(digs: &[DigRow]) -> Vec<Point> {
    let mut res: Vec<Point> = vec![Point { row: 0, col: 0 }];

    for dig in digs {
        let (row_d, col_d) = dig.dir.dir();
        for _ in 0..dig.count {
            let prev = res.last().unwrap();
            res.push(Point {
                row: prev.row + row_d,
                col: prev.col + col_d,
            });
        }
    }

    res[..res.len() - 1].to_vec()
}

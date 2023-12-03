use std::path::Path;

use anyhow::Result;

use super::solution::Solution;
use super::util::read;
use gear::Gear;
use number::{numbers, Number};

mod gear;
mod number;

pub struct Day03 {}

impl Solution for Day03 {
    fn first(&self, path: &str) -> Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> Result<()> {
        second(path)
    }
}

fn first(path: impl AsRef<Path>) -> Result<()> {
    let lines = read(path)?;
    let mut res: Vec<Number> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let nums = numbers(line, i, lines.len() - 1);
        res.extend(nums?);
    }

    let matrix = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let res: usize = res
        .iter()
        .filter(|n| n.borders_symbol(&matrix))
        .map(|n| n.num)
        .sum();

    println!("{res}");

    Ok(())
}

fn second(path: impl AsRef<Path>) -> Result<()> {
    let lines = read(path)?;
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let nums = number::numbers(line, i, lines.len() - 1);
        numbers.extend(nums?);
        gears.extend(gear::gears(line, i, lines.len()));
    }

    let res: usize = gears.iter().filter_map(|g| g.gear_ratio(&numbers)).sum();

    println!("{res}");

    Ok(())
}

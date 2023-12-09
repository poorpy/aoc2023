use crate::{solution::Solution, util};
use anyhow::Result;
use itertools::Itertools;

pub struct Day09 {}

impl Solution for Day09 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let lines = util::read(path)?;
    let res: i32 = lines
        .iter()
        .map(|s| parse(s))
        .map(|v| extrapolate(&v))
        .sum();

    println!("{res}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let lines = util::read(path)?;
    let res: i32 = lines
        .iter()
        .map(|s| parse(s).into_iter().rev().collect_vec())
        .map(|v| extrapolate(&v))
        .sum();

    println!("{res}");

    Ok(())
}

fn parse(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn extrapolate(data: &[i32]) -> i32 {
    if data.iter().all(|&d| d == 0) {
        return 0;
    }

    let diff = data.windows(2).map(|w| w[1] - w[0]).collect_vec();

    data.last().unwrap() + extrapolate(&diff)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[1, 1, 1, 1], 1)]
    #[case(&[0, 3, 6, 9, 12, 15], 18)]
    fn test_extrapolate(#[case] input: &[i32], #[case] expected: i32) {
        assert_eq!(extrapolate(input), expected)
    }
}

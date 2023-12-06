use std::num::ParseIntError;

use crate::{solution::Solution, util::read};
use anyhow::{anyhow, Result};
use thiserror::Error;

pub struct Day06 {}

impl Solution for Day06 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let lines = read(path)?;
    let (_, times) = lines[0]
        .split_once(": ")
        .ok_or(anyhow!("missing times delimiter"))?;

    let times = times
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let (_, records) = lines[1]
        .split_once(": ")
        .ok_or(anyhow!("missing records delimiter"))?;

    let records = records
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let races = times
        .into_iter()
        .zip(records.into_iter())
        .map(|pair: (&str, &str)| Race::try_from(pair))
        .collect::<std::result::Result<Vec<Race>, RaceError>>()?;

    let res: usize = races.iter().map(|r| r.solve()).product();

    println!("{res}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let lines = read(path)?;
    let (_, times) = lines[0]
        .split_once(": ")
        .ok_or(anyhow!("missing times delimiter"))?;

    let time = times.replace(' ', "").parse::<usize>()?;

    let (_, records) = lines[1]
        .split_once(": ")
        .ok_or(anyhow!("missing records delimiter"))?;

    let record = records.replace(' ', "").parse::<usize>()?;

    let race = Race { time, record };

    println!("{}", race.solve());

    Ok(())
}

#[derive(Debug, Error)]
pub enum RaceError {
    #[error("failed to parse value")]
    RaceParseError(#[from] ParseIntError),
}

struct Race {
    time: usize,
    record: usize,
}

impl TryFrom<(&str, &str)> for Race {
    type Error = RaceError;
    fn try_from((time, record): (&str, &str)) -> std::result::Result<Self, Self::Error> {
        let time = time.parse::<usize>()?;
        let record = record.parse::<usize>()?;

        Ok(Race { time, record })
    }
}

impl Race {
    fn solve(&self) -> usize {
        (0..self.time)
            .filter(|x: &usize| (self.time - x) * x > self.record)
            .count()
    }
}

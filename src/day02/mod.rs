use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::{anyhow, Result};

use super::solution::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn first(&self, path: &str) -> Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> Result<()> {
        second(path)
    }
}

pub fn first(path: impl AsRef<Path>) -> Result<()> {
    let res: usize = read(path)?
        .iter()
        .map(|line| parse(line))
        .collect::<Result<Vec<Game>>>()?
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
        })
        .map(|g| g.id)
        .sum();

    println!("{res}");

    Ok(())
}

pub fn second(path: impl AsRef<Path>) -> Result<()> {
    let res: usize = read(path)?
        .iter()
        .map(|line| parse(line))
        .collect::<Result<Vec<Game>>>()?
        .iter()
        .map(|g| power(&g.sets))
        .sum();

    println!("{res}");

    Ok(())
}

fn power(sets: &Vec<Set>) -> usize {
    let mut power = Set {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in sets {
        power.red = std::cmp::max(set.red, power.red);
        power.green = std::cmp::max(set.green, power.green);
        power.blue = std::cmp::max(set.blue, power.blue);
    }

    power.red * power.green * power.blue
}

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

fn parse_set(set: &str) -> Result<Set> {
    let mut balls = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    for ball in set.split(",") {
        let info = ball
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let count = info[0].parse::<usize>()?;
        match info[1] {
            "red" => balls.red += count,
            "green" => balls.green += count,
            "blue" => balls.blue += count,
            _ => unreachable!(),
        }
    }
    Ok(balls)
}

fn parse(line: &str) -> Result<Game> {
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
    };

    if let Some((id, sets)) = line.split_once(":") {
        if let Some(id) = id.split_whitespace().nth(1) {
            game.id = id.parse::<usize>()?;
        }

        for set in sets.split(";") {
            game.sets.push(parse_set(set)?);
        }

        return Ok(game);
    }

    Err(anyhow!("failed to split line: {:?}", line))
}

fn read(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

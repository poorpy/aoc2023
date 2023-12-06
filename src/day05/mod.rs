use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use self::mapping::{parse_segment, Map};
use crate::solution::Solution;

mod mapping;

pub struct Day05 {}

impl Solution for Day05 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let (seeds, cats) = read(path)?;

    let mut res: BTreeSet<usize> = BTreeSet::new();

    for seed in seeds {
        let mut val = seed;
        for category in cats.iter() {
            val = map_to_category(val, category);
        }
        res.insert(val);
    }

    println!("{}", res.first().unwrap());

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let (seeds, cats) = read(path)?;

    let mut res: usize = std::usize::MAX;
    for (start, len) in seeds.into_iter().tuples() {
        for seed in start..start + len {
            let mut val = seed;
            for category in cats.iter() {
                val = map_to_category(val, category);
            }
            res = std::cmp::min(val, res)
        }
    }

    println!("{res}");

    Ok(())
}

fn map_to_category(value: usize, tree: &BTreeMap<usize, Map>) -> usize {
    match tree.range(..=value).last() {
        None => value, // there is no range that could inlude this value
        Some((_, map)) => {
            if map.source <= value && value < map.source + map.len {
                return map.destination + (value - map.source);
            }

            value // not in range so 1:1 corespondence
        }
    }
}

type Categories = Vec<BTreeMap<usize, Map>>;

fn read(path: impl AsRef<Path>) -> Result<(Vec<usize>, Categories)> {
    let contents = std::fs::read_to_string(path)?;
    let contents = contents.split("\n\n").collect::<Vec<&str>>();

    let (_, seeds) = contents[0]
        .split_once(": ")
        .ok_or(anyhow!("malformed seeds line"))?;

    let seeds: Vec<usize> = seeds
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let categories = contents[1..]
        .iter()
        .map(|c| parse_segment(c))
        .collect::<Result<Categories>>()?;

    Ok((seeds, categories))
}

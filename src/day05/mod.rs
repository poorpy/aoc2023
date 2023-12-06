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

    let mut res: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (start, end) in seeds.into_iter().tuples().map(|(s, l)| (s, s + l)) {
        let mut val = start;
        for category in cats.iter() {
            val = map_to_category(val, category);
        }
        res.insert(val, (start, end));
    }

    println!("{:?}", res);
    loop {
        let (loc, (start, len)) = res.first_key_value().unwrap();
        let loc = *loc;
        let left: (usize, usize) = (*start, len / 2);
        let left_location = location(left.0 + left.1, cats.iter());

        let right: (usize, usize) = (start + len / 2, len / 2);
        let right_location = location(right.0, cats.iter());

        if left_location == loc || right_location == loc {
            break;
        }

        res.insert(right_location, right);
        res.insert(loc, left);

        println!("{:?}", res);
    }

    // // take best score split in two and run again
    // let (score, (start, len)) = res.first_key_value().ok_or(anyhow!("empty result map"))?;
    //
    println!("{:?}", res);

    Ok(())
}

fn location<'a>(value: usize, trees: impl Iterator<Item = &'a BTreeMap<usize, Map>>) -> usize {
    let mut val = value;
    for tree in trees {
        val = map_to_category(val, tree);
    }
    val
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

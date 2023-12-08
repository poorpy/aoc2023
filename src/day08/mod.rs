use std::collections::HashMap;

use crate::solution::Solution;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use num::integer::lcm;

pub struct Day08 {}

impl Solution for Day08 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let (commands, nodes) = read(path)?;
    let mut graph: HashMap<String, (String, String)> = HashMap::new();

    for node in nodes {
        let (key, value) = parse(&node)?;
        graph.insert(key, value);
    }

    let count = solve("AAA", "ZZZ", &commands, &graph);

    println!("{count}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let (commands, nodes) = read(path)?;
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut starting: Vec<String> = Vec::new();

    for node in nodes {
        let (key, value) = parse(&node)?;
        graph.insert(key.clone(), value);

        if key.ends_with('A') {
            starting.push(key);
        }
    }

    let count: usize = starting
        .iter()
        .map(|start| solve(start, "Z", &commands, &graph))
        .reduce(lcm)
        .ok_or(anyhow!("empty iterator"))?;

    println!("{count}");

    Ok(())
}

fn solve(
    start: &str,
    end: &str,
    commands: &str,
    graph: &HashMap<String, (String, String)>,
) -> usize {
    let mut count: usize = 0;
    let mut current: &str = start;
    for c in commands.chars().cycle() {
        if c == 'L' {
            current = &graph[current].0;
        } else {
            current = &graph[current].1;
        }

        count += 1;
        if current.ends_with(end) {
            break;
        }
    }

    count
}

fn parse(line: &str) -> Result<(String, (String, String))> {
    let (index, left, right) = line
        .chars()
        .filter(|c| c.is_alphanumeric())
        .chunks(3)
        .into_iter()
        .map(|c| String::from_iter(c))
        .collect_tuple()
        .ok_or(anyhow!("missing input in line: {line}"))?;

    Ok((index, (left, right)))
}

fn read(path: &str) -> Result<(String, Vec<String>)> {
    let file = std::fs::read_to_string(path)?;
    let (commands, graph) = file
        .split_once("\n\n")
        .ok_or(anyhow!("malformed input file"))?;

    let graph = graph
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    Ok((commands.to_owned(), graph))
}

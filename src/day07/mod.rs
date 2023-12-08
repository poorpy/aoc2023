use crate::{solution::Solution, util};
use anyhow::Result;
use itertools::Itertools;

use self::hand::Hand;

mod hand;
mod hand_type;

pub struct Day07 {}

impl Solution for Day07 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let lines = util::read(path)?;
    let mut res: Vec<Hand> = Vec::new();
    for line in lines {
        res.push(hand::parse(
            &line,
            &hand::card_value_part_one,
            &hand_type::parse_part_one,
        )?);
    }
    res.sort();

    let res: usize = res.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum();

    println!("{res}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let lines = util::read(path)?;
    let mut res: Vec<Hand> = Vec::new();
    for line in lines {
        res.push(hand::parse(
            &line,
            &hand::card_value_part_two,
            &hand_type::parse_part_two,
        )?);
    }
    res.sort();

    println!(
        "{:#?}",
        res.iter()
            .map(|h| format!("{{ \"kind\": \"{:?}\", \"bid\": {}}}", h.hand_type, h.bid))
            .collect_vec()
    );

    let res: usize = res.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum();

    println!("{res}");

    Ok(())
}

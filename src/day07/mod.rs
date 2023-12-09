#![allow(dead_code)]

use crate::{solution::Solution, util};
use anyhow::{anyhow, Result};
use itertools::Itertools;

use self::card::{Card, Kind};

mod card;

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
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|l| parse(l, false))
        .collect::<Result<Vec<Hand>>>()?;
    hands.sort();

    let res: usize = hands.iter().enumerate().map(|(i, e)| e.bid * (i + 1)).sum();

    println!("{res}");

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let lines = util::read(path)?;
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|l| parse(l, true))
        .collect::<Result<Vec<Hand>>>()?;
    hands.sort();

    let res: usize = hands.iter().enumerate().map(|(i, e)| e.bid * (i + 1)).sum();

    println!("{res}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    pub cards: Vec<Card>,
    pub bid: usize,
    pub kind: Kind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.kind.cmp(&other.kind);
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }

        for i in 0..5 {
            if self.cards[i] == other.cards[i] {
                continue;
            }

            return self.cards[i].cmp(&other.cards[i]);
        }

        std::cmp::Ordering::Equal
    }
}

fn parse(line: &str, part_two: bool) -> Result<Hand> {
    let (cards, bid) = line
        .split_once(' ')
        .ok_or(anyhow!("malformed line: {line}"))?;

    let bid = bid.parse::<usize>()?;
    let mut cards = cards.chars().map(Card::from).collect_vec();

    if part_two {
        cards.iter_mut().for_each(|card| {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        })
    }

    let kind = Kind::from(cards.as_slice());

    Ok(Hand { cards, bid, kind })
}

use std::num::ParseIntError;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use thiserror::Error;

use super::hand_type::HandType;

#[derive(Debug, Error)]
pub enum HandError {
    #[error("failed to parse bid")]
    BidParseError(#[from] ParseIntError),

    #[error("failed to parse input")]
    HandParseError(#[from] anyhow::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<usize>,
    pub bid: usize,
    pub hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        }
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
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

pub fn card_value_part_one(c: char) -> usize {
    if c.is_ascii_digit() {
        return usize::try_from(c.to_digit(10).unwrap()).unwrap();
    }

    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => 0,
    }
}

pub fn card_value_part_two(c: char) -> usize {
    if c.is_ascii_digit() {
        return usize::try_from(c.to_digit(10).unwrap()).unwrap();
    }

    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => 0,
    }
}

pub fn parse(
    value: &str,
    value_fn: &dyn Fn(char) -> usize,
    type_fn: &dyn Fn(&str) -> HandType,
) -> Result<Hand> {
    let (cards, bid) = value
        .split_once(' ')
        .ok_or(anyhow!("missing line delimiter"))?;

    let bid = bid.parse::<usize>()?;
    let hand_type = type_fn(cards);
    let cards = cards.chars().map(value_fn).collect_vec();

    Ok(Hand {
        cards,
        bid,
        hand_type,
    })
}

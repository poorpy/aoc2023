use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use anyhow::Result;

use crate::{solution::Solution, util};

pub struct Day04 {}

impl Solution for Day04 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }
    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

type Id = usize;

struct Card {
    id: Id,
    winning: HashSet<usize>,
    given: HashSet<usize>,
}

fn card(line: &str) -> Result<Card> {
    let (id, numbers) = line
        .split_once(':')
        .ok_or(anyhow::anyhow!("missing ':' in line: {line}"))?;

    let id = id
        .split_whitespace()
        .last()
        .ok_or(anyhow::anyhow!("malformed id part "))?
        .parse::<usize>()?;

    let (winning, given) = numbers
        .split_once('|')
        .ok_or(anyhow::anyhow!("missing '|' delimiter"))?;

    let winning: HashSet<usize> = winning
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let given: HashSet<usize> = given
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    Ok(Card { id, winning, given })
}

fn first(path: impl AsRef<Path>) -> Result<()> {
    let mut acc: usize = 0;
    for card in util::read(path)?.iter().map(|l| card(l)) {
        let card = card?;

        let won = card.winning.intersection(&card.given).count();
        if won > 0 {
            acc += 1 << (won - 1);
        }
    }

    println!("{acc}");

    Ok(())
}

fn second(path: impl AsRef<Path>) -> Result<()> {
    let cards = util::read(path)?
        .iter()
        .map(|l| card(l))
        .collect::<Result<Vec<Card>>>()?;

    let mut mem: HashMap<Id, &[Card]> = HashMap::new();
    let mut refs = cards.iter().collect::<Vec<_>>();

    for card in refs.iter() {
        let won = card.winning.intersection(&card.given).count();
        mem.insert(card.id, &cards[card.id..card.id + won]);
    }

    let mut index = 0;
    while index < refs.len() {
        let won = mem.get(&refs[index].id).unwrap();
        refs.extend(won.iter());
        index += 1;
    }

    println!("{index}");

    Ok(())
}

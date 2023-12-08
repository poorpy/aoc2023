use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

pub fn parse_part_one(value: &str) -> HandType {
    let mut map: HashMap<char, usize> = HashMap::new();
    for char in value.chars() {
        *map.entry(char).or_insert(0) += 1;
    }

    let mut values = map.values().copied().collect_vec();
    values.sort();

    match values.as_slice() {
        [5] => HandType::Five,
        [1, 4] => HandType::Four,
        [2, 3] => HandType::Full,
        [1, 1, 3] => HandType::Three,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

pub fn parse_part_two(value: &str) -> HandType {
    let mut map: HashMap<char, usize> = HashMap::new();
    for char in value.chars() {
        *map.entry(char).or_insert(0) += 1;
    }

    let jokers = map.get(&'J').copied().unwrap_or(0);
    let mut values = map.values().copied().collect_vec();
    values.sort();

    match values.as_slice() {
        [5] => HandType::Five,
        [1, 4] if jokers > 0 => HandType::Five,
        [1, 4] => HandType::Four,
        [2, 3] if jokers > 0 => HandType::Five,
        [2, 3] => HandType::Full,
        [1, 1, 3] if jokers > 0 => HandType::Four,
        [1, 1, 3] => HandType::Three,
        [1, 2, 2] if jokers == 2 => HandType::Four,
        [1, 2, 2] if jokers == 1 => HandType::Full,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] if jokers > 0 => HandType::Three,
        [1, 1, 1, 2] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

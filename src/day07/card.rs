use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Card {
    Joker,
    Num(usize),
    Tower,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Tower,
            n => Self::Num(n.to_digit(10).unwrap() as usize),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<&[Card]> for Kind {
    fn from(value: &[Card]) -> Self {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for card in value {
            *map.entry(*card).or_insert(0) += 1;
        }

        let mut jokers: usize = 0;
        let mut values = Vec::new();
        for (k, v) in map {
            if k == Card::Joker {
                jokers = v;
                continue;
            }

            values.push((k, v));
        }

        if jokers == 5 {
            return Self::Five;
        }

        values.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        values.reverse();
        values[0].1 += jokers;

        match values[0].1 {
            5 => Self::Five,
            4 => Self::Four,
            3 => match values[1].1 {
                2 => Self::FullHouse,
                _ => Self::Three,
            },
            2 => match values[1].1 {
                2 => Self::TwoPair,
                _ => Self::OnePair,
            },
            _ => Self::HighCard,
        }
    }
}

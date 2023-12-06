use std::collections::BTreeMap;
use std::num::ParseIntError;

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("failed to parse range")]
    MappingParseError(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub destination: usize,
    pub source: usize,
    pub len: usize,
}

impl TryFrom<&str> for Map {
    type Error = MappingError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut values: Vec<usize> = Vec::new();
        for value in value.split_whitespace() {
            let value = value.parse::<usize>()?;
            values.push(value);
        }

        Ok(Map {
            destination: values[0],
            source: values[1],
            len: values[2],
        })
    }
}

pub fn parse_segment(input: &str) -> Result<BTreeMap<usize, Map>> {
    let mut res: BTreeMap<usize, Map> = BTreeMap::new();
    for line in input.split('\n') {
        // skip header if present
        if line.ends_with("map:") || line.is_empty() {
            continue;
        }

        let map = Map::try_from(line)?;
        res.insert(map.source, map);
    }

    Ok(res)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parsing_map() {
        assert_eq!(
            Map::try_from("21 37 420").unwrap(),
            Map {
                destination: 21,
                source: 37,
                len: 420,
            }
        )
    }
}

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "3" | "U" => Self::Up,
            "1" | "D" => Self::Down,
            "2" | "L" => Self::Left,
            _ => Self::Right,
        }
    }
}

impl Direction {
    pub fn dir(&self) -> (isize, isize) {
        match self {
            Self::Up => (1, 0),
            Self::Down => (-1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug)]
pub struct DigRow {
    pub dir: Direction,
    pub count: usize,
    pub color: String,
}

impl TryFrom<&str> for DigRow {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let (dir, count, color) = line
            .splitn(3, ' ')
            .collect_tuple()
            .ok_or(anyhow!("malformed line: {line}"))?;

        let dir = Direction::from(dir);
        let count = count.parse::<usize>()?;
        let color = color.to_string();

        Ok(DigRow { dir, count, color })
    }
}

impl DigRow {
    pub fn second(&self) -> DigRow {
        let color = self
            .color
            .trim_matches(|c| c == '(' || c == ')' || c == '#');
        let count = usize::from_str_radix(&color[..5], 16).unwrap();
        let dir = Direction::from(&color[5..]);

        DigRow {
            dir,
            count,
            color: "".to_string(),
        }
    }
}

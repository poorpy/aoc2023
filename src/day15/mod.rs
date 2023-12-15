#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day15 {}

impl Solution for Day15 {
    fn first(&self, path: &str) -> Result<()> {
        let sum: usize = std::fs::read_to_string(path)?
            .strip_suffix('\n')
            .unwrap()
            .split(',')
            .map(hash)
            .sum();

        println!("{sum}");

        Ok(())
    }

    fn second(&self, path: &str) -> Result<()> {
        let steps = std::fs::read_to_string(path)?
            .strip_suffix('\n')
            .unwrap()
            .split(',')
            .map(Step::from)
            .collect_vec();

        let mut cells: Vec<Vec<Lens>> = Vec::with_capacity(256);
        for _ in 0..256 {
            cells.push(Vec::new());
        }

        for step in steps {
            let index = hash(&step.label);

            if let Operation::Remove = step.op {
                if let Some(li) = cells[index]
                    .iter()
                    .position(|lens| lens.label == step.label)
                {
                    cells[index].remove(li);
                }
            }

            if let Operation::Adjust(length) = step.op {
                match cells[index]
                    .iter()
                    .position(|lens| lens.label == step.label)
                {
                    Some(li) => cells[index][li].length = length,
                    None => cells[index].push(Lens {
                        label: step.label,
                        length,
                    }),
                }
            }
        }

        let focus: usize = cells
            .iter()
            .map(|cell| {
                cell.iter()
                    .enumerate()
                    .map(|(i, lens)| (i + 1) * lens.length)
                    .sum::<usize>()
            })
            .enumerate()
            .map(|(i, p)| (i + 1) * p)
            .sum();

        println!("{focus}");

        Ok(())
    }
}

fn hash(s: &str) -> usize {
    let mut acc: usize = 0;

    for c in s.chars() {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
    }

    acc
}

struct Lens {
    label: String,
    length: usize,
}

struct Step {
    label: String,
    op: Operation,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        if let Some((label, length)) = value.split_once('=') {
            return Step {
                label: label.to_string(),
                op: Operation::Adjust(length.parse().unwrap()),
            };
        }

        let (label, _) = value.split_once('-').unwrap();
        Step {
            label: label.to_string(),
            op: Operation::Remove,
        }
    }
}

enum Operation {
    Remove,
    Adjust(usize),
}

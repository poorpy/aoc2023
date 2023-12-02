use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::Result;

use super::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn first(&self, path: &str) -> Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> Result<()> {
        second(path)
    }
}

fn first(path: impl AsRef<Path>) -> Result<()> {
    let lines = read(path)?;
    let mut acc: u64 = 0;
    for line in lines {
        let first = line
            .bytes()
            .nth(line.find(|c: char| c.is_digit(10)).unwrap())
            .unwrap() as char;

        let last = line
            .bytes()
            .nth(line.rfind(|c: char| c.is_digit(10)).unwrap())
            .unwrap() as char;

        let res = String::from_iter([first, last]).parse::<u64>().unwrap();
        acc += res
    }

    println!("{acc}");

    Ok(())
}

fn second(path: impl AsRef<Path>) -> Result<()> {
    let mut acc: u64 = 0;

    for line in read(path)? {
        let n = find(&line)?;
        acc += n
    }

    println!("{acc}");

    Ok(())
}

struct Pattern {
    index: usize,
    digit: char,
}

fn find(line: &str) -> Result<u64> {
    let patterns = &[
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut first = Pattern {
        index: line.len(),
        digit: 'a',
    };

    let mut last = Pattern {
        index: 0,
        digit: 'b',
    };

    for (pat, rep) in patterns {
        if let Some(index) = line.find(pat) {
            if index < first.index {
                first.index = index;
                first.digit = *rep;
            }
        }
        if let Some(index) = line.rfind(pat) {
            if index > last.index {
                last.index = index;
                last.digit = *rep;
            }
        }
    }

    let mut first_digit = first.digit;
    if let Some(index) = line.find(|c: char| c.is_digit(10)) {
        if index <= first.index {
            first_digit = line.bytes().nth(index).unwrap() as char;
        }
    }

    let mut last_digit = last.digit;
    if let Some(index) = line.rfind(|c: char| c.is_digit(10)) {
        if index >= last.index {
            last_digit = line.bytes().nth(index).unwrap() as char;
        }
    }

    let res = String::from_iter([first_digit, last_digit])
        .parse::<u64>()
        .unwrap();

    Ok(res)
}

fn read(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

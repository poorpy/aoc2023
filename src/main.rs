use anyhow::Result;
use clap::{Parser, ValueEnum};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod solution;
mod util;

#[derive(Parser)]
struct Cli {
    day: Day,
    part: Part,
    path: String,
}

#[derive(Clone, Copy, ValueEnum)]
enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
}

#[derive(Clone, Copy, ValueEnum)]
enum Part {
    First,
    Second,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let sol: Box<dyn solution::Solution> = match cli.day {
        Day::Day01 => Box::new(day01::Day01 {}),
        Day::Day02 => Box::new(day02::Day02 {}),
        Day::Day03 => Box::new(day03::Day03 {}),
        Day::Day04 => Box::new(day04::Day04 {}),
        Day::Day05 => Box::new(day05::Day05 {}),
        Day::Day06 => Box::new(day06::Day06 {}),
        Day::Day07 => Box::new(day07::Day07 {}),
        Day::Day08 => Box::new(day08::Day08 {}),
    };

    match cli.part {
        Part::First => sol.first(&cli.path)?,
        Part::Second => sol.second(&cli.path)?,
    }

    Ok(())
}

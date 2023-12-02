use anyhow::Result;
use clap::{Parser, ValueEnum};

mod day01;
mod day02;
mod solution;

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
    };

    match cli.part {
        Part::First => sol.first(&cli.path)?,
        Part::Second => sol.second(&cli.path)?,
    }

    Ok(())
}

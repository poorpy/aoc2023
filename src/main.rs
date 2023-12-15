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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
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
    Day09,
    Day10,
    Day11,
    Day13,
    Day14,
    Day15,
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
        Day::Day09 => Box::new(day09::Day09 {}),
        Day::Day10 => Box::new(day10::Day10 {}),
        Day::Day11 => Box::new(day11::Day11 {}),
        Day::Day13 => Box::new(day13::Day13 {}),
        Day::Day14 => Box::new(day14::Day14 {}),
        Day::Day15 => Box::new(day15::Day15 {}),
    };

    match cli.part {
        Part::First => sol.first(&cli.path)?,
        Part::Second => sol.second(&cli.path)?,
    }

    Ok(())
}

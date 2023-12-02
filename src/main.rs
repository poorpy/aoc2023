use anyhow::Result;
use clap::{Parser, Subcommand};

mod day01;
mod day02;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Day,
}

#[derive(Subcommand, Clone)]
enum Day {
    Day01 {
        #[command(subcommand)]
        part: Part,
    },
}

#[derive(Subcommand, Clone)]
enum Part {
    First {
        #[arg(short, long)]
        path: String,
    },
    Second {
        #[arg(short, long)]
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Day::Day01 { part } => {
            if let Part::First { path } = part {
                day01::part_one(&path)?;
            }
            if let Part::Second { path } = part {
                day01::part_two(&path)?;
            }
        }
    }

    Ok(())
}

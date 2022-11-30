use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use std::fs;
use std::path::PathBuf;

mod puzzle;
mod day1;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(long)]
    day: u32,

    // Which part of the day's code to run
    #[arg(long)]
    part: u32,

    input: PathBuf,
}

macro_rules! run_day {
    ($day:ident, $dayty:ty, $part:expr, $input:expr) => {
        {
            let raw_input = fs::read_to_string($input)?;
            let day: $dayty = Puzzle::from_input(&raw_input)?;
            let output = match $part {
                1 => day.solve_part1()?,
                2 => day.solve_part2()?,
                _ => bail!("invalid part"),
            };
            println!("SOLUTION: {}", output);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("RUNNING DAY {} PART {}...", args.day, args.part);

    match args.day {
        1 => run_day!(day1, day1::Day1, args.part, &args.input),
        _ => bail!("day {} not found", args.day),
    }

    Ok(())
}

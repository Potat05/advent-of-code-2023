
mod days;

use std::{error::Error, fs};
use clap::Parser;
use days::Days;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solver to use.
    #[arg(short, long, value_enum)]
    day: Days,

    /// Part to solve.
    #[arg(short, long, default_value_t = 1)]
    part: i32,

    /// Input to feed to solver.
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = fs::read_to_string(args.input)?;

    let solved = args.day.solve(input, args.part);

    println!("Solved: {}", solved);

    Ok(())
}



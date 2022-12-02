use std::error::Error;

use clap::Parser;
use day1::solve_day_1;

mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long, required = true)]
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.day {
        1 => {
            let result = solve_day_1();

            println!("Solution: {result}");
        }
        _ => println!("Couldn't find day {}", args.day),
    }

    Ok(())
}

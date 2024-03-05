//use std::env::args;
use clap::Parser;
use std::path::PathBuf;
use std::fs::read_to_string;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    // The Pattern to look for
    pattern: String,
    // The path to the file to read
    path: PathBuf,
}

//#[derive(Debug)]
//struct CustomError(String);

fn main() -> Result<()>{
    let args = Cli::parse();

    println!("Arguments -> 1st: {:?}, 2nd: {:?}", args.pattern, args.path);

    let content = read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("Line: {}", line);
        }
    }

    Ok(())
    //let first_argument = args().nth(1).expect("No First argument was passed!");
    //let second_argument = args().nth(2).expect("No Second argument was passed!");
}

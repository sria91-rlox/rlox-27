// mod error_types;

use anyhow::Result;
use std::fs;
use std::io::BufRead;

use clap::{AppSettings, Clap};

/// Lox compile and interpreter
#[derive(Clap)]
#[clap(version = "1.0", author = "David Weis <dweis7@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap()]
    path: Option<String>,
}
fn main() -> Result<()> {
    let args: Args = Args::parse();
    match args.path {
        Some(path) => run_file(path)?,
        None => run_prompt()?,
    }
    Ok(())
}

fn run_file(path: String) -> Result<()> {
    let text = fs::read_to_string(path)?;
    run(text)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        run(line?)?;
    }
    Ok(())
}

fn run(text: String) -> Result<()> {
    println!("Hi you said {}", text);

    Ok(())
}

use std::{fs::File, io, path::PathBuf};

use anyhow::{Context as _, Result};
use chumsky::Parser as _;
use clap::Parser as _;

mod cli;
mod parsing;

fn run(file: Option<PathBuf>) -> Result<()> {
    fn read_all(mut input: impl io::Read) -> io::Result<String> {
        let mut result = String::new();
        input.read_to_string(&mut result).map(|_| result)
    }
    let data = if let Some(path) = file {
        let file = File::open(&path).with_context(|| format!("Error opening {:?}", path))?;
        read_all(file).with_context(|| format!("Error reading from {:?}", path))
    } else {
        read_all(io::stdin()).context("Error reading from stdin")
    }?;
    let parser = parsing::parser();
    dbg!(parser.parse(data));
    todo!()
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    run(cli.filepath)
}

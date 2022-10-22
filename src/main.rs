use std::{fs::File, io, path::PathBuf};

use anyhow::{anyhow, Context as _, Result};
use clap::Parser;

mod cli;
mod parse;

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
    let parsed = parse::file(data.as_str()).map_err(|()| anyhow!("Error parsing"))?;
    dbg!(parsed);
    todo!()
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    run(cli.filepath)
}

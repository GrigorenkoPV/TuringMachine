use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(help_expected = true, propagate_version = true)]
pub(crate) struct Cli {
    /// Path to turing machine file (if none, reads from stdin)
    pub(crate) filepath: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}

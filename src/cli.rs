use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(help_expected = true)]
pub struct Cli {
    #[clap(help = "File with the code of the turing machine")]
    pub machine_file: PathBuf,
    #[clap(help = "File to redirect input from")]
    pub input_file: Option<PathBuf>,
    #[clap(help = "Step limit for execution", short, long, default_value_t = 0)]
    pub time_limit: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::IntoApp;
        Cli::command().debug_assert();
    }
}

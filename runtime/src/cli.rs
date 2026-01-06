use crate::error;
use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    path: String,
}

pub fn cli() -> Result<String, error::Error> {
    let cli_args = CliArgs::parse();
    let some_path = std::fs::canonicalize(cli_args.path).unwrap();
    Ok(some_path.to_string_lossy().to_string())
}

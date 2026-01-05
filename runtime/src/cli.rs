use crate::{debug, error, logic};
use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    path: String,
}

pub fn cli() -> Result<(), error::Error> {
    let cli_args = CliArgs::parse();
    let some_path = std::fs::canonicalize(cli_args.path).unwrap();
    debug::info(&format!("{:#?}", some_path));
    logic::test()?;
    Ok(())
}

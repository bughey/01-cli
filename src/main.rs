// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;

use anyhow::Result;
use rcli::{
    opts::{Opts, SubCommand},
    process::process_csv,
};

fn main() -> Result<()> {
    let cli = Opts::parse();

    match cli.cmd {
        SubCommand::Csv(opts) => process_csv(opts)?,
    }

    Ok(())
}

// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;

use anyhow::Result;
use rcli::{
    opts::{Opts, SubCommand},
    process::{csv_convert::process_csv, gen_pass::process_genpass},
};

fn main() -> Result<()> {
    let cli = Opts::parse();

    match cli.cmd {
        SubCommand::Csv(opts) => process_csv(opts)?,
        SubCommand::GenPass(opts) => process_genpass(opts)?,
    }

    Ok(())
}

// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;

use anyhow::Result;
use rcli::{
    cli::{Opts, SubCommand},
    process::{
        base64::process_base64, csv_convert::process_csv, gen_pass::process_genpass,
        http_serve::process_http, text::process_text,
    },
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Opts::parse();

    match cli.cmd {
        SubCommand::Csv(opts) => process_csv(opts)?,
        SubCommand::GenPass(opts) => {
            let pwd = process_genpass(opts)?;
            println!("{}", pwd);

            let estimate = zxcvbn(&pwd, &[]);
            eprintln!("Password strength {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => process_base64(subcmd)?,
        SubCommand::Text(subcmd) => process_text(subcmd)?,
        SubCommand::Http(subcmd) => process_http(subcmd).await?,
    }

    Ok(())
}

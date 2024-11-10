pub mod base64;
pub mod csv;
pub mod genpass;
pub mod text;

use std::path::Path;

use anyhow::Result;
use base64::Base64SubCommand;
use clap::Parser;
use csv::CsvOpts;
use genpass::GenPassOpts;
use text::TextSubCommand;

// rcli csv -i input.csv -o output.json --header -d ','

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign or verify a text")]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found".to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File not found".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File not found".into()));
    }
}

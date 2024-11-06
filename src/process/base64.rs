use anyhow::{Ok, Result};

use crate::cli::base64::Base64SubCommand;

pub fn process_base64(subcmd: Base64SubCommand) -> Result<()> {
    match subcmd {
        Base64SubCommand::Encode(opts) => {
            println!("encode: {:?}", opts);
        }
        Base64SubCommand::Decode(opts) => {
            println!("decode: {:?}", opts);
        }
    }
    Ok(())
}

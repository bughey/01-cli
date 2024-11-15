use anyhow::{Ok, Result};
use base64::{engine::GeneralPurpose, prelude::*};

use crate::{
    cli::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format},
    utils::read_input,
};

/* pub fn process_base64(subcmd: Base64SubCommand) -> Result<()> {
    match subcmd {
        Base64SubCommand::Encode(opts) => {
            let encoded = process_encode(&opts)?;
            println!("encoded: {:?}", encoded);
        }
        Base64SubCommand::Decode(opts) => {
            let decoded = process_decode(&opts)?;
            println!("decoded: {:?}", String::from_utf8(decoded)?);
        }
    }
    Ok(())
} */

pub fn process_encode(opts: &Base64EncodeOpts) -> Result<String> {
    let mut buf = Vec::new();
    read_input(&opts.input, &mut buf)?;
    // println!("\ninput: {:?}", String::from_utf8(buf.clone())?);

    let encoded = engine(&opts.format).encode(buf);
    Ok(encoded)
}

pub fn process_decode(opts: &Base64DecodeOpts) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    read_input(&opts.input, &mut buf)?;
    // println!("\ninput: {:?}", String::from_utf8(buf.clone())?);

    let decoded = engine(&opts.format).decode(buf)?;
    Ok(decoded)
}

fn engine(format: &Base64Format) -> GeneralPurpose {
    match format {
        Base64Format::Standard => BASE64_STANDARD,
        Base64Format::Urlsafe => BASE64_URL_SAFE_NO_PAD,
    }
}

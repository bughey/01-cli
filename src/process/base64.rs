use std::{fs::File, io::Read};

use anyhow::{Ok, Result};
use base64::{engine::GeneralPurpose, prelude::*};

use crate::cli::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};

pub fn process_base64(subcmd: Base64SubCommand) -> Result<()> {
    match subcmd {
        Base64SubCommand::Encode(opts) => {
            process_encode(opts)?;
        }
        Base64SubCommand::Decode(opts) => {
            process_decode(opts)?;
        }
    }
    Ok(())
}

fn process_encode(opts: Base64EncodeOpts) -> Result<()> {
    let mut buf = Vec::new();
    read_input(&opts.input, &mut buf)?;
    println!("\ninput: {:?}", String::from_utf8(buf.clone())?);

    let encoded = engine(opts.format).encode(buf);
    println!("encoded: {:?}", encoded);
    Ok(())
}

fn process_decode(opts: Base64DecodeOpts) -> Result<()> {
    let mut buf = Vec::new();
    read_input(&opts.input, &mut buf)?;
    println!("\ninput: {:?}", String::from_utf8(buf.clone())?);

    let decoded = engine(opts.format).decode(buf)?;
    println!("decoded: {:?}", String::from_utf8(decoded)?);
    Ok(())
}

fn read_input(input: &str, buf: &mut Vec<u8>) -> Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    reader.read_to_end(buf)?;
    Ok(())
}

fn engine(format: Base64Format) -> GeneralPurpose {
    match format {
        Base64Format::Standard => BASE64_STANDARD,
        Base64Format::Urlsafe => BASE64_URL_SAFE_NO_PAD,
    }
}

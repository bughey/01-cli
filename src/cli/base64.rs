use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use enum_dispatch::enum_dispatch;

use crate::process::base64::{process_decode, process_encode};

use super::{verify_file, Processor};

#[enum_dispatch(Processor)]
#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    /// 输入字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 输出格式
    #[arg(long, default_value_t = Base64Format::Standard)]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    /// 输入字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 输出格式
    #[arg(long, default_value_t = Base64Format::Standard)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::Urlsafe => write!(f, "urlsafe"),
        }
    }
}

impl Processor for Base64EncodeOpts {
    fn process(&self) -> Result<()> {
        let encoded = process_encode(self)?;
        println!("\nencoded: {}", encoded);
        Ok(())
    }
}

impl Processor for Base64DecodeOpts {
    fn process(&self) -> Result<()> {
        let decoded = process_decode(self)?;
        println!("\ndecoded: {}", String::from_utf8(decoded)?);
        Ok(())
    }
}

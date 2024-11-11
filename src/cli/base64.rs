use std::fmt::Display;

use clap::{Parser, ValueEnum};

use super::verify_file;

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

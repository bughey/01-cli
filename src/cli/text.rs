use std::{fmt::Display, path::PathBuf};

use clap::{Parser, ValueEnum};

use super::{verify_file, verify_path};

#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a secret/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key", alias = "g")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Parser, Debug)]
pub struct TextKeyGenerateOpts {
    /// 签名算法
    #[arg(short, long, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
    /// 输出文件
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    /// 输入字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 密钥
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// 签名算法
    #[arg(long, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    /// 输入字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 密钥
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// 签名算法
    #[arg(long, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
    /// 签名
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

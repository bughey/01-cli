use std::{fmt::Display, fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use enum_dispatch::enum_dispatch;

use crate::{
    process::text::{process_generate, process_sign, process_verify},
    Processor,
};

use super::{verify_file, verify_path};

#[enum_dispatch(Processor)]
#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a secret/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text")]
    Verify(TextVerifyOpts),
    #[command(about = "[g] Generate a new key", alias = "g")]
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
    #[arg(short, long, default_value_t = TextSignFormat::Blake3)]
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
    #[arg(short, long, default_value_t = TextSignFormat::Blake3)]
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

impl Processor for TextSignOpts {
    async fn process(self) -> Result<()> {
        let sig = process_sign(self)?;
        println!("\nsigned: {}", sig);
        Ok(())
    }
}

impl Processor for TextVerifyOpts {
    async fn process(self) -> Result<()> {
        let verified = process_verify(self)?;
        println!("\nverified: {}", verified);
        Ok(())
    }
}

impl Processor for TextKeyGenerateOpts {
    async fn process(self) -> Result<()> {
        let keys = process_generate(&self)?;

        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &keys[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output.join("ed25519");
                if !name.exists() {
                    fs::create_dir_all(&name)?;
                }
                fs::write(name.join("sk"), &keys[0])?;
                fs::write(name.join("pk"), &keys[1])?;
            }
        }

        Ok(())
    }
}

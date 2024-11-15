use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::process::csv_convert::process_csv;

use super::{verify_file, Processor};

#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// 输入文件
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    /// 输出文件
    #[arg(short, long)]
    pub output: Option<String>,
    /// 输出文件格式
    #[arg(short, long, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    /// 输入文件是否包含csv头部，默认为true
    #[arg(long, default_value_t = true)]
    pub header: bool,
    /// 输入文件列分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

impl Processor for CsvOpts {
    async fn process(self) -> Result<()> {
        process_csv(self)
    }
}

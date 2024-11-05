use std::{fmt::Display, path::Path};

use clap::{Parser, Subcommand, ValueEnum};

// rcli csv -i input.csv -o output.json --header -d ','

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// 输入文件
    #[arg(short, long, value_parser = verify_input_file)]
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

/* impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::Json => "json".to_string(),
            OutputFormat::Yaml => "yaml".to_string(),
        }
    }
} */

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found".to_string())
    }
}

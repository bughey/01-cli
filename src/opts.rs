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
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
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

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    /// 密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    /// 是否包含小写字母
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    /// 是否包含大写字母
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    /// 是否包含数字
    #[arg(long, default_value_t = true)]
    pub number: bool,
    /// 是否包含特殊字符
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
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

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found".to_string())
    }
}

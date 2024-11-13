use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Start a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    /// 目录
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    /// 监听地址
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

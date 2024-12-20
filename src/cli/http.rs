use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process::http_serve::process_http_serve, Processor};

use super::verify_path;

#[enum_dispatch(Processor)]
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

impl Processor for HttpServeOpts {
    async fn process(self) -> Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}

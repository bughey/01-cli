use anyhow::Result;
use clap::Parser;
use zxcvbn::zxcvbn;

use crate::process::gen_pass::process_genpass;

use super::Processor;

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

impl Processor for GenPassOpts {
    fn process(self) -> Result<()> {
        let pwd = process_genpass(self)?;
        println!("{}", pwd);

        let estimate = zxcvbn(&pwd, &[]);
        eprintln!("Password strength {}", estimate.score());
        Ok(())
    }
}

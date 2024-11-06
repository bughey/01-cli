use clap::Parser;

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
    #[arg(short, long)]
    pub input: String,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    /// 输入字符串
    #[arg(short, long)]
    pub input: String,
}

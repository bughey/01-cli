// rcli csv -i input.csv -o output.json --header -d ','

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "RCli")]
#[command(version = "1.0")]
#[command(about = "A simple to use, efficient, and full-featured command line tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Csv {
        /// 输入文件
        #[arg(short, long)]
        input: String,
        /// 输入文件
        #[arg(short, long)]
        output: Option<String>,
        /// 输入文件是否包含csv头部，默认为false
        #[arg(long, default_value_t = false)]
        header: bool,
        /// 输入文件列分隔符，默认为","
        #[arg(short, long, default_value_t = String::from(","))]
        delimiter: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Csv {
            input,
            output,
            header,
            delimiter,
        } => {
            println!(
                "input: {}, output: {:?}, header: {}, delimiter: {}",
                input, output, header, delimiter
            );
        }
    }
}

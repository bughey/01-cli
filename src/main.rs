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
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long, default_value_t = false)]
        header: bool,
        #[arg(short, long, default_value_t = String::from(","))]
        delimiter: String,
    },
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
}

// rcli csv -i input.csv -o output.json --header -d ','

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    path::Path,
};

use clap::{Parser, Subcommand};
use csv::ReaderBuilder;

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
        /// 输入文件是否包含csv头部，默认为true
        #[arg(long, default_value_t = true)]
        header: bool,
        /// 输入文件列分隔符，默认为","
        #[arg(short, long, default_value_t = String::from(","))]
        delimiter: String,
    },
}

fn main() -> std::io::Result<()> {
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
            let input_path = Path::new(input.as_str());
            let f = File::open(input_path)?;

            let mut reader = ReaderBuilder::new()
                .has_headers(header)
                .delimiter(delimiter.as_bytes()[0])
                .from_reader(f);

            // read headers
            let headers = reader.headers()?.clone();

            // read records
            let mut records: Vec<HashMap<String, String>> = Vec::new();
            for result in reader.records() {
                let record = result?;
                let mut row = HashMap::new();
                for (i, header) in headers.iter().enumerate() {
                    row.insert(header.to_string(), record.get(i).unwrap().to_string());
                }
                records.push(row);
            }

            // write records to file
            let output = output.unwrap_or_else(|| input.clone() + ".json");
            let output_path = Path::new(output.as_str());
            let output_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_path)?;
            serde_json::to_writer_pretty(output_file, &records)?;

            println!("{input} -> {output}, Done.");

            Ok(())
        }
    }
}

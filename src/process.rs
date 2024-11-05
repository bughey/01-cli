use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    path::Path,
};

use anyhow::Result;
use csv::ReaderBuilder;

use crate::opts::CsvOpts;

pub fn process_csv(opts: CsvOpts) -> Result<()> {
    let CsvOpts {
        input,
        output,
        header,
        delimiter,
    } = opts;
    println!(
        "input: {}, output: {:?}, header: {}, delimiter: {}",
        input, output, header, delimiter
    );
    let input_path = Path::new(input.as_str());
    let f = File::open(input_path)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(header)
        .delimiter(delimiter as u8)
        .from_reader(f);

    // read headers
    let headers = reader.headers()?.clone();

    // read records
    let mut records: Vec<BTreeMap<String, String>> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut row = BTreeMap::new();
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
    serde_json::to_writer_pretty(&output_file, &records)?;

    println!("{input} -> {output}, Done.");

    Ok(())
}

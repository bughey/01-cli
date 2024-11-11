use std::{fs::File, io::Read};

use anyhow::Result;

pub fn read_input(input: &str, buf: &mut Vec<u8>) -> Result<()> {
    let mut reader = get_reader(input)?;

    reader.read_to_end(buf)?;

    Ok(())
}

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    Ok(if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    })
}

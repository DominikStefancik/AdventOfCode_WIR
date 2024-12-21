use crate::error::AdventOfCodeError;
use crate::filesystem_processor::check_sum;
use crate::parser::parse_input;
use std::fs;

mod error;
mod filesystem_processor;
mod parser;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let file_blocks = parse_input(input.as_str());
    let check_sum = check_sum(&file_blocks)?;

    println!("Filesystem checksum: {check_sum}");

    Ok(())
}

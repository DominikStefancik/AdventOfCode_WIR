use std::fs;
use crate::antidote_counter::calculate_antidotes_count;
use crate::error::AdventOfCodeError;
use crate::parser::parse_input;

mod error;
mod parser;
mod antidote_counter;
mod antenna;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let map = parse_input(input.as_str());
    let antidotes_count = calculate_antidotes_count(&map);

    println!("Number of unique locations within with an antinode: {antidotes_count}");

    Ok(())
}

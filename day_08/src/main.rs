use crate::antidote_counter::{calculate_antidotes_at_any_grid_count, calculate_antidotes_count};
use crate::error::AdventOfCodeError;
use crate::parser::parse_input;
use std::fs;

mod antenna;
mod antidote_counter;
mod error;
mod parser;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let map = parse_input(input.as_str());
    let map_dimensions = (map.len(), map[0].len());
    let antennas_map = antidote_counter::create_antennas_map(&map);
    let antidotes_count = calculate_antidotes_count(&antennas_map, map_dimensions);
    let antidotes_at_any_grid_count =
        calculate_antidotes_at_any_grid_count(&antennas_map, map_dimensions);

    println!("Number of unique locations with an antinode: {antidotes_count}");
    println!(
        "Number of unique locations with an antinode at any grid: {antidotes_at_any_grid_count}"
    );

    Ok(())
}

use crate::error::AdventOfCodeError;
use crate::map_traversal::number_of_moves_until_exit;
use crate::parse::parse_input;
use std::fs;

mod direction;
mod error;
mod map_traversal;
mod parse;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let map = parse_input(input.as_str());
    let moves_count = number_of_moves_until_exit(map);

    println!("Number of moves until guard gets out: {moves_count}");
    Ok(())
}

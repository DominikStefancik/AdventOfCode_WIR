use crate::error::AdventOfCodeError;

pub fn parse_input(text: &str) -> Vec<Vec<char>> {
    let map = text.lines().map(|line| line.chars().collect()).collect();

    map
}

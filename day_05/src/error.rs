use std::io;
use std::num;

#[derive(Debug)]
pub enum AdventOfCodeError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for AdventOfCodeError {
    fn from(err: io::Error) -> Self {
        AdventOfCodeError::IoError(err)
    }
}

impl From<num::ParseIntError> for AdventOfCodeError {
    fn from(err: num::ParseIntError) -> Self {
        AdventOfCodeError::ParseError(err)
    }
}

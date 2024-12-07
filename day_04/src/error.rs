use std::io;

#[derive(Debug)]
pub enum AdventOfCodeError {
    IoError(io::Error),
    RegexError(regex::Error)
}

impl From<io::Error> for AdventOfCodeError {
    fn from(err: io::Error) -> Self {
        AdventOfCodeError::IoError(err)
    }
}

impl From<regex::Error> for AdventOfCodeError {
    fn from(err: regex::Error) -> Self {
        AdventOfCodeError::RegexError(err)
    }
}
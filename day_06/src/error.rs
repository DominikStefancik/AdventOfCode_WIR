use std::io;

#[derive(Debug)]
pub enum AdventOfCodeError {
    IoError(io::Error),
}

impl From<io::Error> for AdventOfCodeError {
    fn from(err: io::Error) -> Self {
        AdventOfCodeError::IoError(err)
    }
}

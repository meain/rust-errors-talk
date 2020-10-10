use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum CustomErrors {
    NoFile,
    NonRustFile,
}

impl Error for CustomErrors {}
impl fmt::Display for CustomErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

fn main() {}

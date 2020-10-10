use std::error::Error;
use std::fmt;
use std::fs::File;

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

fn do_thing() -> Result<(), Box<dyn Error>> {
    let file = File::open("foo.txt")?;
    Ok(())
}

fn main() {
    do_thing();
}

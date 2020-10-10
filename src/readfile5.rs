use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file_contents(name: &str) -> Result<String, Error> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()>{
    let file_contents = read_file_contents("foo.txt")?;
    println!("{}", file_contents);
    Ok(())
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("foo.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    // ... beam file contents to space
}

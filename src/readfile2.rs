use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("secret-to-understanding-borrow-checker.txt");
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents);
        }
        Err(_) => {
	    println!("Hey, I don't think that file exists.");
        }
    };
}

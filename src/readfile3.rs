use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file_contents(name: &str) -> Result<String, Error> {
    let file = File::open(name);
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            let erro = f.read_to_string(&mut contents);
	    return match erro {
		Ok(_) => Ok(contents),
		Err(e) => Err(e)
	    }
        }
        Err(err) => Err(err),
    }
}

fn main() {
    let file_contents = read_file_contents("foo.txt");
    match file_contents {
        Ok(fc) => {
            println!("{}", fc);
        }
        Err(_) => {
            println!("Hey, I don't think that file exists.");
        }
    };
}

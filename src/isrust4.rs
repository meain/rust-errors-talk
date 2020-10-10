use std::io::{Error, ErrorKind};

fn is_rustfile(name: &str) -> Result<String, Error> {
    if name.ends_with(".rs") {
        Ok(name.to_string())
    } else {
        Err(Error::new(ErrorKind::Other, "Where is my rust file?"))
    }
}

fn main() {
    println!("main.rs: {:?}", is_rustfile("main.rs"));
    println!("main.go: {:?}", is_rustfile("main.go"));
}

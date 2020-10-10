fn is_rustfile(name: &str) -> Option<String> {
    if name.ends_with(".rs") {
        Some(name.to_string())
    } else {
        None
    }
}

fn main() {
    println!("main.rs: {:?}", is_rustfile("main.rs"));
    println!("main.go: {:?}", is_rustfile("main.go"));
}

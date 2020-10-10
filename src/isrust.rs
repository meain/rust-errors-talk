#[derive(Debug)]
enum FileKind {
    Rust(String),
    NotRust(String),
}

fn is_rustfile(name: &str) -> FileKind {
    if name.ends_with(".rs") {
        FileKind::Rust(name.to_string())
    } else {
        FileKind::NotRust(name.to_string())
    }
}

fn main() {
    println!("main.rs: {:?}", is_rustfile("main.rs"));
    println!("main.go: {:?}", is_rustfile("main.go"));
}

fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let greeting = greet("meain".to_string());
    println!("{}", greeting);
}

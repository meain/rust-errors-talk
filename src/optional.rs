fn optional_greet(name: String) -> Option<String> {
    if name == "Ravi" {
        Some(format!("Hello, {}!", name))
    } else {
        None
    }
}

fn main() {
    let greeting = optional_greet("meain".to_string());
    match greeting {
        Some(g) => println!("{}", g),
        None => println!("Why are you here?"),
    };
}

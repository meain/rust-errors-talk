fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = std::env::args().nth(1);
    println!("{:?}", name);
    let greeting = greet(name);
    println!("{}", greeting);
}

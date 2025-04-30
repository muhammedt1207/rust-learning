 fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn say_message() {
    let message = greet("Alice");
    println!("{}", message);
    
}

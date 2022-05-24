use std::io::stdin;

fn your_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
        name
            .trim()
            .to_lowercase()
}


fn main() {
    println!("Type your name, please");
    let username = your_name();
    println!("Hello, {}", username);
}

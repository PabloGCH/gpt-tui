fn main() {
    let value = std::env::args().nth(1).unwrap();
    println!("Hello, world!");
    println!("Value: {}", value);
}

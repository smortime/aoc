use std::fs;

fn main() {
    let binding = fs::read_to_string("input/input.txt").unwrap();
    let content = binding.trim();
    println!("Hello, world!");
}

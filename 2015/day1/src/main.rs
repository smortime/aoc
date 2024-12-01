use std::fs::{self};

fn main() {
    let binding = fs::read_to_string("./input/input.txt").unwrap();
    let input = binding.trim();
    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }
        if count < 0 {
            println!("Position: {i + 1}");
            return;
        }
    }
    println!("Floor {}", count);
}

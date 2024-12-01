use std::fs;

fn main() {
    let binding = fs::read_to_string("input/input.txt").unwrap();
    let content = binding.trim();
    let mut total = 0;
    for line in content.lines() {
        let mut parts: Vec<i32> = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        parts.sort();
        total += 2 * parts[0] + 2 * parts[1] + parts.iter().fold(1, |acc, x| acc * x);
    }
    println!("Total: {total}");
}

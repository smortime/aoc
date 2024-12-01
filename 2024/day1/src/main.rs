use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let mut v1 = HashMap::<i32, i32>::new();
    let mut count = HashMap::<i32, i32>::new();
    let mut v2 = Vec::<i32>::new();
    for i in input.lines() {
        let parts: Vec<&str> = i.split("   ").collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        v1.insert(x, 0);
        count.insert(x, count.get(&x).unwrap_or(&0) + 1);
        v2.push(y);
    }

    for v in v2 {
        if v1.contains_key(&v) {
            v1.insert(v, v1.get(&v).unwrap_or(&0) + 1);
        }
    }

    let mut diff = 0;
    for (k, v) in v1 {
        diff += k * v * count.get(&k).unwrap_or(&1);
    }

    println!("Diff is: {diff}");
}

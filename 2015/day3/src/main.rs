use std::{collections::HashSet, fs};

fn main() {
    let mut set = HashSet::<(i32, i32)>::new();
    set.insert((0, 0));

    let mut x = 0;
    let mut y = 0;
    let mut x_1 = 0;
    let mut y_1 = 0;

    let bind = fs::read_to_string("input/input.txt").unwrap();
    let input = bind.trim();
    let mut santa = true;
    for c in input.chars() {
        let p = if santa {
            (x, y) = location(c, x, y);
            (x, y)
        } else {
            (x_1, y_1) = location(c, x_1, y_1);
            (x_1, y_1)
        };
        set.insert(p);
        santa = !santa;
    }

    println!("Houses: {}", set.len());
}

fn location(c: char, x: i32, y: i32) -> (i32, i32) {
    match c {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => (0, 0),
    }
}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_file() -> Vec<String> {
    let f = File::open("input.txt").unwrap();
    BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect()
}

fn should_destroy(x: char, y: char ) -> bool {
    let has_uppercase = x.is_uppercase() ^ y.is_uppercase();
    has_uppercase && x.to_lowercase().to_string() == y.to_lowercase().to_string()
}

fn main() {
    let lines = read_file();
    let mut input = lines[0].clone();

    let mut stack: Vec<char> = vec![];
    for ch in input.chars() {
        if let Some(ch2) = stack.pop() {
            if should_destroy(ch, ch2) {
                continue;
            } else {
                stack.push(ch2);
            }
        }

        stack.push(ch);
    }

    println!("{}", stack.len());
}

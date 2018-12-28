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
    let has_uppercase = x.is_uppercase() || y.is_uppercase();
    has_uppercase && x.to_lowercase().to_string() == y.to_lowercase().to_string()
}

fn get_indices(input: &String) -> Vec<usize> {
    let last_index = input.len();
    let mut destroy_indices: Vec<usize> = vec![];

    for (i, current) in input.chars().enumerate() {
        let next_index = i + 1;
        if next_index < last_index {
            let next = input.chars().nth(next_index).unwrap();
            if should_destroy(current, next) {
                destroy_indices.push(i);
            }
        }
    }

    destroy_indices
}

fn remove_indices(input: &String, indices: Vec<usize>) -> String {
    let mut new_string = String::new();
    let mut i = 0;
    for idx in indices {
        new_string.push_str(&input[i..idx]);
        i = i + 2;
    }

    new_string
}

fn char_at(input: &String, idx: usize) -> Option<char> {
    input.chars().nth(idx)
}
                                       

fn main() {
    let lines = read_file();
    let mut input = lines[0].clone();
    let mut input = "dabAcCaCBAcCcaDA".to_string();
    let mut output = String::new();
    let mut max_index = input.len();
    let mut cur_idx = 0;
    let mut next_idx = cur_idx + 1;
    let mut bad_indices = HashSet::new();
    while cur_idx < max_index && next_idx < max_index {
        let cur = match char_at(&input, cur_idx) {
            None => panic!("cur_idx out of bounds {}", cur_idx),
            Some(cur) => cur,
        };
        if let Some(next) = char_at(&input, next_idx) {
            if should_destroy(cur, next) {
                bad_indices.insert(cur_idx);
                bad_indices.insert(next_idx);
                if cur_idx > 0 {
                    cur_idx -= 1;
                }
                next_idx += 1;
                continue;
                
            } else {
                cur_idx = next_idx;
                next_idx = cur_idx + 1;
            }
        }
    }

    for (i, ch) in input.chars().enumerate() {
        if !bad_indices.contains(&i) {
            output.push(ch);
        }
    }
    
    println!("{}", output.len());
}

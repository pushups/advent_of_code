use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn count_chars(s: &String) -> HashMap<char, u8> {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

fn compare_strings(x: &String, y: &String) -> Option<String> {
    let mut diffs = 0;
    let mut solution = String::from("");

    if x == y {
        return None;
    }

    for (ch, ch2) in x.chars().zip(y.chars()) {
        if ch == ch2 {
            solution.push(ch);
        } else {
            diffs += 1;

            if diffs > 1 {
                return None;
            }
        }
    }

    return Some(solution);
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect();

    let mut answer: Option<String> = None;
    for x in lines.iter() {
        for y in lines.iter() {
            answer = compare_strings(&x, &y);

            if answer != None {
                break;
            }
        }

        if answer != None {
            break;
        }
    }
    
    println!("{:?}", answer);
    Ok(())
}

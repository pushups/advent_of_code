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

fn has_count(x: &HashMap<char, u8>, count: u8) -> bool {
    for val in x.values() {
        if *val == count {
            return true;
        }
    }

    return false;
}
                                              


fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect();

    let counts: Vec<HashMap<char, u8>> = lines.iter().map(|s| count_chars(&s)).collect();
    let twos = counts.iter().filter(|x| has_count(&x, 2)).count();
    let threes = counts.iter().filter(|x| has_count(&x, 3)).count();
    
    println!("{:?}", twos * threes);
    Ok(())
}

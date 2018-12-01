use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input_file = "input.txt";
    let path = Path::new(input_file);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut input = String::new();

    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        _ => (),
    };

    let nums: Vec<i32> = input.split("\n").collect::<Vec<_>>()
        .iter()
        .filter(|&x| x != &"")
        .map(|&x| {
            match x.parse::<i32>() {
                Err(why) => panic!("{}", why.description()),
                Ok(val) => return val,
            }
        }).collect();

    let mut freqs = HashMap::new();
    let mut freq = 0;
    let mut seen_twice: Option<i32> = None;
    let mut it = nums.iter().cycle();
    while seen_twice == None {
        match it.next() {
            Some(x) => freq += x,
            _ => panic!("infinite cycle somehow ran out?")
        }
        if freqs.contains_key(&freq) {
            seen_twice = Some(freq);
        } else {
            freqs.insert(freq, true);
        }
    }

    println!("{:?}", seen_twice);
}

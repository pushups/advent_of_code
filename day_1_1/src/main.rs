use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


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

    let solution: i32 = input.split("\n").collect::<Vec<_>>()
        .iter()
        .filter(|&x| x != &"")
        .map(|&x| {
            match x.parse::<i32>() {
                Err(why) => panic!("{}", why.description()),
                Ok(val) => return val,
            }
        })
        .sum();
              
    println!("{:?}", solution);
}

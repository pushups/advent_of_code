extern crate regex;
extern crate chrono;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Eq;
use chrono::DateTime;

struct LogEntry {
    
}

fn read_file() -> Vec<String> {
    let f = File::open("input.txt").unwrap();
    BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect()
}

fn parse_line(line: &String) -> DateTime {
    let date = DateTime::parse_from_str(, "[%Y-%m-%d %H:%M]");
}

fn main() {
    let lines = read_file();
    println!("{}", lines[0]);
}

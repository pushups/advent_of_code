extern crate regex;
extern crate chrono;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Eq;
use chrono::{NaiveDateTime, Timelike};
use std::time::Duration;
use std::clone::Clone;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Event {
    BeginsShift,
    FallsAsleep,
    WakesUp
}

#[derive(Debug)]
#[derive(Clone)]
struct LogEntry {
    id: Option<String>,
    timestamp: NaiveDateTime,
    event: Event,
    minute: u32,
}

#[derive(Debug)]
struct Summary {
    total_time_asleep: i64,
    minutes_asleep: HashMap<u32, u32>,
}

fn read_file() -> Vec<String> {
    let f = File::open("input.txt").unwrap();
    BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect()
}

fn parse_event(x: &str) -> Event {
    if x == "begins shift" {
        return Event::BeginsShift;
    }
    if x == "falls asleep" {
        return Event::FallsAsleep;
    }

    if x == "wakes up" {
        return Event::WakesUp;
    }

    panic!("Unable to parse event {}", x);
}

fn parse_line(line: &String) -> LogEntry {
    let mut re = Regex::new(r"^\[(.+?)\] Guard #(\d+) (.+?)$").unwrap();
    match re.captures(line) {
        Some(caps) => {
            let timestamp = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M").unwrap();
            LogEntry {
                id: Some(caps[2].to_string()),
                timestamp: timestamp,
                event: parse_event(&caps[3]),
                minute: timestamp.minute(),
            }
        },
        None => {
            re = Regex::new(r"^\[(.+?)\] (.+?)$").unwrap();
            let caps = re.captures(line).unwrap();
            let timestamp = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M").unwrap();
            LogEntry {
                id: None,
                timestamp: timestamp,
                event: parse_event(&caps[2]),
                minute: timestamp.minute(),
            }
        },
    }
}

fn fix_ids(entries: &mut Vec<LogEntry>) {
    for i in 0..(entries.len() - 1) {
        if entries[i].id != None && entries[i+1].id == None {
            let id = entries[i].id.clone();
            entries[i+1].id = id;
        }
    }
}

fn debug(entries: &Vec<LogEntry>) {
    for i in 0..10 {
        println!("{:?}", entries[i]);
    }
}

fn main() {
    let lines = read_file();
    let mut parsed_lines: Vec<LogEntry> = lines.iter().map(|x| parse_line(x)).collect();
    parsed_lines.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    fix_ids(&mut parsed_lines);
    //debug(&parsed_lines);
    let mut summaries: HashMap<String, Summary> = HashMap::new();
    for i in 0..(parsed_lines.len() - 1) {
        let a = parsed_lines[i].clone();
        let b = parsed_lines[i+1].clone();
        if a.event == Event::FallsAsleep && b.event == Event::WakesUp {
            let mut summary = summaries.entry(a.id.unwrap()).or_insert(Summary{total_time_asleep: 0, minutes_asleep: HashMap::new()});
            let time_asleep = b.timestamp.signed_duration_since(a.timestamp).num_minutes();
            summary.total_time_asleep += time_asleep;
            for min in a.minute..b.minute {
                *summary.minutes_asleep.entry(min).or_insert(0) += 1;
            }
        }
    }

    println!("{:?}", summaries);
}

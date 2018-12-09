extern crate regex;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Eq;

#[derive(Debug)]
struct Claim {
    id: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Claim {
    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        for x in self.x .. (self.x + self.width) {
            for y in self.y .. (self.y + self.height) {
                points.push(Point{x: x, y: y});
            }
        }
        points
    }
}



fn parse_claim(x: &String) -> Claim {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let caps = re.captures(x).unwrap();
    Claim {
        id: caps[1].to_string(),
        x: caps[2].parse::<u32>().unwrap(),
        y: caps[3].parse::<u32>().unwrap(),
        width: caps[4].parse::<u32>().unwrap(),
        height: caps[5].parse::<u32>().unwrap(),
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect();

    let claims: Vec<Claim> = lines.iter().map(|x| parse_claim(&x)).collect();

    let mut claimed = HashMap::new();
    for claim in claims.iter() {
        for point in claim.points() {
            let key = format!("{}x{}", point.x, point.y);
            *claimed.entry(key).or_insert(0) += 1;
        }
    }

    for claim in claims.iter() {
        let points = claim.points();
        let mut num_points = points.len();
        for point in points {
            let key = format!("{}x{}", point.x, point.y);
            match claimed.get(&key) {
                None => panic!("WTF?"),
                Some(val) => {
                    if *val >= 2 {
                        num_points -= 1;
                    }
                }
            }
        }

        if num_points == claim.points().len() {
            println!("{}", claim.id);
        }
    }
    
    Ok(())
}

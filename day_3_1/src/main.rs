extern crate regex;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: String,
    x: u32,
    y: u32,
    width: u32,
    length: u32,
    
}

impl Claim {
    fn contains(&self, x: u32, y: u32) -> bool {
        let x_bound = self.x + self.width;
        let y_bound = self.y + self.length;
        x >= self.x && y >= self.y && x <= x_bound && y <= y_bound
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
        length: caps[5].parse::<u32>().unwrap(),
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(f).lines()
        .map(|l| l.expect("Unable to parse"))
        .collect();

    let claims: Vec<Claim> = lines.iter().map(|x| parse_claim(&x)).collect();
    let mut claim_count = 0;
    let mut inch_count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            for claim in claims.iter() {
                if claim.contains(x, y) {
                    claim_count += 1;
                }
            }

            if claim_count >= 2 {
                inch_count += 1;
                claim_count = 0;
            }
        }
    }

    println!("{}", inch_count);
    
    Ok(())
}

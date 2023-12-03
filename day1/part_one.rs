#!/usr/bin/env run-cargo-script

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        if let (Some(&first_digit), Some(&last_digit)) = (digits.first(), digits.last()) {
            let number = first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
            sum += number as i32;
        }
    }

    println!("Total sum: {}", sum);
    Ok(())
}


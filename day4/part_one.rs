#!/usr/bin/env run-cargo-script
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILE_PATH: &str = "input.txt";

fn main() -> io::Result<()> {
    let path = Path::new(FILE_PATH);
    let file = File::open(&path)?;

    let mut total_points = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let (card, winning_nums, card_nums) = parse_line(&line)?;

        let points = calculate_points(&winning_nums, &card_nums);
        total_points += points;
    }

    println!("Total points: {}", total_points);
    Ok(())
}

fn parse_line(line: &str) -> io::Result<(String, String, String)> {
    let mut splits = line.splitn(2, ':');
    let card = splits.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "incorrect input"))?.to_string();
    let all_numbers = splits.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "incorrect input"))?;

    let mut splits = all_numbers.splitn(2, '|');
    let winning_nums = splits.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "incorrect input"))?.to_string();
    let card_nums = splits.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "incorrect input"))?.to_string();

    Ok((card, winning_nums, card_nums))
}

fn calculate_points(winning_nums: &str, card_nums: &str) -> i32 {
    let winning_set: HashSet<i32> = winning_nums
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let card_set: HashSet<i32> = card_nums
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let intersection_count = winning_set.intersection(&card_set).count();
    if intersection_count > 0 {
        1 << (intersection_count - 1)
    } else {
        0
    }
}

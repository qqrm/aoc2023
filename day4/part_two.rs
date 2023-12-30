#!/usr/bin/env run-cargo-script

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

const FILE_PATH: &str = "input.txt";

fn main() -> io::Result<()> {
    let path = Path::new(FILE_PATH);
    let file = File::open(&path)?;
    let mut cards_num = vec![];
    let mut dups = vec![];

    let mut total_dup_cards = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let (card, intersection_count) = parse_line(&line)?;

        cards_num.push((card as usize, intersection_count));
    }

    for (card, dups_count) in &cards_num {
        dups.extend(&cards_num[*card..(card + dups_count)]);
        total_dup_cards += dups_count;
    }

    while !dups.is_empty() {
        let mut temp = vec![];

        for (card, dups_count) in &dups {
            temp.extend(&cards_num[*card..(card + dups_count)]);
            total_dup_cards += dups_count;
        }

        dups = temp;
    }

    println!("Total points: {}", total_dup_cards + cards_num.len());
    Ok(())
}

fn parse_line(line: &str) -> io::Result<(i64, usize)> {
    let create_io_error =
        |message: &str| -> io::Error { Error::new(ErrorKind::InvalidData, message) };

    let mut splits = line.splitn(2, ':');
    let card = splits
        .next()
        .ok_or_else(|| create_io_error("incorrect input"))?;

    let card_num = card
        .split_whitespace()
        .last()
        .ok_or_else(|| create_io_error("incorrect input"))?
        .parse::<i64>()
        .map_err(|_| create_io_error("failed to parse card number"))?;

    let all_numbers = splits
        .next()
        .ok_or_else(|| create_io_error("incorrect input"))?;

    let mut numbers_splits = all_numbers.splitn(2, '|');
    let winning_nums = numbers_splits
        .next()
        .ok_or_else(|| create_io_error("incorrect input"))?
        .to_string();
    let card_nums = numbers_splits
        .next()
        .ok_or_else(|| create_io_error("incorrect input"))?
        .to_string();

    let winning_set: HashSet<i64> = winning_nums
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let card_set: HashSet<i64> = card_nums
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let intersection_count = winning_set.intersection(&card_set).count();

    Ok((card_num, intersection_count))
}

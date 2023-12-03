#!/usr/bin/env run-cargo-script

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let (num_map, rev_num_map) = build_number_maps();

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let (start_num, end_num) = parse_line(&line, &num_map, &rev_num_map);
        total_sum += start_num * 10 + end_num;
    }

    println!("Total sum: {}", total_sum);
    Ok(())
}

fn build_number_maps() -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut num_map = HashMap::new();
    let mut rev_num_map = HashMap::new();

    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for &(word, value) in &numbers {
        num_map.insert(word.to_string(), value);
        rev_num_map.insert(word.chars().rev().collect(), value);
    }

    (num_map, rev_num_map)
}

fn build_reversed_number_map(num_map: &HashMap<String, i32>) -> HashMap<String, i32> {
    num_map
        .iter()
        .map(|(key, &value)| (key.chars().rev().collect::<String>(), value))
        .collect()
}

fn parse_line(
    line: &str,
    num_map: &HashMap<String, i32>,
    rev_num_map: &HashMap<String, i32>,
) -> (i32, i32) {
    let start_num = extract_number(&line, num_map);
    let reversed_line = line.chars().rev().collect::<String>();
    let end_num = extract_number(&reversed_line, rev_num_map);
    (start_num, end_num)
}

fn extract_number(s: &str, num_map: &HashMap<String, i32>) -> i32 {
    let mut current_word = String::new();

    for ch in s.chars() {
        if ch.is_digit(10) {
            return ch.to_digit(10).unwrap() as i32;
        } else if ch.is_alphabetic() {
            current_word.push(ch);
            for (num_word, &num_value) in num_map {
                if current_word.contains(num_word) {
                    return num_value;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two1nine() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(parse_line("two1nine", &num_map, &rev_num_map), (2, 9));
    }

    #[test]
    fn test_eightwothree() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(parse_line("eightwothree", &num_map, &rev_num_map), (8, 3));
    }

    #[test]
    fn test_abcone2threexyz() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(
            parse_line("abcone2threexyz", &num_map, &rev_num_map),
            (1, 3)
        );
    }

    #[test]
    fn test_xtwone3four() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(parse_line("xtwone3four", &num_map, &rev_num_map), (2, 4));
    }

    #[test]
    fn test_4nineeightseven2() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(
            parse_line("4nineeightseven2", &num_map, &rev_num_map),
            (4, 2)
        );
    }

    #[test]
    fn test_zoneight234() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(parse_line("zoneight234", &num_map, &rev_num_map), (1, 4));
    }

    #[test]
    fn test_7pqrstsixteen() {
        let (num_map, rev_num_map) = build_number_maps();
        assert_eq!(parse_line("7pqrstsixteen", &num_map, &rev_num_map), (7, 6));
    }
}

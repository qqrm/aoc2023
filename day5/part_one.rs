#!/usr/bin/env run-cargo-script
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILE_PATH: &str = "input.txt";

/// Parses the first line of the provided lines to extract seed numbers.
/// Returns a Vec of tuples containing the seed number and a boolean flag.
fn parse_seeds(
    lines: &mut std::io::Lines<std::io::BufReader<File>>,
) -> io::Result<Vec<(i64, bool)>> {
    let seed_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No seed line"))??;
    let seeds = seed_line
        .split(':')
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No seeds part"))?;

    seeds
        .split_ascii_whitespace()
        .map(|s| {
            s.parse::<i64>().map(|num| (num, false)).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Error parsing seed numbers")
            })
        })
        .collect()
}

/// Resets the modification flags for all seeds.
fn reset_seed_flags(seed_nums: &mut Vec<(i64, bool)>) {
    for seed in seed_nums.iter_mut() {
        seed.1 = false;
    }
}

fn main() -> io::Result<()> {
    let path = Path::new(FILE_PATH);
    let file = File::open(&path)?;

    let mut lines = io::BufReader::new(file).lines();
    let mut seed_nums = parse_seeds(&mut lines)?;

    for line_result in lines {
        let line = line_result?;

        if line.ends_with(':') {
            reset_seed_flags(&mut seed_nums);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let map_nums: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Error parsing map numbers"))?;

        let src_start_range = map_nums[1];
        let src_end_range = src_start_range + map_nums[2];
        let dst_start_range = map_nums[0];

        for seed in seed_nums.iter_mut() {
            if src_start_range <= seed.0 && seed.0 < src_end_range && !seed.1 {
                let offset = seed.0 - src_start_range;
                *seed = (dst_start_range + offset, true);
            }
        }
    }

    let binding = seed_nums.iter().map(|n| n.0).collect::<Vec<_>>();
    match binding.iter().min() {
        Some(min) => println!("Min: {}", min),
        None => println!("No minimum value found"),
    }

    Ok(())
}

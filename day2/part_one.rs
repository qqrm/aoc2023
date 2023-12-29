#!/usr/bin/env run-cargo-script

// Import necessary modules
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// Define a constant array for the maximum allowed counts of each color
// This array will be used for validating color counts in the game data
static MAX_ARRAY: [(&'static str, u64); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() -> io::Result<()> {
    // Open the input file 'input.txt'
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;

    // Process each line in the file
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        process_line(&line, &mut sum)?;
    }

    // Print the total sum after processing all lines
    println!("Total sum: {}", sum);
    Ok(())
}

// Function to parse the game ID from a line
fn parse_game_id(line: &str) -> (u64, &str) {
    let game_id = line.split(':').next().unwrap();
    let game_id_num: u64 = game_id
        .trim()
        .parse()
        .expect("Failed to parse string to u64");
    let remaining_line = &line[game_id.len() + 2..]; // Skip game_id and ": "
    (game_id_num, remaining_line)
}

fn process_line(line: &str, sum: &mut u64) -> io::Result<()> {
    let mut is_game_valid = true;

    // skip "game " prefix in the line
    let line = &line[5..];

    let (game_id_num, mut line) = parse_game_id(line);

    // Process each game in the line
    while let Some(game) = line.split(';').next() {
        let mut set = game;

        // Process each color data in the game
        while let Some(color_data) = set.split(',').next() {
            is_game_valid = process_one_color_data(color_data, &mut is_game_valid);

            if set.len() == color_data.len() || !is_game_valid {
                break;
            }

            set = &set[color_data.len() + 1..];
        }

        // Process next game or finalize the current game
        if line.len() == game.len() {
            if is_game_valid {
                println!(
                    "+ Valid game sum {sum} + {game_id_num} = {:?}",
                    game_id_num + *sum
                );
                *sum += game_id_num;
            } else {
                println!("- Invalid game");
            }
            break;
        }

        line = &line[game.len() + 1..];
    }
    Ok(())
}

// Function to process a single color data
fn process_one_color_data(color_data: &str, is_game_valid: &mut bool) -> bool {
    let space_pos = color_data.rfind(' ').unwrap();
    let count = &color_data[..space_pos];
    let count: u64 = count.trim().parse().expect("Failed to parse string to u64");
    let color = &color_data[space_pos + 1..];

    // Validate the color count against the maximum allowed
    for (max_color, max_count) in MAX_ARRAY {
        if max_color == color {
            if count <= max_count {
                println!("Valid color {count} <= {max_count} for {color}");
            } else {
                println!("Invalid color {count} > {max_count} for {color}");
                *is_game_valid = false;
                break;
            }
        }
    }

    *is_game_valid
}

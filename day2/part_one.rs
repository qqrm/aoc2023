#!/usr/bin/env run-cargo-script

//
//
//

use io::ErrorKind;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

static MAX_ARRAY: [(&'static str, u64); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        // skip game word
        let line = &line[5..];
        let mut is_game_valid = true;

        let game_id = line.split(':').next().unwrap();
        dbg!(&game_id);
        let game_id_num: u64 = game_id
            .trim()
            .parse()
            .expect("Failed to parse string to u64");

        // skip ": "
        let line = &line[2..];
        // skip game_id
        let mut line = &line[game_id.len()..];

        dbg!(&line);

        while let Some(game) = line.split(';').next() {
            // dbg!(&game);

            let mut set = game;
            dbg!(&set);

            while let Some(color_data) = set.split(',').next() {
                // dbg!(&color_data);

                let space_pos = color_data.rfind(' ').unwrap();

                let count = &color_data[..space_pos];
                let count: u64 = count.trim().parse().expect("Failed to parse string to u64");

                // dbg!(&count);

                let color = &color_data[space_pos + 1..];
                // dbg!(&color);

                // validation

                for (max_color, max_count) in MAX_ARRAY {
                    if max_color == color {
                        if count <= max_count {
                            println!("Valid color {count} <= {max_count} for {color}");
                            // is_game_valid = true;
                        } else {
                            println!("Invalid color {count} > {max_count} for {color}");

                            is_game_valid = false;

                            break;
                        }
                    }
                }

                if set.len() == color_data.len() || !is_game_valid {
                    break;
                }

                set = &set[color_data.len() + 1..];
            }

            if line.len() == game.len() {
                if is_game_valid {
                    println!(
                        "+ Valid game sum {sum} + {game_id_num} = {:?}",
                        game_id_num + sum
                    );

                    sum += game_id_num;
                } else {
                    println!("- Invalid game");
                }
                break;
            }

            line = &line[game.len() + 1..];
        }

        // dbg!(&games);

        // dbg!(&line);

        // println!("Valid rund  in game {sum}");

        // return Ok(());

        // pause_for_input("Press Enter to continue...");
    }

    println!("Total sum: {}", sum);
    Ok(())
}

fn pause_for_input(message: &str) {
    let mut input = String::new();
    print!("{}", message);
    // io::stdout().flush().unwrap(); // Make sure the message is displayed before pausing
    io::stdin().read_line(&mut input).unwrap();
}

#!/usr/bin/env run-cargo-script

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SIZE: usize = 140;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;
    let mut stars_num = HashMap::new();
    let mut matrix = [[' '; SIZE]; SIZE];

    for (i, line) in io::BufReader::new(file).lines().enumerate().take(SIZE) {
        line?
            .chars()
            .enumerate()
            .take(SIZE)
            .for_each(|(j, ch)| matrix[i][j] = ch);
    }

    matrix.iter().enumerate().for_each(|(y, row)| {
        let (mut buf, mut num_buf) = (vec![], String::new());

        row.iter().enumerate().for_each(|(x, &ch)| {
            if ch.is_ascii_alphanumeric() {
                num_buf.push(ch);
                buf.push((x as i64, y as i64));
            }

            if !ch.is_ascii_alphanumeric() || x == SIZE - 1 && !buf.is_empty() {
                let contour = extract_contour(&buf);
                let number: i64 = num_buf.parse().unwrap_or(0);

                contour
                    .iter()
                    .find(|&&(cx, cy)| {
                        matrix[cy as usize][cx as usize] == '*'
                            && stars_num.entry((cx, cy)).or_insert(number) != &number
                    })
                    .map(|&(cx, cy)| sum += stars_num[&(cx, cy)] * number);

                buf.clear();
                num_buf.clear();
            }
        });
    });

    println!("Total sum: {}", sum);
    Ok(())
}

fn extract_contour(buf: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut circle = vec![];
    let mut visited = std::collections::HashSet::new();

    buf.iter()
        .flat_map(|&(x, y)| get_neighbors(x, y))
        .filter(|&(nx, ny)| {
            nx >= 0
                && ny >= 0
                && nx < SIZE as i64
                && ny < SIZE as i64
                && !buf.contains(&(nx, ny))
                && visited.insert((nx, ny))
        })
        .for_each(|coord| circle.push(coord));

    circle
}

fn get_neighbors(x: i64, y: i64) -> [(i64, i64); 8] {
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ]
}

fn print_contour(coords: &Vec<(i64, i64)>, matrix: &[[char; SIZE]; SIZE]) {
    // Find the minimum and maximum x and y coordinates
    let min_x = coords.iter().map(|&(x, _)| x).min().unwrap() as usize;
    let max_x = coords.iter().map(|&(x, _)| x).max().unwrap() as usize;
    let min_y = coords.iter().map(|&(_, y)| y).min().unwrap() as usize;
    let max_y = coords.iter().map(|&(_, y)| y).max().unwrap() as usize;

    // Iterate over the specified contour area and print the relevant part of the matrix
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            // Check if the coordinate is within the bounds of the matrix
            if i < SIZE && j < SIZE {
                print!("{}", matrix[i][j]);
            } else {
                print!(" "); // Print a space if outside the matrix bounds
            }
        }
        println!(); // New line at the end of each row
    }
    println!(); // New line at the end of each row
}

fn wait_for_enter() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

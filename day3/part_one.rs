#!/usr/bin/env run-cargo-script

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SIZE: usize = 140;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;

    let mut matrix = vec![vec![' '; SIZE]; SIZE]; // Initialize a 140x140 matrix with spaces

    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        if i >= SIZE {
            break; // Only read the first 140 lines
        }

        let line = line?;
        for (j, ch) in line.chars().enumerate().take(SIZE) {
            matrix[i][j] = ch;
        }
    }

    // print_matrix(&matrix);

    println!("Total sum: {}", sum);
    Ok(())
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &ch in row {
            print!("{}", ch);
        }
        println!(); // Print a new line at the end of each row
    }
}

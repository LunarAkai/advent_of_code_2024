pub mod day_1;

use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn advent_of_code_2024() {
    day_1::historian_hysteria();
}

// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
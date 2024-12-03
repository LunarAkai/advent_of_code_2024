pub mod day_1;
pub mod day_2;
pub mod day_3;

use std::{fs::{self, File}, io::{self, BufRead}, path::Path};

pub fn advent_of_code_2024() {
    //day_1::historian_hysteria();
    //day_2::red_nosed_reports();
    day_3::mull_it_over();
}


pub fn read_file<P>(filename: P) -> String 
    where P: AsRef<Path> {
    fs::read_to_string(filename).expect("Unable to read file")
}

// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
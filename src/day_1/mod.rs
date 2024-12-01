// Day 1: Historian Hysteria

/*
    --- PART 1 --- 
    pair up smallest number in left list with smallest number in the right list
    -> distances between those two numbers
    -> add up all the distances

    --- PART 2 ---
    how often appears a number from the left list in the right list
    -> calculate similarity score by adding up each number in the left list after
        multiplying it by the number of times that number appears in the right list
*/

use std::collections::HashMap;

use crate::read_lines;

pub fn historian_hysteria() {
    let result_p1 = historian_hysteria_part_one("src/day_1/input.txt");
    let result_p2 = historian_hysteria_part_two("src/day_1/input.txt");

    println!("PART 1 - total diff: {:?}", result_p1);
    println!("PART 2 - similarity score: {:?}", result_p2);
}

pub fn historian_hysteria_part_one(path: &str) -> u32 {
    let (left_numbers, right_numbers) = return_sets(path);
    
    left_numbers
        .iter()
        .zip(right_numbers.iter()).map(|(left, right)| (left.abs_diff(*right)))
        .sum()
}

pub fn historian_hysteria_part_two(path: &str) -> u32 {
    let (left_numbers, right_numbers) = return_sets(path);

    let mut counts = HashMap::with_capacity(left_numbers.len());
    for number in right_numbers {
        *counts.entry(number).or_insert(0) += 1;
    }

    left_numbers.iter().map(|i| i * counts.get(i).unwrap_or(&0)).sum()
}

pub fn return_sets(path: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let new_line = line.unwrap();
            let (a, b) = new_line.split_at(new_line.find("   ").unwrap());
            left.push(a.parse::<u32>().unwrap());
            right.push(b.trim().parse::<u32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    (left,right)
}



// Day 3: Mull It Over

/*

"mul[(]\d{1,3}\S\d{1,3}[)]"gm


maybe:
[^don't().*do()]\s*mul[(](\d{1,3}),(\d{1,3})[)]
*/

use regex::Regex;

use crate::read_file;

pub fn mull_it_over() {
    mull_it_over_part_one();
    mull_it_over_part_two();
}

fn mull_it_over_part_one() {
    let re = Regex::new(r"mul[(](\d{1,3}),(\d{1,3})[)]").unwrap();

    let test_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let data = read_file("src/day_3/input.txt");

    let valid_numbers: Vec<(i32, i32)> = re.captures_iter(&data).map(|caps| {
        let (_, [a, b]) = caps.extract();
        (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    }).collect();

    let mut result = 0;
    for numbers in valid_numbers {
        result += numbers.0 * numbers.1;
    }
    println!("Part 1: {:?}", result);
}

fn mull_it_over_part_two() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();

    let test_string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let mut result = 0;
    let mut enabled = true;

    for c in re.captures_iter(read_file("src/day_3/input.txt").as_str()) {
        if let Some(_) = c.get(1) {
            if enabled {
                let a: i32 = c[1].parse().unwrap();
                let b: i32 = c[2].parse().unwrap();
                result += a * b;
            }
        } else if let Some(_) = c.get(3) {
            enabled = true;
        } else if let Some(_) = c.get(4) {
            enabled = false;
        }
    }
    println!("Part 2: {:?}", result);
}
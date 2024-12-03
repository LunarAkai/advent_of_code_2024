// Day 2: Red-Nosed Reports

/*
    one report per line
    -> list of number called levels

    Safe: 
    Decrease or Increase by 1, 2 or 3 && must *all* increase or decrease

    Unsafe:
    Decrease or Increase by >= 4

    test safe:
    7 6 4 2 1
*/


#[derive(PartialEq)]
enum Direction {
    Increase,
    Decrease,
    NotDefined
}


pub fn red_nosed_reports() {
    red_nosed_reports_part_1("src/day_2/input.txt");

}

pub fn red_nosed_reports_part_1(path: &str) {
    let report = "7 6 4 2 1";

    let split: Vec<u32> = report.split_ascii_whitespace().map(|v| v.parse::<u32>().unwrap()).collect();
    for n in 1 .. split.len() {
        let current = split[n];
        let prev = split[n - 1];

        let mut direction = Direction::NotDefined;

        if current - prev > 0 && direction == Direction::NotDefined {
            direction = Direction::Increase
        } else {
            direction = Direction::Decrease
        }

        let difference = current.abs_diff(prev);


        if difference < 1 || difference > 3 {
            panic!("unsafe")
        }

    }

}

pub fn is_safe(report: &str) -> bool {
    true
}

pub fn check_value_change(value: Vec<u32>) {
    

}




use advent_of_code_2024::day_2::*;

#[test]
fn d2_is_safe() {
    assert_eq!(is_safe("7 6 4 2 1"), true);
}

#[test]
fn d2_is_unsafe() {
    assert_eq!(is_safe("1 2 7 8 9"), false);
}
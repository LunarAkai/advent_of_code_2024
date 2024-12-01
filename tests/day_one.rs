use advent_of_code_2024::day_1::*;

#[test]
fn test_d1_total_diff() {
    assert_eq!(historian_hysteria_part_one("src/day_1/test_data.txt"), 11);
}

#[test]
fn test_d1_sets() {
    let a: Vec<u32> = vec![1, 2, 3, 3, 3, 4];

    assert_eq!(return_sets("src/day_1/test_data.txt").0, a);

    let b: Vec<u32> = vec![3, 3, 3, 4, 5, 9];

    assert_eq!(return_sets("src/day_1/test_data.txt").1, b);
}

#[test] 
fn test_d1_total_sim_score() {
    assert_eq!(historian_hysteria_part_two("src/day_1/test_data.txt"), 31);
}

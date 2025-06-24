use aoc24::days;

#[test]
fn test_known_results() {
    // Update these with your actual results
    let expected_results = vec![
        ("Day 01", "Part 1: 2378066, Part 2: 18934359"),
        ("Day 02", "Part 1: 379, Part 2: 430"),
    ];
    println!("Testing known results");
    for (day, expected) in expected_results {
        match day {
            "Day 01" => assert_eq!(days::day01::solution(), expected),
            "Day 02" => assert_eq!(days::day02::solution(), expected),
            _ => panic!("Unknown day: {}", day),
        }
    }
}

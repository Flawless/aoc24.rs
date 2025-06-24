use crate::aoc_solution;
use crate::util::aoc::Solution;

aoc_solution!(2, part_1, part_2);

const MAX_DISTANCE_SQUARED: i64 = 3 * 3;
const MIN_DISTANCE_SQUARED: i64 = 1 * 1;

pub fn parse_input(input: &str) -> Result<Vec<Vec<i64>>, String> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let mut report = Vec::new();
        for part in line.split_whitespace() {
            if let Ok(num) = part.parse::<i64>() {
                report.push(num);
            } else {
                return Err("Failed to parse integers".into());
            }
        }
        reports.push(report);
    }
    Ok(reports)
}

fn distances(report: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(report.len() - 1);
    for i in 1..report.len() {
        result.push(report[i] - report[i - 1]);
    }
    result
}

fn safe_distance(distance: i64) -> bool {
    distance * distance >= MIN_DISTANCE_SQUARED && distance * distance <= MAX_DISTANCE_SQUARED
}

fn safe_report(report: &[i64]) -> bool {
    let distances = distances(report);
    for i in 0..distances.len() {
        if i != 0 && distances[i] * distances[i - 1] <= 0 {
            break;
        }

        if !safe_distance(distances[i]) {
            break;
        }
        if i == distances.len() - 1 {
            return true;
        }
    }
    false
}

fn slice_excluding_nth<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(slice.len() - 1);
    result.extend_from_slice(&slice[..n]);
    result.extend_from_slice(&slice[n + 1..]);
    result
}

fn part_1(input: &str) -> u64 {
    let reports = parse_input(input).unwrap();

    let mut acc = 0;
    for report in reports {
        if safe_report(&report) {
            acc += 1;
        }
    }
    acc
}

fn part_2(input: &str) -> u64 {
    let reports = parse_input(input).unwrap();

    let mut acc = 0;
    for report in reports {
        for i in 0..report.len() {
            let report = slice_excluding_nth(&report, i);
            if safe_report(&report) {
                acc += 1;
                break;
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 4);
    }
}

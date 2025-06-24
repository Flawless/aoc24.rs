use crate::aoc_solution;
use crate::util::aoc::Solution;
use crate::util::search;
use crate::util::sort;

aoc_solution!(1, part_1, part_2);

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(first), Ok(second)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_vec.push(first);
                second_vec.push(second);
            } else {
                return Err("Failed to parse integers".into());
            }
        } else {
            return Err("Line does not contain enough parts".into());
        }
    }
    Ok((first_vec, second_vec))
}

fn part_1(input: &str) -> i32 {
    let mut input = parse_input(input).unwrap();

    sort::qs(&mut input.0);
    sort::qs(&mut input.1);

    let mut acc = 0;
    for i in 0..input.0.len() {
        let distance = input.0[i] - input.1[i];
        acc += distance.abs();
    }
    acc
}

fn part_2(input: &str) -> i32 {
    let mut input = parse_input(input).unwrap();

    sort::qs(&mut input.0);
    sort::qs(&mut input.1);

    let mut acc = 0;
    for i in 0..input.0.len() {
        if let Some(mut index) = search::bs(&input.1, input.0[i]) {
            while index <= input.1.len() && input.1[index] == input.0[i] {
                index += 1;
                acc += input.0[i];
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 31);
    }
}

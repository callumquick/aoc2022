use aoc2022::{get_day_input, print_elapsed_time, parse_input_lines};
use std::{ops::RangeInclusive, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
struct Range(RangeInclusive<u32>);

#[derive(Debug, Clone)]
struct Assignment {
    first: Range,
    second: Range,
}

impl FromStr for Range {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, "-").map(|p| p.parse().unwrap()).collect();
        Ok(Range(RangeInclusive::new(parts[0], parts[1])))
    }
}

impl FromStr for Assignment {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, ",").map(|p| p.parse::<Range>().unwrap()).collect();
        Ok(Assignment { first: parts[0].clone(), second: parts[1].clone() })
    }
}

fn part_one(input: &[Assignment]) -> u32 {
    input.iter().filter(|assignment| -> bool {
        assignment.first.0.to_owned().into_iter().all(|i| assignment.second.0.contains(&i)) ||
            assignment.second.0.to_owned().into_iter().all(|i| assignment.first.0.contains(&i))
    }).count() as u32
}

fn part_two(input: &[Assignment]) -> u32 {
    input.iter().filter(|assignment| -> bool {
        assignment.first.0.to_owned().into_iter().any(|i| assignment.second.0.contains(&i)) ||
            assignment.second.0.to_owned().into_iter().any(|i| assignment.first.0.contains(&i))
    }).count() as u32
}

fn parse_input(input_str: &str) -> Vec<Assignment> {
    parse_input_lines(input_str)
}

fn main() {
    let input_str = get_day_input("04");
    let input = parse_input(&input_str);
    println!("Day 04:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_04() {
        let input_str: String = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        );

        let input = parse_input(&input_str);

        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 4);
    }
}

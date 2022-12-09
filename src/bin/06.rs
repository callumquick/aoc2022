use std::collections::HashSet;

use aoc2022::{get_day_input, print_elapsed_time};

fn find_marker(input: &Input, window_size: usize) -> usize {
    input
        .windows(window_size)
        .enumerate()
        .filter(|(_, w)| HashSet::<char>::from_iter(w.iter().cloned()).len() == window_size)
        .next()
        .unwrap()
        .0
        + window_size
}

fn part_one(input: &Input) -> usize {
    find_marker(input, 4)
}

fn part_two(input: &Input) -> usize {
    find_marker(input, 14)
}

type Input = Vec<char>;
fn parse_input(input_str: &str) -> Input {
    input_str.chars().collect()
}

fn main() {
    let input_str = get_day_input("06");
    let input = parse_input(&input_str);
    println!("Day 06:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_06() {
        let input_str: String = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 7);
        assert_eq!(part_two(&input), 19);
    }

    #[test]
    fn test_given_example_others_06() {
        let input_str: String = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let input = parse_input(&input_str);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), 23);

        let input_str: String = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let input = parse_input(&input_str);
        assert_eq!(part_one(&input), 6);
        assert_eq!(part_two(&input), 23);

        let input_str: String = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let input = parse_input(&input_str);
        assert_eq!(part_one(&input), 10);
        assert_eq!(part_two(&input), 29);

        let input_str: String = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let input = parse_input(&input_str);
        assert_eq!(part_one(&input), 11);
        assert_eq!(part_two(&input), 26);
    }
}

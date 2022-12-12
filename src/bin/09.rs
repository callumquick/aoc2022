use std::{cmp::max, collections::HashSet, num::ParseIntError, str::FromStr};

use aoc2022::{get_day_input, print_elapsed_time};

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, amount_str) = s.split_once(" ").expect("Malformed instruction");
        let amount = amount_str.parse().expect("Malformed instruction");
        let direction = match direction_str {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Malformed instruction"),
        };
        Ok(Instruction { direction, amount })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i64, i64);

impl Point {
    fn move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Point(self.0, self.1 + 1),
            Direction::Down => Point(self.0, self.1 - 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
        }
    }

    fn diff(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }

    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    fn pull_point(&self, other: &Self) -> Self {
        let diff = &self.diff(other);

        if diff.0.abs() < 2 && diff.1.abs() < 2 {
            // There is no pulling force
            return *other;
        }

        // Always moves a unit diagonally or in the single diff direction
        let pull_step = Point(diff.0.signum(), diff.1.signum());
        other.add(&pull_step)
    }
}

fn part_one(input: &Input) -> usize {
    // The head can only be max one away from the tail, so can just track it in
    // a 3x3 grid where the tail is always in the middle square, and if the head
    // goes of one side can just add a new tail position to the tracking
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    let mut tail_positions = HashSet::from([tail]);

    for instruction in input {
        for _ in 0..instruction.amount {
            head = head.move_in_direction(&instruction.direction);
            tail = head.pull_point(&tail);
            tail_positions.insert(tail);
        }
    }

    tail_positions.len()
}

fn part_two(input: &Input) -> usize {
    let mut head = Point(0, 0);
    let mut tails = [Point(0, 0); 9];
    let mut tail_9_positions = HashSet::from([tails[tails.len() - 1]]);

    for instruction in input {
        for _ in 0..instruction.amount {
            head = head.move_in_direction(&instruction.direction);

            let mut new_tails = [Point(0, 0); 9];
            let mut prev_tail = head;
            for (i, next_tail) in tails.iter().enumerate() {
                let new_tail = prev_tail.pull_point(&next_tail);
                new_tails[i] = new_tail;
                prev_tail = new_tail;
            }
            tail_9_positions.insert(new_tails[new_tails.len() -1]);
            tails = new_tails;
        }
    }

    tail_9_positions.len()
}

type Input = Vec<Instruction>;
fn parse_input(input_str: &str) -> Input {
    input_str.lines().map(|l| l.parse().unwrap()).collect()
}

fn main() {
    let input_str = get_day_input("09");
    let input = parse_input(&input_str);
    println!("Day 09:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_09() {
        let input_str: String = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn test_second_example_09() {
        let input_str: String = String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );

        let input = parse_input(&input_str);
        assert_eq!(part_two(&input), 36);
    }
}

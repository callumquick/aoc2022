use aoc2022::{get_day_input, print_elapsed_time};
use std::char::ParseCharError;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Action {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => panic!("Didn't match an action"),
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Outcome::Lose),
            "B" | "Y" => Ok(Outcome::Draw),
            "C" | "Z" => Ok(Outcome::Win),
            _ => panic!("Didn't match an action"),
        }
    }
}

impl Action {
    fn play(&self, other: &Action) -> Outcome {
        match (&self, other) {
            (Action::Rock, Action::Scissors) => Outcome::Win,
            (Action::Rock, Action::Paper) => Outcome::Lose,
            (Action::Rock, Action::Rock) => Outcome::Draw,
            (Action::Paper, Action::Scissors) => Outcome::Lose,
            (Action::Paper, Action::Paper) => Outcome::Draw,
            (Action::Paper, Action::Rock) => Outcome::Win,
            (Action::Scissors, Action::Scissors) => Outcome::Draw,
            (Action::Scissors, Action::Paper) => Outcome::Win,
            (Action::Scissors, Action::Rock) => Outcome::Lose,
        }
    }
}

impl Outcome {
    fn needed_action(&self, other: &Action) -> Action {
        match (&self, other) {
            (Outcome::Win, Action::Rock) => Action::Paper,
            (Outcome::Win, Action::Paper) => Action::Scissors,
            (Outcome::Win, Action::Scissors) => Action::Rock,
            (Outcome::Lose, Action::Rock) => Action::Scissors,
            (Outcome::Lose, Action::Paper) => Action::Rock,
            (Outcome::Lose, Action::Scissors) => Action::Paper,
            (Outcome::Draw, other) => *other,
        }
    }
}

fn part_one(input: &[(Action, Action)]) -> u32 {
    input.iter().fold(0, |acc, (opponent, me)| -> u32 {
        acc + me.play(opponent) as u32 + *me as u32
    })
}

fn part_two(input: &[(Action, Outcome)]) -> u32 {
    input.iter().fold(0, |acc, (opponent, outcome)| {
        acc + outcome.needed_action(opponent) as u32 + *outcome as u32
    })
}

fn parse_input_p1(input_str: &str) -> Vec<(Action, Action)> {
    input_str
        .lines()
        .map(|line| {
            let actions: Vec<Action> = line
                .trim()
                .split_whitespace()
                .map(|action| action.parse())
                .collect::<Result<_, _>>()
                .unwrap();
            (actions[0], actions[1])
        })
        .collect()
}

fn parse_input_p2(input_str: &str) -> Vec<(Action, Outcome)> {
    input_str
        .lines()
        .map(|line| {
            let inputs: Vec<&str> = line.trim().split_whitespace().collect();
            (inputs[0].parse().unwrap(), inputs[1].parse().unwrap())
        })
        .collect()
}

fn main() {
    let input_str = get_day_input("02");
    let input_p1 = parse_input_p1(&input_str);
    let input_p2 = parse_input_p2(&input_str);
    println!("Day 02:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input_p1)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input_p2)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_02() {
        let input_str: String = String::from(
            "A Y
B X
C Z",
        );

        let input_p1 = parse_input_p1(&input_str);
        let input_p2 = parse_input_p2(&input_str);

        assert_eq!(part_one(&input_p1), 15);
        assert_eq!(part_two(&input_p2), 12);
    }
}

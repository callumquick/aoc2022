use aoc2022::{get_day_input, parse_input_lines, print_elapsed_time};
use std::{char::ParseCharError, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [num, from, to] = s
            .split(char::is_whitespace)
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>()[..]
        {
            Ok(Instruction { num, from, to })
        } else {
            panic!("Malformed input")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Crate {
    letter: char,
}

impl FromStr for Crate {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 3);
        let letter: char = s.trim_matches(|c| c == '[' || c == ']').parse()?;
        Ok(Crate { letter })
    }
}

#[derive(Debug, Clone)]
struct CrateSpecification {
    piles: Vec<Vec<Crate>>,
}

impl FromStr for CrateSpecification {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let num_cols: usize = lines
            .iter()
            .last()
            .unwrap()
            .trim()
            .rsplit_once(char::is_whitespace)
            .unwrap()
            .1
            .parse()?;

        let mut piles: Vec<Vec<Crate>> = Vec::new();

        for i in 0..num_cols {
            let mut col: Vec<Crate> = Vec::new();
            for line in lines.iter().rev().skip(1) {
                let crate_str = line.chars().skip(i * 4).take(3).collect::<String>();
                if !crate_str.trim().is_empty() {
                    col.push(crate_str.parse().unwrap());
                }
            }
            piles.push(col);
        }

        Ok(CrateSpecification { piles })
    }
}

#[derive(Debug)]
struct Procedure {
    initial_state: CrateSpecification,
    instructions: Vec<Instruction>,
}

impl Procedure {
    fn implement_9000(&self) -> CrateSpecification {
        let mut state = self.initial_state.clone();

        for instruction in &self.instructions {
            let from_pile = &mut state.piles[instruction.from - 1];
            let new_len = from_pile.len() - instruction.num;
            let move_items: Vec<_> = from_pile.drain(new_len..).rev().collect();

            let to_pile = &mut state.piles[instruction.to - 1];
            to_pile.extend(move_items);
        }

        state
    }

    fn implement_9001(&self) -> CrateSpecification {
        let mut state = self.initial_state.clone();

        for instruction in &self.instructions {
            let from_pile = &mut state.piles[instruction.from - 1];
            let new_len = from_pile.len() - instruction.num;
            let move_items: Vec<_> = from_pile.drain(new_len..).collect();

            let to_pile = &mut state.piles[instruction.to - 1];
            to_pile.extend(move_items);
        }

        state
    }
}

fn part_one(input: &Procedure) -> String {
    let new_state = input.implement_9000();

    new_state
        .piles
        .iter()
        .map(|v| v.last().unwrap().letter)
        .collect()
}

fn part_two(input: &Procedure) -> String {
    let new_state = input.implement_9001();

    new_state
        .piles
        .iter()
        .map(|v| v.last().unwrap().letter)
        .collect()
}

fn parse_input(input_str: &str) -> Procedure {
    if let [first_part, second_part] = input_str.splitn(2, "\n\n").collect::<Vec<_>>()[..] {
        Procedure {
            initial_state: first_part.parse().expect("Malformed input"),
            instructions: parse_input_lines(second_part),
        }
    } else {
        panic!("Input is malformed")
    }
}

fn main() {
    let input_str = get_day_input("05");
    let input = parse_input(&input_str);
    println!("Day 05:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_05() {
        let input_str: String = String::from(
            "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), "CMZ");
        assert_eq!(part_two(&input), "MCD");
    }
}
